use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod cpi_helpers;

use instructions::*;
pub use constants::*;
pub use cpi_helpers::*;

declare_id!("FGmtS6VD6rhP9k8meqnYoT2w2txRfzTQB8DeXDcoxzaR");

#[program]
pub mod governance_dao {
    use super::*;

    /// Initialize the governance configuration
    pub fn initialize_governance(
        ctx: Context<InitializeGovernance>,
        bump: u8,
        agro_token_mint: Pubkey,
        governance_authority: Pubkey,
        quorum_threshold_bps: u16,
        approval_threshold_bps: u16,
        parameter_change_threshold_bps: u16,
        min_agro_to_propose: u64,
        min_agro_to_vote: u64,
        max_reputation_weight_bps: u16,
    ) -> Result<()> {
        ctx.accounts.initialize_governance(
            bump,
            agro_token_mint,
            governance_authority,
            quorum_threshold_bps,
            approval_threshold_bps,
            parameter_change_threshold_bps,
            min_agro_to_propose,
            min_agro_to_vote,
            max_reputation_weight_bps,
        )
    }

    /// Create a new governance proposal
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        proposal_id: u64,
        bump: u8,
        title: String,
        description: String,
        proposal_type: state::ProposalType,
        voting_period_days: u8,
        instruction_data: Option<Vec<u8>>,
    ) -> Result<()> {
        ctx.accounts.create_proposal(
            proposal_id,
            bump,
            title,
            description,
            proposal_type,
            voting_period_days,
            instruction_data,
            &ctx.bumps,
        )
    }

    /// Cast a vote on a proposal
    pub fn cast_vote(
        ctx: Context<CastVote>,
        proposal_id: u64,
        vote_choice: state::VoteChoice,
        bump: u8,
    ) -> Result<()> {
        ctx.accounts.cast_vote(proposal_id, vote_choice, bump)
    }

    /// Tally votes for a proposal after voting period ends
    pub fn tally_votes(
        ctx: Context<TallyVotes>,
        proposal_id: u64,
    ) -> Result<()> {
        ctx.accounts.tally_votes(proposal_id)
    }

    /// Execute an approved proposal within the execution window
    pub fn execute_proposal(
        ctx: Context<ExecuteProposal>,
        proposal_id: u64,
    ) -> Result<()> {
        ctx.accounts.execute_proposal(proposal_id)
    }

    /// Update governance configuration parameters
    pub fn update_governance_config(
        ctx: Context<UpdateGovernanceConfig>,
        new_quorum_threshold_bps: Option<u16>,
        new_approval_threshold_bps: Option<u16>,
        new_parameter_change_threshold_bps: Option<u16>,
        new_min_agro_to_propose: Option<u64>,
        new_min_agro_to_vote: Option<u64>,
        new_max_reputation_weight_bps: Option<u16>,
        new_governance_authority: Option<Pubkey>,
        emergency_pause: Option<bool>,
    ) -> Result<()> {
        ctx.accounts.update_governance_config(
            new_quorum_threshold_bps,
            new_approval_threshold_bps,
            new_parameter_change_threshold_bps,
            new_min_agro_to_propose,
            new_min_agro_to_vote,
            new_max_reputation_weight_bps,
            new_governance_authority,
            emergency_pause,
        )
    }
}

