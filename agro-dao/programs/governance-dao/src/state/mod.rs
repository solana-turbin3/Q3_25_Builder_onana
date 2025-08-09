use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
pub struct GovernanceConfig {
    pub bump: u8,
    pub governance_authority: Pubkey,
    pub agro_token_mint: Pubkey,
    pub treasury_program_id: Pubkey,
    pub research_program_id: Pubkey,
    
    // Voting Parameters
    pub min_agro_to_propose: u64,
    pub min_agro_to_vote: u64,
    pub quorum_threshold_bps: u16,
    pub approval_threshold_bps: u16,
    pub parameter_change_threshold_bps: u16,
    
    // Reputation Weighting
    pub max_reputation_weight_bps: u16,
    
    // System State
    pub emergency_pause: bool,
    pub total_proposals_created: u64,
    pub created_at: i64,
    pub last_updated: i64,
}

impl GovernanceConfig {
    pub const INIT_SPACE: usize = 8 + // discriminator
        1 + // bump
        32 + // governance_authority
        32 + // agro_token_mint
        32 + // treasury_program_id
        32 + // research_program_id
        8 + // min_agro_to_propose
        8 + // min_agro_to_vote
        2 + // quorum_threshold_bps
        2 + // approval_threshold_bps
        2 + // parameter_change_threshold_bps
        2 + // max_reputation_weight_bps
        1 + // emergency_pause
        8 + // total_proposals_created
        8 + // created_at
        8; // last_updated
}

#[account]
pub struct Proposal {
    pub bump: u8,
    pub proposal_id: u64,
    pub proposer: Pubkey,
    pub proposal_type: ProposalType,
    
    // Proposal Content
    pub title: String,
    pub description: String,
    
    // Timing
    pub created_at: i64,
    pub voting_start_time: i64,
    pub voting_end_time: i64,
    pub execution_available_at: i64,
    pub execution_expires_at: i64,
    
    // Voting Results
    pub total_votes_for: u64,
    pub total_votes_against: u64,
    pub total_abstain_votes: u64,
    pub total_voters: u32,
    pub quorum_reached: bool,
    
    // Execution
    pub proposal_status: ProposalStatus,
    pub instruction_data: Option<Vec<u8>>,
    pub executed_at: Option<i64>,
    pub executed_by: Option<Pubkey>,
    pub failure_reason: Option<String>,
}

impl Proposal {
    pub const INIT_SPACE: usize = 8 + // discriminator
        1 + // bump
        8 + // proposal_id
        32 + // proposer
        1 + // proposal_type
        4 + MAX_PROPOSAL_TITLE_LENGTH + // title
        4 + MAX_PROPOSAL_DESCRIPTION_LENGTH + // description
        8 + // created_at
        8 + // voting_start_time
        8 + // voting_end_time
        8 + // execution_available_at
        8 + // execution_expires_at
        8 + // total_votes_for
        8 + // total_votes_against
        8 + // total_abstain_votes
        4 + // total_voters
        1 + // quorum_reached
        1 + // proposal_status
        1 + 4 + 1000 + // instruction_data (Option<Vec<u8>>)
        1 + 8 + // executed_at (Option<i64>)
        1 + 32 + // executed_by (Option<Pubkey>)
        1 + 4 + MAX_EXECUTION_RESULT_LENGTH; // failure_reason (Option<String>)

    pub fn is_voting_active(&self, current_timestamp: i64) -> bool {
        current_timestamp >= self.voting_start_time && 
        current_timestamp <= self.voting_end_time &&
        self.proposal_status == ProposalStatus::Active
    }
}

#[account]
pub struct Vote {
    pub bump: u8,
    pub proposal_id: u64,
    pub voter: Pubkey,
    pub vote_choice: VoteChoice,
    pub agro_weight: u64,
    pub reputation_weight: u64,
    pub total_weight: u64,
    pub cast_at: i64,
    pub is_delegate_vote: bool,
}

impl Vote {
    pub const INIT_SPACE: usize = 8 + // discriminator
        1 + // bump
        8 + // proposal_id
        32 + // voter
        1 + // vote_choice
        8 + // agro_weight
        8 + // reputation_weight
        8 + // total_weight
        8 + // cast_at
        1; // is_delegate_vote

    pub fn calculate_voting_weight(
        agro_balance: u64,
        reputation_balance: u64,
        max_reputation_weight_bps: u16,
    ) -> Result<u64> {
        // Calculate reputation multiplier (capped at max_reputation_weight_bps)
        let reputation_multiplier_bps = std::cmp::min(
            reputation_balance.saturating_mul(100), // 1 reputation = 1% boost
            max_reputation_weight_bps as u64
        );

        // Apply reputation boost to AGRO balance
        let reputation_boost = agro_balance
            .saturating_mul(reputation_multiplier_bps)
            .saturating_div(10000);

        Ok(agro_balance.saturating_add(reputation_boost))
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalType {
    Treasury,    // Funding proposals, treasury management
    Parameter,   // Governance parameter changes
    Emergency,   // Emergency actions (pause/unpause, emergency withdrawals)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Active,      // Currently in voting period
    Approved,    // Voting passed, awaiting execution
    Executed,    // Successfully executed
    Failed,      // Failed to pass or execution failed
    Expired,     // Execution window expired
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum VoteChoice {
    For,
    Against,
    Abstain,
}
