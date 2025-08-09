use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct SubmitProposalForFunding<'info> {
    #[account(
        mut,
        seeds = [b"proposal", research_proposal.researcher.key().as_ref(), research_proposal.id.to_le_bytes().as_ref()],
        bump = research_proposal.bump,
        has_one = researcher @ ErrorCode::UnauthorizedResearcher,
        constraint = research_proposal.status == ProposalStatus::Draft @ ErrorCode::InvalidProposalStatus
    )]
    pub research_proposal: Account<'info, ResearchProposal>,
    
    #[account(
        seeds = [b"researcher", researcher.key().as_ref()],
        bump = researcher_profile.bump,
        has_one = researcher @ ErrorCode::UnauthorizedResearcher
    )]
    pub researcher_profile: Account<'info, ResearcherProfile>,
    
    #[account(mut)]
    pub researcher: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> SubmitProposalForFunding<'info> {
    pub fn submit_proposal_for_funding(&mut self) -> Result<()> {
        // Check if funding deadline hasn't passed
        let current_timestamp = Clock::get()?.unix_timestamp;
        require!(
            current_timestamp < self.research_proposal.funding_deadline,
            ErrorCode::FundingDeadlineExpired
        );

        // Check if researcher has minimum reputation for larger proposals
        if self.research_proposal.funding_target > 10 * anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL {
            require!(
                self.researcher_profile.reputation_score >= 100,
                ErrorCode::InsufficientReputation
            );
        }

        // Update proposal status
        self.research_proposal.status = ProposalStatus::SubmittedForFunding;

        emit!(ProposalSubmittedForFunding {
            proposal_id: self.research_proposal.id,
            researcher: self.researcher.key(),
            funding_target: self.research_proposal.funding_target,
            deadline: self.research_proposal.funding_deadline,
            timestamp: current_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct ProposalSubmittedForFunding {
    pub proposal_id: u64,
    pub researcher: Pubkey,
    pub funding_target: u64,
    pub deadline: i64,
    pub timestamp: i64,
}
