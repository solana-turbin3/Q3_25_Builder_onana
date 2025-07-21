use crate::{error::CustomError, state::{UserAccount, StakeConfig, StakeAccount}};

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, transfer, Mint, TokenAccount, Transfer}
};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        mut,
        seeds = [b"config", user.key().as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,
    pub nft_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub user_nft_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = config,
    )]
    pub vault_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        close = user,
        seeds = [b"stake", user.key().as_ref(), nft_mint.key().as_ref()],
        bump = stake_account.bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<Unstake>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    require!(
        now - ctx.accounts.stake_account.staked_at >= ctx.accounts.config.freeze_period as i64,
        CustomError::NotFrozen
    );
    require!(
        ctx.accounts.user_account.amount_staked > 0,
        CustomError::NothingToUnstake
    );
    ctx.accounts.user_account.amount_staked = ctx.accounts.user_account.amount_staked
        .checked_sub(1).ok_or(CustomError::Underflow)?;
    let user_key = ctx.accounts.user.key();
    let seeds: &[&[u8]] = &[
        b"config",
        user_key.as_ref(),
        &[ctx.accounts.config.bump],
    ];
    let signer: &[&[&[u8]]] = &[seeds];
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_ata.to_account_info(),
        to: ctx.accounts.user_nft_ata.to_account_info(),
        authority: ctx.accounts.config.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer);
    transfer(cpi_ctx, 1)?;
    Ok(())
}
