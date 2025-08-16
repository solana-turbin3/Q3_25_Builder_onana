use anchor_lang::prelude::*;

/// Governance seeds
#[constant]
pub const GOVERNANCE_SEED: &[u8] = b"governance";

#[constant]
pub const PROPOSAL_SEED: &[u8] = b"proposal";

#[constant]
pub const VOTE_SEED: &[u8] = b"vote";

/// Cross-program IDs
pub const TREASURY_PROGRAM_ID: Pubkey = pubkey!("BT9K4n1w56VP6pL9fAwZesLCJWJ9rmaJ2d3XZxGuGkYB");
pub const REPUTATION_PROGRAM_ID: Pubkey = pubkey!("WZ13w2w964gyDhpd3GWpFuCJQWYaGNAgybt3rrrUuxD");
pub const RESEARCH_PROGRAM_ID: Pubkey = pubkey!("FUpDQNRZyx2u8uEnerDP9Y6gRT4HUaTZcU7ViziYxWQp");

/// Basis points constants
pub const BASIS_POINTS_MAX: u16 = 10_000;

/// Time constants
pub const SECONDS_PER_DAY: i64 = 86_400;
pub const EXECUTION_WINDOW_DAYS: i64 = 7;

/// Proposal limits
pub const MAX_TITLE_LENGTH: usize = 100;
pub const MAX_DESCRIPTION_LENGTH: usize = 1000;
pub const MAX_INSTRUCTION_DATA_LENGTH: usize = 2048;
