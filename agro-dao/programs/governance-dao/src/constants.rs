use anchor_lang::prelude::*;

/// Governance seeds
#[constant]
pub const GOVERNANCE_SEED: &[u8] = b"governance";

#[constant]
pub const PROPOSAL_SEED: &[u8] = b"proposal";

#[constant]
pub const VOTE_SEED: &[u8] = b"vote";

/// Cross-program IDs
pub const TREASURY_PROGRAM_ID: Pubkey = pubkey!("9cDozwVvb4EHtzVwtbseAkuWRXjPfSmhuoiLiVK8yMY8");
pub const REPUTATION_PROGRAM_ID: Pubkey = pubkey!("CwcGWv7BjjJKVXKqTaLmtvbXpBn2XqULeeJbPgGvfanN");
pub const RESEARCH_PROGRAM_ID: Pubkey = pubkey!("DF1y7PHHo7ekNEKztCMTDsZ3TrYdLAhgBCFQPzoi3PHw");

/// Basis points constants
pub const BASIS_POINTS_MAX: u16 = 10_000;

/// Time constants
pub const SECONDS_PER_DAY: i64 = 86_400;
pub const EXECUTION_WINDOW_DAYS: i64 = 7;

/// Proposal limits
pub const MAX_TITLE_LENGTH: usize = 100;
pub const MAX_DESCRIPTION_LENGTH: usize = 1000;
pub const MAX_INSTRUCTION_DATA_LENGTH: usize = 2048;
