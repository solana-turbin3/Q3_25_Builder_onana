use anchor_lang::prelude::*;
use crate::state::{ReputationConfig, UserReputation, ReputationTier};
use crate::constants::*;
use crate::error::*;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct GetReputation<'info> {
    #[account(
        seeds = [REPUTATION_CONFIG_SEED],
        bump = reputation_config.bump
    )]
    pub reputation_config: Account<'info, ReputationConfig>,
    
    #[account(
        seeds = [USER_REPUTATION_SEED, user.as_ref()],
        bump = user_reputation.bump
    )]
    pub user_reputation: Account<'info, UserReputation>,
}

impl<'info> GetReputation<'info> {
    pub fn get_reputation(&self, user: Pubkey) -> Result<ReputationData> {
        require!(
            self.reputation_config.is_active,
            ReputationError::SystemNotActive
        );
        
        require!(
            self.user_reputation.user == user,
            ReputationError::UserReputationNotFound
        );
        
        Ok(ReputationData {
            user,
            reputation_score: self.user_reputation.reputation_score,
            tier: self.user_reputation.tier,
            events_count: self.user_reputation.events_count,
            last_event_ts: self.user_reputation.last_event_ts,
            created_at: self.user_reputation.created_at,
        })
    }
    
    pub fn get_tier_info(&self) -> Result<TierInfo> {
        require!(
            self.reputation_config.is_active,
            ReputationError::SystemNotActive
        );
        
        Ok(TierInfo {
            bronze_threshold: self.reputation_config.tier_threshold_bronze,
            silver_threshold: self.reputation_config.tier_threshold_silver,
            gold_threshold: self.reputation_config.tier_threshold_gold,
            platinum_threshold: self.reputation_config.tier_threshold_platinum,
            diamond_threshold: self.reputation_config.tier_threshold_diamond,
        })
    }
    
    pub fn check_tier_eligibility(&self, user: Pubkey, target_tier: ReputationTier) -> Result<bool> {
        require!(
            self.user_reputation.user == user,
            ReputationError::UserReputationNotFound
        );
        
        let current_score = self.user_reputation.reputation_score;
        if current_score < 0 {
            return Ok(false);
        }
        
        let score_u64 = current_score as u64;
        let is_eligible = match target_tier {
            ReputationTier::None => true,
            ReputationTier::Bronze => score_u64 >= self.reputation_config.tier_threshold_bronze,
            ReputationTier::Silver => score_u64 >= self.reputation_config.tier_threshold_silver,
            ReputationTier::Gold => score_u64 >= self.reputation_config.tier_threshold_gold,
            ReputationTier::Platinum => score_u64 >= self.reputation_config.tier_threshold_platinum,
            ReputationTier::Diamond => score_u64 >= self.reputation_config.tier_threshold_diamond,
        };
        
        Ok(is_eligible)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ReputationData {
    pub user: Pubkey,
    pub reputation_score: i64,
    pub tier: ReputationTier,
    pub events_count: u64,
    pub last_event_ts: i64,
    pub created_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TierInfo {
    pub bronze_threshold: u64,
    pub silver_threshold: u64,
    pub gold_threshold: u64,
    pub platinum_threshold: u64,
    pub diamond_threshold: u64,
}
