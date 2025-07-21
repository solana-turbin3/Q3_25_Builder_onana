use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::{
    error::CustomError,
    state::*,
};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + StakeConfig::INIT_SPACE,
        seeds = [b"config", admin.key().as_ref()],
        bump
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        init,
        payer = admin,
        seeds = [b"config", admin.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub reward_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<InitializeConfig>,
    points_per_stake: u8,
    max_unstake_period: u8,
    freeze_period: u32,
) -> Result<()> {
    ctx.accounts.config.set_inner(StakeConfig {
        points_per_stake,
        max_unstake_period,
        freeze_period,
        reward_bump: ctx.bumps.reward_mint,
        bump: ctx.bumps.config,
        last_update: Clock::get()?.unix_timestamp,
    });
    Ok(())
}
