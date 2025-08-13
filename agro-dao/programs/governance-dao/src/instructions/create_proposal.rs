use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::error::*;

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(
        mut,
        seeds = [GOVERNANCE_SEED],
        bump = governance_config.bump,
        constraint = !governance_config.emergency_pause @ GovernanceError::GovernancePaused
    )]
    pub governance_config: Account<'info, GovernanceConfig>,

    #[account(
        init,
        payer = proposer,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [PROPOSAL_SEED, &governance_config.total_proposals.to_le_bytes()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub proposer: Signer<'info>,

    /// CHECK: Proposer's AGRO token account, validated in instruction
    pub proposer_agro_account: UncheckedAccount<'info>,

    /// CHECK: AGRO token mint address
    pub agro_token_mint: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
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
        bumps: &CreateProposalBumps,
    ) -> Result<()> {
        require!(title.len() <= MAX_TITLE_LENGTH, GovernanceError::TitleTooLong);
        require!(description.len() <= MAX_DESCRIPTION_LENGTH, GovernanceError::DescriptionTooLong);
        
        if let Some(ref data) = instruction_data {
            require!(data.len() <= MAX_INSTRUCTION_DATA_LENGTH, GovernanceError::InstructionDataTooLong);
        }
        
        require!(voting_period_days > 0 && voting_period_days <= 30, GovernanceError::InvalidVotingPeriod);

        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;
        
        let voting_starts_at = current_time;
        let voting_ends_at = current_time + (voting_period_days as i64 * SECONDS_PER_DAY);
        let execution_window_end = voting_ends_at + (EXECUTION_WINDOW_DAYS * SECONDS_PER_DAY);

        self.proposal.set_inner(Proposal {
            proposal_id,
            bump: bumps.proposal, // Use the actual derived bump
            proposer: self.proposer.key(),
            title: title.clone(),
            description,
            proposal_type: proposal_type.clone(),
            status: ProposalStatus::Active,
            created_at: current_time,
            voting_starts_at,
            voting_ends_at,
            execution_window_end,
            yes_votes: 0,
            no_votes: 0,
            total_votes: 0,
            total_voting_power: 0,
            executed_at: None,
            instruction_data,
        });

        // Increment total proposals
        self.governance_config.total_proposals = self.governance_config.total_proposals
            .checked_add(1)
            .ok_or(GovernanceError::ArithmeticOverflow)?;

        emit!(ProposalCreated {
            proposal_id,
            proposer: self.proposer.key(),
            title,
            proposal_type,
            voting_ends_at,
        });

        Ok(())
    }
}

#[event]
pub struct ProposalCreated {
    pub proposal_id: u64,
    pub proposer: Pubkey,
    pub title: String,
    pub proposal_type: ProposalType,
    pub voting_ends_at: i64,
}
