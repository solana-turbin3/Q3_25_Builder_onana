use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GovernanceConfig {
    pub bump: u8,
    pub agro_token_mint: Pubkey,
    pub governance_authority: Pubkey,
    pub quorum_threshold_bps: u16,
    pub approval_threshold_bps: u16,
    pub parameter_change_threshold_bps: u16,
    pub min_agro_to_propose: u64,
    pub min_agro_to_vote: u64,
    pub max_reputation_weight_bps: u16,
    pub total_proposals: u64,
    pub emergency_pause: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[account]
#[derive(InitSpace)]
pub struct Proposal {
    pub proposal_id: u64,
    pub bump: u8,
    pub proposer: Pubkey,
    #[max_len(100)]
    pub title: String,
    #[max_len(1000)]
    pub description: String,
    pub proposal_type: ProposalType,
    pub status: ProposalStatus,
    pub created_at: i64,
    pub voting_starts_at: i64,
    pub voting_ends_at: i64,
    pub execution_window_end: i64,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub total_votes: u64,
    pub total_voting_power: u64,
    pub executed_at: Option<i64>,
    #[max_len(2048)]
    pub instruction_data: Option<Vec<u8>>,
}

#[account]
#[derive(InitSpace)]
pub struct Vote {
    pub proposal_id: u64,
    pub bump: u8,
    pub voter: Pubkey,
    pub vote_choice: VoteChoice,
    pub voting_power: u64,
    pub agro_amount: u64,
    pub reputation_weight: u64,
    pub cast_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq)]
pub enum ProposalType {
    Treasury,
    Research,
    ParameterChange,
    Emergency,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq)]
pub enum ProposalStatus {
    Active,
    Approved,
    Rejected,
    Executed,
    Expired,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}

impl GovernanceConfig {
    pub const SEED: &'static [u8] = b"governance";
}

impl Proposal {
    pub const SEED: &'static [u8] = b"proposal";
    
    pub fn is_voting_active(&self, current_time: i64) -> bool {
        current_time >= self.voting_starts_at && current_time <= self.voting_ends_at
    }
    
    pub fn is_voting_ended(&self, current_time: i64) -> bool {
        current_time > self.voting_ends_at
    }
    
    pub fn is_execution_window_open(&self, current_time: i64) -> bool {
        current_time <= self.execution_window_end
    }
}

impl Vote {
    pub const SEED: &'static [u8] = b"vote";
}

