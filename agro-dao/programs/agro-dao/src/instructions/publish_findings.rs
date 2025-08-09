use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct PublishFindings<'info> {
    #[account(
        mut,
        seeds = [b"proposal", research_proposal.id.to_le_bytes().as_ref()],
        bump = research_proposal.bump,
        has_one = researcher @ ErrorCode::UnauthorizedResearcher,
        constraint = research_proposal.status == ProposalStatus::Completed @ ErrorCode::InvalidProposalStatus
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
        has_one = researcher @ ErrorCode::UnauthorizedResearcher
    )]
    pub researcher_profile: Account<'info, ResearcherProfile>,

    #[account(mut)]
    pub researcher: Signer<'info>,  
    pub system_program: Program<'info, System>,

}

impl<'info> PublishFindings<'info> {
    pub fn publish_findings(
        &mut self,
        findings_ipfs_hash: [u8; 32],
    ) -> Result<()> {
        let current_timestamp = Clock::get()?.unix_timestamp;
        
        // Check if findings haven't been published yet
        require!(
            self.research_proposal.findings_ipfs_hash.is_none(),
            ErrorCode::FindingsAlreadyPublished
        );

        // Update proposal with findings
        self.research_proposal.findings_ipfs_hash = Some(findings_ipfs_hash);

        // Update protocol research data counter
        self.protocol_state.research_data_counter = self.protocol_state.research_data_counter.checked_add(1).unwrap();

        // Award significant reputation boost for publishing findings (50 points)
        self.researcher_profile.reputation_score = self.researcher_profile.reputation_score.saturating_add(50);

        emit!(FindingsPublished {
            proposal_id: self.research_proposal.id,
            researcher: self.researcher.key(),
            findings_ipfs_hash,
            timestamp: current_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct FindingsPublished {
    pub proposal_id: u64,
    pub researcher: Pubkey,
    pub findings_ipfs_hash: [u8; 32],
    pub timestamp: i64,
}
