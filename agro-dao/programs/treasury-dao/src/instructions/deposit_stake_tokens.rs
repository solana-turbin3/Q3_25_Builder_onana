use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo};
use crate::state::*;
use crate::constants::*;
use crate::error::TreasuryError;

#[derive(Accounts)]
pub struct DepositStakeTokens<'info> {
    #[account(
        seeds = [TREASURY_CONFIG_SEED],
        bump = treasury_config.bump,
        constraint = !treasury_config.emergency_pause @ TreasuryError::TreasuryPaused
    )]
    pub treasury_config: Account<'info, TreasuryConfig>,

    #[account(
        mut,
        seeds = [TOKEN_VAULT_SEED, depositor_token_account.mint.as_ref()],
        bump = token_vault.bump,
    )]
    pub token_vault: Account<'info, TokenVault>,

    #[account(
        mut,
        seeds = [b"vault_ata", depositor_token_account.mint.as_ref()],
        bump,
        token::mint = depositor_token_account.mint,
        token::authority = token_vault,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [FEE_VAULT_SEED, depositor_token_account.mint.as_ref()],
        bump,
        token::mint = depositor_token_account.mint,
        token::authority = token_vault,
    )]
    pub fee_vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        seeds = [STAKE_ACCOUNT_SEED, depositor.key().as_ref()],
        bump,
        payer = depositor,
        space = 8 + StakeAccount::INIT_SPACE
    )]
    pub stake_account: Account<'info, StakeAccount>,

    // Research DAO integration - verify researcher is verified
    #[account(
        seeds = [b"researcher", depositor.key().as_ref()],
        bump,
        seeds::program = research_dao_program.key(),
        constraint = researcher_profile.is_verified @ TreasuryError::ResearcherNotVerified
    )]
    pub researcher_profile: Account<'info, ResearcherProfile>,

    #[account(
        mut,
        seeds = [AGRO_MINT_SEED],
        bump,
    )]
    pub agro_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = agro_mint,
        associated_token::authority = depositor,
    )]
    pub depositor_agro_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = depositor_token_account.mint,
        token::authority = depositor,
    )]
    pub depositor_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub depositor: Signer<'info>,

    /// CHECK: This is the research DAO program for CPI
    pub research_dao_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> DepositStakeTokens<'info> {
    pub fn deposit_stake_tokens(&mut self, amount: u64, bumps: &DepositStakeTokensBumps) -> Result<()> {
        // Validate inputs
        require!(amount > 0, TreasuryError::InvalidAmount);
        
        // Check if token is supported
        let token_mint = self.depositor_token_account.mint;
        require!(
            self.treasury_config.supported_tokens.contains(&token_mint),
            TreasuryError::UnsupportedToken
        );

        // Calculate fees and net amount
        let fee_amount = amount
            .checked_mul(self.treasury_config.fee_rate_bps as u64)
            .ok_or(TreasuryError::ArithmeticOverflow)?
            .checked_div(10000)
            .ok_or(TreasuryError::ArithmeticOverflow)?;
        
        let net_amount = amount
            .checked_sub(fee_amount)
            .ok_or(TreasuryError::ArithmeticUnderflow)?;

        let clock = Clock::get()?;

        // Transfer tokens to vault
        let transfer_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.depositor_token_account.to_account_info(),
                to: self.vault_token_account.to_account_info(),
                authority: self.depositor.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, amount)?;

        // Transfer fees to fee vault
        if fee_amount > 0 {
            let fee_transfer_ctx = CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.vault_token_account.to_account_info(),
                    to: self.fee_vault.to_account_info(),
                    authority: self.token_vault.to_account_info(),
                },
            );

            let vault_seeds = &[
                TOKEN_VAULT_SEED,
                token_mint.as_ref(),
                &[self.token_vault.bump],
            ];
            let vault_signer = &[&vault_seeds[..]];

            token::transfer(
                fee_transfer_ctx.with_signer(vault_signer),
                fee_amount,
            )?;
        }

        // Mint AGRO tokens (1:1 ratio with net amount)
        let mint_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.agro_mint.to_account_info(),
                to: self.depositor_agro_account.to_account_info(),
                authority: self.agro_mint.to_account_info(),
            },
        );

        let agro_mint_seeds = &[
            AGRO_MINT_SEED,
            &[bumps.agro_mint],
        ];
        let agro_mint_signer = &[&agro_mint_seeds[..]];

        token::mint_to(mint_ctx.with_signer(agro_mint_signer), net_amount)?;

        // Update token vault
        self.token_vault.total_deposits = self.token_vault.total_deposits
            .checked_add(amount)
            .ok_or(TreasuryError::ArithmeticOverflow)?;
        
        self.token_vault.available_balance = self.token_vault.available_balance
            .checked_add(net_amount)
            .ok_or(TreasuryError::ArithmeticOverflow)?;

        // Initialize or update stake account
        if self.stake_account.owner == Pubkey::default() {
            self.stake_account.set_inner(StakeAccount {
                bump: bumps.stake_account,
                owner: self.depositor.key(),
                total_agro_minted: net_amount,
                deposits: vec![TokenDeposit {
                    token_mint,
                    amount,
                    agro_minted: net_amount,
                    timestamp: clock.unix_timestamp,
                }],
                last_activity: clock.unix_timestamp,
                created_at: clock.unix_timestamp,
            });
        } else {
            // Update existing stake account
            self.stake_account.total_agro_minted = self.stake_account.total_agro_minted
                .checked_add(net_amount)
                .ok_or(TreasuryError::ArithmeticOverflow)?;
            
            self.stake_account.last_activity = clock.unix_timestamp;
            
            // Add new deposit or update existing one
            if let Some(existing_deposit) = self.stake_account.deposits
                .iter_mut()
                .find(|d| d.token_mint == token_mint) {
                existing_deposit.amount = existing_deposit.amount
                    .checked_add(amount)
                    .ok_or(TreasuryError::ArithmeticOverflow)?;
                existing_deposit.agro_minted = existing_deposit.agro_minted
                    .checked_add(net_amount)
                    .ok_or(TreasuryError::ArithmeticOverflow)?;
                existing_deposit.timestamp = clock.unix_timestamp;
            } else {
                self.stake_account.deposits.push(TokenDeposit {
                    token_mint,
                    amount,
                    agro_minted: net_amount,
                    timestamp: clock.unix_timestamp,
                });
            }
        }

        emit!(TokensDeposited {
            depositor: self.depositor.key(),
            token_mint,
            amount,
            fee_amount,
            net_amount,
            agro_minted: net_amount,
            researcher_profile: self.researcher_profile.key(),
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct TokensDeposited {
    pub depositor: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub fee_amount: u64,
    pub net_amount: u64,
    pub agro_minted: u64,
    pub researcher_profile: Pubkey,
    pub timestamp: i64,
}
