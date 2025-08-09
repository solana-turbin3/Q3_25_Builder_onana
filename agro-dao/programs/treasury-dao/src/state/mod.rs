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

// Research DAO Integration Structs (for CPI)
#[account]
pub struct ResearcherProfile {
    pub researcher: Pubkey,
    pub name: String,
    pub bio: String,
    pub specialization: String,
    pub reputation_score: u64,
    pub total_proposals: u32,
    pub completed_projects: u32,
    pub total_funding_received: u64,
    pub creation_timestamp: i64,
    pub is_verified: bool,
    pub bump: u8,
}

#[account]
pub struct ResearchProposal {
    pub researcher: Pubkey,
    pub proposal_id: u32,
    pub title: String,
    pub description: String,
    pub category: ResearchCategory,
    pub funding_target: u64,
    pub funding_received: u64,
    pub funding_deadline: i64,
    pub milestones: Vec<Milestone>,
    pub status: ProposalStatus,
    pub ipfs_hash: [u8; 32],
    pub created_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ResearchCategory {
    CropScience,
    SoilHealth,
    Irrigation,
    PestManagement,
    SustainableAgriculture,
    ClimateResilience,
    Other,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ProposalStatus {
    Draft,
    UnderReview,
    Approved,
    Active,
    Completed,
    Rejected,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Milestone {
    pub title: String,
    pub description: String,
    pub funding_percentage: u8,
    pub deadline: i64,
    pub status: MilestoneStatus,
    pub evidence_ipfs_hash: Option<[u8; 32]>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum MilestoneStatus {
    Pending,
    InProgress,
    Completed,
    Verified,
}
