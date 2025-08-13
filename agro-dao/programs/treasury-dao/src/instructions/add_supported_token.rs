use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::*;
use crate::constants::*;
use crate::error::TreasuryError;

#[derive(Accounts)]
#[instruction(token_mint: Pubkey)]
pub struct AddSupportedToken<'info> {
    #[account(
        mut,
        seeds = [TREASURY_CONFIG_SEED],
        bump = treasury_config.bump,
        has_one = authority @ TreasuryError::Unauthorized,
        constraint = !treasury_config.emergency_pause @ TreasuryError::TreasuryPaused
    )]
    pub treasury_config: Account<'info, TreasuryConfig>,

    #[account(
        init,
        seeds = [TOKEN_VAULT_SEED, token_mint.as_ref()],
        bump,
        payer = authority,
        space = 8 + TokenVault::INIT_SPACE
    )]
    pub token_vault: Account<'info, TokenVault>,

    #[account(
        init,
        seeds = [FEE_VAULT_SEED, token_mint.as_ref()],
        bump,
        payer = authority,
        token::mint = token_mint_account,
        token::authority = token_vault,
    )]
    pub fee_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        seeds = [b"vault_ata", token_mint.as_ref()],
        bump,
        payer = authority,
        token::mint = token_mint_account,
        token::authority = token_vault,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub token_mint_account: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> AddSupportedToken<'info> {
    pub fn add_supported_token(&mut self, token_mint: Pubkey, bump: u8) -> Result<()> {
        // Check if we've reached the maximum number of supported tokens
        require!(
            self.treasury_config.supported_tokens.len() < MAX_SUPPORTED_TOKENS,
            TreasuryError::MaxSupportedTokensReached
        );

        // Check if token is already supported
        require!(
            !self.treasury_config.supported_tokens.contains(&token_mint),
            TreasuryError::UnsupportedToken
        );

        let clock = Clock::get()?;

        // Initialize token vault
        self.token_vault.set_inner(TokenVault {
            bump,
            token_mint,
            vault_authority: self.token_vault.key(),
            total_deposits: 0,
            available_balance: 0,
            allocated_to_proposals: 0,
            reserved_amount: 0,
            yield_positions: 0,
            created_at: clock.unix_timestamp,
        });

        // Add token to supported list
        self.treasury_config.supported_tokens.push(token_mint);

        emit!(SupportedTokenAdded {
            token_mint,
            token_vault: self.token_vault.key(),
            fee_vault: self.fee_vault.key(),
            vault_token_account: self.vault_token_account.key(),
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct SupportedTokenAdded {
    pub token_mint: Pubkey,
    pub token_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub vault_token_account: Pubkey,
    pub timestamp: i64,
}