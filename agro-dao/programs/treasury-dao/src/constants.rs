// Treasury Config Constants
pub const MAX_SUPPORTED_TOKENS: usize = 10;
pub const MAX_FUNDING_SOURCES: usize = 20;

// Fee Constants
pub const MAX_FEE_RATE_BPS: u16 = 1000; // 10% max fee
pub const MIN_RESERVE_RATIO_BPS: u16 = 1000; // 10% min reserve
pub const MAX_RESERVE_RATIO_BPS: u16 = 5000; // 50% max reserve

// Validation Constants
pub const MAX_PROPOSAL_ID_LENGTH: usize = 50;
pub const MAX_DISTRIBUTION_AMOUNT: u64 = 1_000_000_000_000; // 1M tokens max in one distribution

// Time Constants
pub const EMERGENCY_PAUSE_DURATION: i64 = 7 * 24 * 60 * 60; // 7 days in seconds

// Seeds
pub const TREASURY_CONFIG_SEED: &[u8] = b"treasury_config";
pub const TOKEN_VAULT_SEED: &[u8] = b"token_vault";
pub const FEE_VAULT_SEED: &[u8] = b"fee_vault";
pub const AGRO_MINT_SEED: &[u8] = b"agro_mint";
pub const STAKE_ACCOUNT_SEED: &[u8] = b"stake_account";
pub const PROPOSAL_FUNDING_SEED: &[u8] = b"proposal_funding";
