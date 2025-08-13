use anchor_lang::prelude::*;

/// Reputation seeds
#[constant]
pub const REPUTATION_CONFIG_SEED: &[u8] = b"reputation_config";

#[constant] 
pub const USER_REPUTATION_SEED: &[u8] = b"user_reputation";

/// Reputation score bounds
pub const MIN_REPUTATION_SCORE: i64 = -1000;
pub const MAX_REPUTATION_SCORE: i64 = 10000;

/// Reputation change amounts
pub const MILESTONE_SUCCESS_BONUS: i64 = 100;
pub const MILESTONE_FAILURE_PENALTY: i64 = -50;
pub const PROJECT_COMPLETION_BONUS: i64 = 200;
pub const PROJECT_ABANDONMENT_PENALTY: i64 = -100;
pub const PEER_REVIEW_BONUS: i64 = 25;
pub const DISPUTE_RESOLUTION_PENALTY: i64 = -75;

/// Tier thresholds (default values)
pub const DEFAULT_BRONZE_THRESHOLD: u64 = 100;
pub const DEFAULT_SILVER_THRESHOLD: u64 = 500;
pub const DEFAULT_GOLD_THRESHOLD: u64 = 1500;
pub const DEFAULT_PLATINUM_THRESHOLD: u64 = 3000;
pub const DEFAULT_DIAMOND_THRESHOLD: u64 = 5000;

/// Event types for reputation tracking
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ReputationEvent {
    MilestoneCompleted,
    MilestoneFailed,
    ProjectCompleted,
    ProjectAbandoned,
    PeerReviewPositive,
    DisputeResolved,
    Custom(i64), // Custom reputation change amount
}

impl ReputationEvent {
    pub fn get_score_delta(&self) -> i64 {
        match self {
            Self::MilestoneCompleted => MILESTONE_SUCCESS_BONUS,
            Self::MilestoneFailed => MILESTONE_FAILURE_PENALTY,
            Self::ProjectCompleted => PROJECT_COMPLETION_BONUS,
            Self::ProjectAbandoned => PROJECT_ABANDONMENT_PENALTY,
            Self::PeerReviewPositive => PEER_REVIEW_BONUS,
            Self::DisputeResolved => DISPUTE_RESOLUTION_PENALTY,
            Self::Custom(amount) => *amount,
        }
    }
}
