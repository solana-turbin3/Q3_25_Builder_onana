use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        seeds = [
            b"proposal",
            researcher.key().as_ref(),
            &((researcher_profile.total_proposals as u64).to_le_bytes())
        ],
        bump,
        payer = researcher,
        space = 8 + ResearchProposal::INIT_SPACE
    )]
    pub research_proposal: Account<'info, ResearchProposal>,

    #[account(
        mut,
        seeds = [b"researcher", researcher.key().as_ref()],
        bump = researcher_profile.bump,
        has_one = researcher @ ErrorCode::UnauthorizedResearcher
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
        funding_deadline: i64,
        milestones: Vec<Milestone>,
        ipfs_hash: [u8; 32],
        bump: u8,
    ) -> Result<()> {
        // Validation
        require!(title.len() <= 100, ErrorCode::TitleTooLong);
        require!(description.len() <= 500, ErrorCode::DescriptionTooLong);
        require!(funding_target > 0, ErrorCode::InsufficientFundingTarget);
        require!(milestones.len() <= 10, ErrorCode::TooManyMilestones);
        
        let current_timestamp = Clock::get()?.unix_timestamp;
        require!(funding_deadline > current_timestamp, ErrorCode::InvalidFundingDeadline);

        // Create proposal
        self.research_proposal.set_inner(ResearchProposal {
            id: self.researcher_profile.total_proposals as u64,
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
            bump,
        });

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
