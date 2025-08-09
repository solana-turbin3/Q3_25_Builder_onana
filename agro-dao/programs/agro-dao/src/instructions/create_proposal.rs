use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        seeds = [b"proposal", protocol_state.proposal_id_counter.to_le_bytes().as_ref()],
        bump,
        payer = researcher,
        space = 8 + ResearchProposal::INIT_SPACE
    )]
    pub research_proposal: Account<'info, ResearchProposal>,

    #[account(
        mut,
        seeds = [b"protocol_state"],
        bump = protocol_state.bump,
        constraint = !protocol_state.is_paused @ ErrorCode::ProtocolPaused
    )]
    pub protocol_state: Account<'info, ProtocolState>,

    #[account(
        mut,
        seeds = [b"researcher", researcher.key().as_ref()],
        bump = researcher_profile.bump,
        has_one = researcher @  ErrorCode::UnauthorizedResearcher
    )]
    pub researcher_profile: Account<'info, ResearcherProfile>,

    #[account(mut)]
    pub researcher: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateProposal<'info> {
    pub fn create_proposal(
        &mut self,
        title: String,
        description: String,
        category: ResearchCategory,
        funding_target: u64,
        funding_deadline_days: u64,
        milestones: Vec<Milestone>,
        ipfs_hash: [u8; 32],
        bumps: &CreateProposalBumps,
    ) -> Result<()> {
        // Validation
        require!(title.len() <= 100, ErrorCode::TitleTooLong);
        require!(description.len() <= 500, ErrorCode::DescriptionTooLong);
        require!(funding_target >= self.protocol_state.min_funding_threshold, ErrorCode::InsufficientFundingTarget);
        require!(milestones.len() <= 10, ErrorCode::TooManyMilestones);
        require!(funding_deadline_days >= 7 && funding_deadline_days <= 90, ErrorCode::InvalidFundingDeadline);

        let current_timestamp = Clock::get()?.unix_timestamp;
        let funding_deadline = current_timestamp + (funding_deadline_days as i64 * 24 * 60 * 60);

        // Create proposal
        self.research_proposal.set_inner(ResearchProposal {
            id: self.protocol_state.proposal_id_counter,
            researcher: self.researcher.key(),
            title,
            description,
            category,
            funding_target,
            current_funding: 0,
            status: ProposalStatus::Draft,
            milestones,
            creation_timestamp: current_timestamp,
            funding_deadline,
            ipfs_hash,
            findings_ipfs_hash: None,
            bump: bumps.research_proposal,
        });

        // Update protocol state
        self.protocol_state.proposal_id_counter = self.protocol_state.proposal_id_counter.checked_add(1).unwrap();

        // Update researcher profile
        self.researcher_profile.total_proposals = self.researcher_profile.total_proposals.checked_add(1).unwrap();

        emit!(ProposalCreated {
            proposal_id: self.research_proposal.id,
            researcher: self.researcher.key(),
            funding_target,
            timestamp: current_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct ProposalCreated {
    pub proposal_id: u64,
    pub researcher: Pubkey,
    pub funding_target: u64,
    pub timestamp: i64,
}
