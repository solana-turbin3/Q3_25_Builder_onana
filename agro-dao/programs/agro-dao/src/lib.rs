pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CRAooBpTSWbr3m5YKm2cEWAQcHgqqARmZXYj5Z1RSsX2");

#[program]
pub mod agro_dao {
    use super::*;

    // Protocol Management
    pub fn initialize_protocol(ctx: Context<InitializeProtocol>) -> Result<()> {
        ctx.accounts.initialize_protocol(ctx.bumps.protocol_state)
    }

    pub fn update_protocol(
        ctx: Context<UpdateProtocol>, 
        params: UpdateProtocolParams
    ) -> Result<()> {
        ctx.accounts.update_protocol(params)
    }

    // Research Management
    pub fn create_researcher_profile(
        ctx: Context<CreateResearcherProfile>,
        name: String,
        bio: String,
        specialization: String,
    ) -> Result<()> {
        ctx.accounts.create_researcher_profile(name, bio, specialization, &ctx.bumps)
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
        funding_deadline_days: u64,
        milestones: Vec<Milestone>,
        ipfs_hash: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.create_proposal(
            title,
            description,
            category,
            funding_target,
            funding_deadline_days,
            milestones,
            ipfs_hash,
            &ctx.bumps,
        )
    }

    pub fn submit_proposal_for_funding(ctx: Context<SubmitProposalForFunding>) -> Result<()> {
        ctx.accounts.submit_proposal_for_funding()
    }

    pub fn publish_milestone(
        ctx: Context<PublishMilestone>,
        milestone_index: u8,
        ipfs_evidence_hash: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.publish_milestone(milestone_index, ipfs_evidence_hash)
    }

    pub fn publish_findings(
        ctx: Context<PublishFindings>,
        findings_ipfs_hash: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.publish_findings(findings_ipfs_hash)
    }
}
