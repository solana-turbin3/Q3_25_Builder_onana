use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::state::{GovernanceConfig, Proposal, ProposalType, ProposalStatus};
use crate::constants::*;
use crate::error::GovernanceError;

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct CreateProposal<'info> {
    #[account(
        mut,
        seeds = [GOVERNANCE_CONFIG_SEED],
        bump = governance_config.bump,
        constraint = !governance_config.emergency_pause @ GovernanceError::GovernancePaused
    )]
    pub governance_config: Account<'info, GovernanceConfig>,

    #[account(
        init,
        seeds = [PROPOSAL_SEED, &proposal_id.to_le_bytes()],
        bump,
        payer = proposer,
        space = 8 + Proposal::INIT_SPACE
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(
        constraint = proposer_agro_account.mint == governance_config.agro_token_mint @ GovernanceError::InsufficientAgroToPropose,
        constraint = proposer_agro_account.owner == proposer.key() @ GovernanceError::InsufficientAgroToPropose,
        constraint = proposer_agro_account.amount >= governance_config.min_agro_to_propose @ GovernanceError::InsufficientAgroToPropose
    )]
    pub proposer_agro_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub proposer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> CreateProposal<'info> {
    pub fn create_proposal(
        &mut self,
        proposal_id: u64,
        bump: u8,
        title: String,
        description: String,
        proposal_type: ProposalType,
        voting_period_days: u8,
        instruction_data: Option<Vec<u8>>,
    ) -> Result<()> {
        // Validate input lengths
        require!(
            title.len() <= MAX_PROPOSAL_TITLE_LENGTH,
            GovernanceError::TitleTooLong
        );
        require!(
            description.len() <= MAX_PROPOSAL_DESCRIPTION_LENGTH,
            GovernanceError::DescriptionTooLong
        );

        let clock = Clock::get()?;

        // Calculate timing
        let voting_start_time = clock.unix_timestamp + PROPOSAL_DELAY;
        let voting_end_time = voting_start_time + (voting_period_days as i64 * 24 * 60 * 60)
            .clamp(MIN_VOTING_PERIOD, MAX_VOTING_PERIOD);

        // Basic validation per type (minimal for now)
        let _ = &proposal_type; // placeholder to mark use

        self.proposal.set_inner(Proposal {
            bump,
            proposal_id,
            proposer: self.proposer.key(),
            proposal_type: proposal_type.clone(),
            title: title.clone(),
            description: description.clone(),
            created_at: clock.unix_timestamp,
            voting_start_time,
            voting_end_time,
            execution_available_at: 0,
            execution_expires_at: 0,
            proposal_status: ProposalStatus::Active,
            total_votes_for: 0,
            total_votes_against: 0,
            total_abstain_votes: 0,
            total_voters: 0,
            quorum_reached: false,
            instruction_data,
            executed_at: None,
            executed_by: None,
            failure_reason: None,
        });

        // Update governance config
        self.governance_config.total_proposals_created = proposal_id;

        emit!(ProposalCreated {
            proposal_id,
            proposer: self.proposer.key(),
            proposal_type,
            title,
            description,
            voting_start_time,
            voting_end_time,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    // Placeholder for future rich validation per proposal type
    // fn validate_proposal_type(&self, _proposal_type: &ProposalType) -> Result<()> { Ok(()) }
}

#[event]
pub struct ProposalCreated {
    pub proposal_id: u64,
    pub proposer: Pubkey,
    pub proposal_type: ProposalType,
    pub title: String,
    pub description: String,
    pub voting_start_time: i64,
    pub voting_end_time: i64,
    pub timestamp: i64,
}
