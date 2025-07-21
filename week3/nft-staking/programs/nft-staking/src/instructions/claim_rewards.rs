use crate::{error::CustomError, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, MintTo, Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        seeds = [b"config", user.key().as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        mut,
        seeds = [b"rewards", config.key().as_ref()],
        bump = config.reward_bump
    )]
    pub reward_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = user
    )]
    pub user_reward_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Claim>) -> Result<()> {
    let amount = ctx.accounts.user_account.points;
    require!(amount > 0, CustomError::NoRewardsToClaim);
    let config_key = ctx.accounts.config.key();
    let seeds: &[&[u8]] = &[
        b"config",
        config_key.as_ref(),
        &[ctx.accounts.config.bump],
    ];
    let signer = &[seeds];
    let cpi_accounts = MintTo {
        mint: ctx.accounts.reward_mint.to_account_info(),
        to: ctx.accounts.user_reward_ata.to_account_info(),
        authority: ctx.accounts.config.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer);
    mint_to(cpi_ctx, amount.into())?;
    ctx.accounts.user_account.points = 0;
    Ok(())
}