use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct PublishMilestone<'info> {
    #[account(
        mut,    
        seeds = [b"proposal", research_proposal.researcher.key().as_ref(), research_proposal.id.to_le_bytes().as_ref()],
        bump = research_proposal.bump,
        has_one = researcher @ ErrorCode::UnauthorizedResearcher,       
        // Allow publishing milestones once work has started. If the proposal
        // is still in Draft it must be submitted first. Accept either
        // SubmittedForFunding (first milestone will move it to InProgress)
        // or already InProgress.
        constraint = research_proposal.status == ProposalStatus::InProgress
            || research_proposal.status == ProposalStatus::SubmittedForFunding @ ErrorCode::InvalidProposalStatus
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

impl<'info> PublishMilestone<'info> {
    pub fn publish_milestone(
        &mut self,
        milestone_index: u8,
        ipfs_evidence_hash: [u8; 32],
    ) -> Result<()> {
        let current_timestamp = Clock::get()?.unix_timestamp;
        
        // Validate milestone index
        require!(
            (milestone_index as usize) < self.research_proposal.milestones.len(),
            ErrorCode::InvalidMilestoneIndex
        );

        // If this is the first milestone after submission, transition to InProgress
        if self.research_proposal.status == ProposalStatus::SubmittedForFunding {
            self.research_proposal.status = ProposalStatus::InProgress;
        }

        let milestone = &mut self.research_proposal.milestones[milestone_index as usize];
        
        // Check if milestone is not already completed
        require!(!milestone.is_completed, ErrorCode::MilestoneAlreadyCompleted);

        // Update milestone
        milestone.is_completed = true;
        milestone.completion_date = Some(current_timestamp);
        milestone.ipfs_evidence_hash = Some(ipfs_evidence_hash);

        // Increase researcher reputation (5 points per milestone)
        self.researcher_profile.reputation_score = self.researcher_profile.reputation_score.saturating_add(5);

        // Check if all milestones are completed
        let all_completed = self.research_proposal.milestones.iter().all(|m| m.is_completed);
        if all_completed {
            self.research_proposal.status = ProposalStatus::Completed;
            self.researcher_profile.completed_projects = self.researcher_profile.completed_projects.checked_add(1).unwrap();
            // Additional reputation bonus for completing all milestones
            self.researcher_profile.reputation_score = self.researcher_profile.reputation_score.saturating_add(20);
        }

        emit!(MilestonePublished {
            proposal_id: self.research_proposal.id,
            researcher: self.researcher.key(),
            milestone_index,
            timestamp: current_timestamp,
            all_milestones_completed: all_completed,
        });

        Ok(())
    }
}

#[event]
pub struct MilestonePublished { 
    pub proposal_id: u64,
    pub researcher: Pubkey,
    pub milestone_index: u8,
    pub timestamp: i64,
    pub all_milestones_completed: bool,
}
