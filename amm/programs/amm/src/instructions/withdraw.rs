use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Transfer, transfer, Mint, Token, TokenAccount, MintTo, mint_to}};
use constant_product_curve::ConstantProduct;

use crate::{state::Pool, error::AmmError};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
     #[account(
        mut,
        seeds = [b"lp", pool.key().as_ref()],
        bump = pool.lp_bump
    )]
    pub mint_lp: Account<'info, Mint>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"pool", pool.seed.to_le_bytes().as_ref()],
        bump = pool.pool_bump,
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = pool
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = pool
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub user_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user
    )]
    pub user_y: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        close = user,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
    )]
    pub user_lp: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        require!(self.pool.locked == false, AmmError::PoolLocked);
        require!(amount != 0, AmmError::InvalidAmount);

        let (x, y) = match self.mint_lp.supply == 0 && self.vault_x.amount == 0 && self.vault_y.amount == 0 {
            true => (self.user_x.amount, self.user_y.amount),
            false => {
                let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
                    self.vault_x.amount, 
                    self.vault_y.amount, 
                    self.mint_lp.supply, 
                    amount,
                    self.pool.fee, 
                    None, 
                ).map_err(AmmError::from)?;
                (amounts.x, amounts.y)
            }
        };

        self.withdraw_tokens(true, x)?;
        self.withdraw_tokens(false, y)?;
        self.burn_lp_tokens(amount)
    }
    
    pub fn withdraw_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.vault_x.to_account_info(),
                self.user_x.to_account_info(),
                self.mint_x.to_account_info(),
                self.mint_x.decimals,
            ),
            false => (
                self.vault_y.to_account_info(),
                self.user_y.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals,
            ),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = anchor_spl::token::TransferChecked {
            from,
            to,
            mint,
            authority: self.pool.to_account_info(),
        };

        let seeds = &[
            &b"pool"[..],
            &self.pool.seed.to_le_bytes(),
            &[self.pool.pool_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        anchor_spl::token::transfer_checked(ctx, amount, decimals)
    }

    pub fn burn_lp_tokens(&self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = anchor_spl::token::Burn {
            mint: self.mint_lp.to_account_info(),
            to: self.user_lp.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let ctx = CpiContext::new(cpi_program, cpi_accounts);
        anchor_spl::token::burn(ctx, amount)
    }
}