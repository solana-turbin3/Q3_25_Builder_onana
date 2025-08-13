use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
#[derive(InitSpace)]
pub struct TreasuryConfig {
    pub bump: u8,
    pub authority: Pubkey,
    pub agro_mint: Pubkey,
    pub fee_rate_bps: u16,
    pub min_reserve_ratio_bps: u16,
    pub emergency_pause: bool,
    pub emergency_pause_timestamp: i64,
    #[max_len(MAX_SUPPORTED_TOKENS)]
    pub supported_tokens: Vec<Pubkey>,
    pub total_agro_minted: u64,
    pub total_fees_collected: u64,
    pub created_at: i64,
}

#[account]
#[derive(InitSpace)]
pub struct TokenVault {
    pub bump: u8,
    pub token_mint: Pubkey,
    pub vault_authority: Pubkey,
    pub total_deposits: u64,
    pub available_balance: u64,
    pub allocated_to_proposals: u64,
    pub reserved_amount: u64,
    pub yield_positions: u64,
    pub created_at: i64,
}

#[account]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub bump: u8,
    pub owner: Pubkey,
    pub total_agro_minted: u64,
    #[max_len(5)]
    pub deposits: Vec<TokenDeposit>,
    pub last_activity: i64,
    pub created_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct TokenDeposit {
    pub token_mint: Pubkey,
    pub amount: u64,
    pub agro_minted: u64,
    pub timestamp: i64,
}

#[account]
#[derive(InitSpace)]
pub struct ProposalFunding {
    pub bump: u8,
    #[max_len(MAX_PROPOSAL_ID_LENGTH)]
    pub proposal_id: String,
    pub research_program_id: Pubkey, // Link to research DAO program
    pub proposal_pda: Pubkey, // Link to research proposal PDA
    pub total_committed: u64,
    pub total_distributed: u64,
    #[max_len(MAX_FUNDING_SOURCES)]
    pub funding_sources: Vec<FundingSource>,
    pub status: ProposalFundingStatus,
    #[max_len(10)]
    pub milestone_distributions: Vec<MilestoneDistribution>,
    pub created_at: i64,
    pub last_updated: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct FundingSource {
    pub stakeholder: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub agro_burned: u64,
    pub timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct MilestoneDistribution {
    pub milestone_index: u8,
    pub amount_distributed: u64,
    pub token_mint: Pubkey,
    pub recipient: Pubkey,
    pub distributed_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq)]
pub enum ProposalFundingStatus {
    Active,
    Approved,
    Distributed,
    Cancelled,
}