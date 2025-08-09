// Governance Config Constants
pub const MAX_PROPOSAL_TITLE_LENGTH: usize = 100;
pub const MAX_PROPOSAL_DESCRIPTION_LENGTH: usize = 500;
pub const MAX_EXECUTION_RESULT_LENGTH: usize = 200;
pub const EXECUTION_WINDOW: i64 = 7 * 24 * 60 * 60; // 7 days execution window

// Voting Constants
pub const MIN_VOTING_PERIOD: i64 = 24 * 60 * 60; // 1 day minimum
pub const MAX_VOTING_PERIOD: i64 = 30 * 24 * 60 * 60; // 30 days maximum
pub const PROPOSAL_DELAY: i64 = 24 * 60 * 60; // 24 hours
pub const EXECUTION_DELAY: i64 = 24 * 60 * 60; // 24 hours

// Threshold Constants (in basis points)
pub const MIN_QUORUM_THRESHOLD_BPS: u16 = 500;   // 5% minimum
pub const MAX_QUORUM_THRESHOLD_BPS: u16 = 5000;  // 50% maximum
pub const MIN_APPROVAL_THRESHOLD_BPS: u16 = 5000; // 50% minimum (simple majority)
pub const MAX_APPROVAL_THRESHOLD_BPS: u16 = 8000; // 80% maximum

// Weight Constants
pub const MAX_REPUTATION_WEIGHT_BPS: u16 = 300;  // 300% max (3x multiplier)
pub const MIN_AGRO_TO_PROPOSE: u64 = 1000_000_000_000; // 1000 AGRO (9 decimals)
pub const MIN_AGRO_TO_VOTE: u64 = 10_000_000_000; // 10 AGRO (9 decimals)

// Seeds
pub const GOVERNANCE_CONFIG_SEED: &[u8] = b"governance_config";
pub const PROPOSAL_SEED: &[u8] = b"proposal";
pub const VOTE_SEED: &[u8] = b"vote";
pub const EXECUTION_RECORD_SEED: &[u8] = b"execution_record";
