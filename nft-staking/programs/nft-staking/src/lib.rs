pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("HCnF2pS8AtHit3xxVHF3GqojnvYx6vihpboZgMqfoPSZ");

use instructions::initialize_config;
use instructions::initialize_user_account;
use instructions::stake;
use instructions::unstake;
use instructions::claim_rewards;
use instructions::update_config;

#[program]
pub mod nft_staking {
    use super::*;

    pub fn init_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_unstake_period: u8,
        freeze_period: u32,
    ) -> Result<()> {
        initialize_config::handler(ctx, points_per_stake, max_unstake_period, freeze_period)
    }

    pub fn init_user_account(
        ctx: Context<InitializeUserAccount>,
    ) -> Result<()> {
        initialize_user_account::handler(ctx)
    }

    pub fn stake(
        ctx: Context<Stake>,
    ) -> Result<()> {
        stake::handler(ctx)
    }

    pub fn unstake(
        ctx: Context<Unstake>,
    ) -> Result<()> {
        unstake::handler(ctx)
    }

    pub fn claim(
        ctx: Context<Claim>,
    ) -> Result<()> {
        claim_rewards::handler(ctx)
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        points_per_stake: u8,
        max_unstake_period: u8,
        freeze_period: u32,
    ) -> Result<()> {
        update_config::handler(ctx, points_per_stake, max_unstake_period, freeze_period)
    }
}
