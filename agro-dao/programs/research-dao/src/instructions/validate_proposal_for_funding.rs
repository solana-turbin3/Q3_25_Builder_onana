use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct ValidateProposalForFunding<'info> {
    #[account(
        seeds = [b"proposal", proposal.researcher.as_ref(), &proposal.id.to_le_bytes()],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::SubmittedForFunding @ ErrorCode::ProposalNotEligibleForFunding
    )]
    pub proposal: Account<'info, ResearchProposal>,
    
    #[account(
        seeds = [b"researcher", proposal.researcher.as_ref()],
        bump = researcher_profile.bump,
        constraint = researcher_profile.is_verified @ ErrorCode::ResearcherNotVerifiedForFunding
    )]
    pub researcher_profile: Account<'info, ResearcherProfile>,
}

pub fn validate_proposal_for_funding(
    ctx: Context<ValidateProposalForFunding>,
    proposal_id: u64,
    funding_amount: u64,
) -> Result<()> {
    let proposal = &ctx.accounts.proposal;
    
    // Validate proposal is eligible for funding
    require!(
        proposal.id == proposal_id,
        ErrorCode::InvalidProposalForFunding
    );
    
    require!(
        funding_amount <= proposal.funding_target,
        ErrorCode::ExcessiveFunding
    );
    
    require!(
        Clock::get()?.unix_timestamp < proposal.funding_deadline,
        ErrorCode::FundingDeadlinePassed
    );
    
    Ok(())
}
