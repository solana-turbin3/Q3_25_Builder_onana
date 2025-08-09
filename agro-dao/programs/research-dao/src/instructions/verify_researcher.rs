use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct VerifyResearcher<'info> {
    #[account(
        mut,
        seeds = [b"researcher", researcher_profile.researcher.key().as_ref()],
        bump = researcher_profile.bump,
        constraint = !researcher_profile.is_verified @ ErrorCode::AlreadyVerified
    )]
    pub researcher_profile: Account<'info, ResearcherProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> VerifyResearcher<'info> {
    pub fn verify_researcher(&mut self) -> Result<()> {
        // Mark researcher as verified
        self.researcher_profile.is_verified = true;
        
        // Award reputation bonus for verification (100 points)
        self.researcher_profile.reputation_score = self.researcher_profile.reputation_score.saturating_add(100);

        emit!(ResearcherVerified {
            researcher: self.researcher_profile.researcher,
            authority: self.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct ResearcherVerified {
    pub researcher: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}
