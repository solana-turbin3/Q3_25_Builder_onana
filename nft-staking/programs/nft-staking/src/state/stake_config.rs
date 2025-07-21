use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeConfig {
    pub points_per_stake: u8,
    pub max_unstake_period: u8,
    pub freeze_period: u32,
    pub reward_bump: u8,
    pub bump: u8,
    pub last_update: i64,
}