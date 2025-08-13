use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
#[derive(InitSpace)]
pub struct ReputationConfig {
    pub bump: u8,
    pub authority: Pubkey,
    pub tier_threshold_bronze: u64,
    pub tier_threshold_silver: u64,
    pub tier_threshold_gold: u64,
    pub tier_threshold_platinum: u64,
    pub tier_threshold_diamond: u64,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[account]
#[derive(InitSpace)]
pub struct UserReputation {
    pub bump: u8,
    pub user: Pubkey,
    pub reputation_score: i64,
    pub events_count: u64,
    pub last_event_ts: i64,
    pub created_at: i64,
    pub tier: ReputationTier,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, InitSpace)]
pub enum ReputationTier {
    None,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

impl ReputationTier {
    pub fn from_score(score: i64, config: &ReputationConfig) -> Self {
        if score < 0 {
            return Self::None;
        }
        
        let score_u64 = score as u64;
        
        if score_u64 >= config.tier_threshold_diamond {
            Self::Diamond
        } else if score_u64 >= config.tier_threshold_platinum {
            Self::Platinum
        } else if score_u64 >= config.tier_threshold_gold {
            Self::Gold
        } else if score_u64 >= config.tier_threshold_silver {
            Self::Silver
        } else if score_u64 >= config.tier_threshold_bronze {
            Self::Bronze
        } else {
            Self::None
        }
    }
}

impl ReputationConfig {
    pub const SEED: &'static [u8] = REPUTATION_CONFIG_SEED;
}

impl UserReputation {
    pub const SEED: &'static [u8] = USER_REPUTATION_SEED;
    
    pub fn update_tier(&mut self, config: &ReputationConfig) {
        self.tier = ReputationTier::from_score(self.reputation_score, config);
    }
    
    pub fn can_decrease_reputation(&self, amount: i64) -> bool {
        let new_score = self.reputation_score.saturating_sub(amount.abs());
        new_score >= MIN_REPUTATION_SCORE
    }
    
    pub fn can_increase_reputation(&self, amount: i64) -> bool {
        let new_score = self.reputation_score.saturating_add(amount.abs());
        new_score <= MAX_REPUTATION_SCORE
    }
}
