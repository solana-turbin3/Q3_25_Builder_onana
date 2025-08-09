use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

// Research-specific constants
pub const MAX_NAME_LENGTH: usize = 50;
pub const MAX_BIO_LENGTH: usize = 200;
pub const MAX_SPECIALIZATION_LENGTH: usize = 100;
pub const MAX_TITLE_LENGTH: usize = 100;
pub const MAX_DESCRIPTION_LENGTH: usize = 500;
pub const MAX_MILESTONES: usize = 10;
pub const MIN_REPUTATION_FOR_FUNDING: u32 = 100;
