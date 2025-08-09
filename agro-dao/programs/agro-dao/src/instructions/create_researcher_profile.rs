use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateResearcherProfile<'info> {
    #[account(
        init,
        seeds = [b"researcher", researcher.key().as_ref()],
        bump,
        payer = researcher,
        space = 8 + ResearcherProfile::INIT_SPACE
    )]
    pub researcher_profile: Account<'info, ResearcherProfile>,

    /// The protocol state to check if protocol is active
    #[account(
        mut,
        seeds = [b"protocol_state"],
        bump = protocol_state.bump,
        constraint = !protocol_state.is_paused @ ErrorCode::ProtocolPaused
    )]
    pub protocol_state: Account<'info, ProtocolState>,

    #[account(mut)]
    pub researcher: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateResearcherProfile<'info> {
    pub fn create_researcher_profile(
        &mut self,
        name: String,
        bio: String,
        specialization: String,
        bumps: &CreateResearcherProfileBumps,
    ) -> Result<()> {
        // Validation
        require!(name.len() <= 50, ErrorCode::NameTooLong);
        require!(bio.len() <= 200, ErrorCode::BioTooLong);
        require!(specialization.len() <= 100, ErrorCode::SpecializationTooLong);

        self.researcher_profile.set_inner(ResearcherProfile {
            researcher: self.researcher.key(),
            name,
            bio,
            specialization,
            reputation_score: 0,
            total_proposals: 0,
            completed_projects: 0,
            total_funding_received: 0,
            creation_timestamp: Clock::get()?.unix_timestamp,
            is_verified: false,
            bump: bumps.researcher_profile,
        });

        emit!(ResearcherProfileCreated {
            researcher: self.researcher.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct ResearcherProfileCreated {
    pub researcher: Pubkey,
    pub timestamp: i64,
}
