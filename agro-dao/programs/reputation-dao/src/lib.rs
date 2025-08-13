use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;
pub use state::*;
pub use constants::*;
pub use error::*;
use state::ReputationTier;

declare_id!("HtzmTdZL8j5VSSDMSPYpwvHZLNCbP2b27KNxHzHi52Bw");

#[program]
pub mod reputation_dao {
    use super::*;

    /// Initialize the reputation system configuration
    pub fn initialize_reputation_config(
        ctx: Context<InitializeReputationConfig>,
        tier_threshold_bronze: Option<u64>,
        tier_threshold_silver: Option<u64>,
        tier_threshold_gold: Option<u64>,
        tier_threshold_platinum: Option<u64>,
        tier_threshold_diamond: Option<u64>,
    ) -> Result<()> {
        ctx.accounts.initialize_reputation_config(
            ctx.bumps.reputation_config,
            tier_threshold_bronze,
            tier_threshold_silver,
            tier_threshold_gold,
            tier_threshold_platinum,
            tier_threshold_diamond,
        )
    }

    /// Initialize a user's reputation account
    pub fn initialize_user_reputation(ctx: Context<InitializeUserReputation>) -> Result<()> {
        ctx.accounts.initialize_user_reputation(ctx.bumps.user_reputation)
    }

    /// Update a user's reputation score (called by other programs via CPI)
    pub fn update_reputation(
        ctx: Context<UpdateReputation>,
        user: Pubkey,
        event_type: ReputationEvent,
        custom_amount: Option<i64>,
    ) -> Result<()> {
        ctx.accounts.update_reputation(user, event_type, custom_amount)
    }

    /// Decrease reputation when a researcher fails to meet obligations
    pub fn decrease_reputation_on_failure(
        ctx: Context<DecreaseReputationOnFailure>,
        user: Pubkey,
        failure_type: FailureType,
        custom_penalty: Option<i64>,
    ) -> Result<()> {
        ctx.accounts.decrease_reputation_on_failure(user, failure_type, custom_penalty)
    }

    /// Get a user's reputation data (read-only)
    pub fn get_reputation(
        ctx: Context<GetReputation>,
        user: Pubkey,
    ) -> Result<ReputationData> {
        ctx.accounts.get_reputation(user)
    }

    /// Get tier threshold information
    pub fn get_tier_info(ctx: Context<GetReputation>) -> Result<TierInfo> {
        ctx.accounts.get_tier_info()
    }

    /// Check if a user is eligible for a specific tier
    pub fn check_tier_eligibility(
        ctx: Context<GetReputation>,
        user: Pubkey,
        target_tier: ReputationTier,
    ) -> Result<bool> {
        ctx.accounts.check_tier_eligibility(user, target_tier)
    }
}
