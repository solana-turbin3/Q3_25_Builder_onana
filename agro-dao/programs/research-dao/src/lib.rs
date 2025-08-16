pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FUpDQNRZyx2u8uEnerDP9Y6gRT4HUaTZcU7ViziYxWQp");

#[program]
pub mod research_dao {
    use super::*;

    // Research Management
    pub fn create_researcher_profile(
        ctx: Context<CreateResearcherProfile>,
        name: String,
        bio: String,
        specialization: String,
    ) -> Result<()> {
        ctx.accounts.create_researcher_profile(name, bio, specialization, ctx.bumps.researcher_profile)
    }

    pub fn verify_researcher(ctx: Context<VerifyResearcher>) -> Result<()> {
        ctx.accounts.verify_researcher()
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        category: ResearchCategory,
        funding_target: u64,
        funding_deadline: i64,
        milestones: Vec<Milestone>,
        ipfs_hash: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.create_proposal(
            title,
            description,
            category,
            funding_target,
            funding_deadline,
            milestones,
            ipfs_hash,
            ctx.bumps.research_proposal,
        )
    }

    pub fn submit_proposal_for_funding(ctx: Context<SubmitProposalForFunding>) -> Result<()> {
        ctx.accounts.submit_proposal_for_funding()
    }

    pub fn publish_milestone(
        ctx: Context<PublishMilestone>,
        milestone_index: u8,
        evidence_ipfs_hash: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.publish_milestone(milestone_index, evidence_ipfs_hash)
    }

    pub fn publish_findings(
        ctx: Context<PublishFindings>,
        findings_ipfs_hash: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.publish_findings(findings_ipfs_hash)
    }

    // Treasury DAO validation functions
    pub fn validate_proposal_for_funding(
        ctx: Context<ValidateProposalForFunding>,
        proposal_id: u64,
        funding_amount: u64,
    ) -> Result<()> {
        instructions::validate_proposal_for_funding(ctx, proposal_id, funding_amount)
    }
}
