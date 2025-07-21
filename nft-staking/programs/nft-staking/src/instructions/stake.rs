use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, Token, TokenAccount, Transfer, transfer}, associated_token::AssociatedToken};

use crate::{
    error::CustomError,
    state::*,
};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut,
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
        init_if_needed,
        payer = user,
        seeds = [b"vault", nft_mint.key().as_ref()],
        bump,
        token::mint = nft_mint,
        token::authority = config,
    )]
    pub  vault_ata: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = user,
        seeds = [b"stake", user.key().as_ref(), nft_mint.key().as_ref()],
        bump,
        space = 8 + StakeAccount::INIT_SPACE,
    )]
    pub stake_account: Account<'info, StakeAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<Stake>) -> Result<()> {
    let clock = Clock::get()?;
    ctx.accounts.stake_account.set_inner(StakeAccount {
        owner: ctx.accounts.user.key(),
        nft_mint: ctx.accounts.nft_mint.key(),
        staked_at: clock.unix_timestamp,
        bump: ctx.bumps.stake_account,
    });
    ctx.accounts.user_account.amount_staked = ctx.accounts.user_account.amount_staked.saturating_add(1);
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_nft_ata.to_account_info(),
        to: ctx.accounts.vault_ata.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    transfer(cpi_ctx, 1)?;
    Ok(())
}