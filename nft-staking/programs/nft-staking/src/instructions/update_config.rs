use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config", admin.key().as_ref()],
        bump = config.bump,
        // has_one = admin // Optional: if you store admin pubkey in config
    )]
    pub config: Account<'info, StakeConfig>,
}

pub fn handler(
    ctx: Context<UpdateConfig>,
    points_per_stake: u8,
    max_unstake_period: u8,
    freeze_period: u32,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.points_per_stake = points_per_stake;
    config.max_unstake_period = max_unstake_period;
    config.freeze_period = freeze_period;
    config.last_update = Clock::get()?.unix_timestamp;
    Ok(())
}
