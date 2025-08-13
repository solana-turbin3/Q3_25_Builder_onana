use anchor_lang::prelude::*;
use crate::state::{ReputationConfig, UserReputation, ReputationTier};
use crate::constants::*;
use crate::error::*;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct UpdateReputation<'info> {
    #[account(
        seeds = [REPUTATION_CONFIG_SEED],
        bump = reputation_config.bump,
        constraint = reputation_config.is_active @ ReputationError::SystemNotActive
    )]
    pub reputation_config: Account<'info, ReputationConfig>,
    
    #[account(
        mut,
        seeds = [USER_REPUTATION_SEED, user.as_ref()],
        bump = user_reputation.bump
    )]
    pub user_reputation: Account<'info, UserReputation>,
    
    /// CHECK: Authority can be any program that calls this via CPI
    pub authority: Signer<'info>,
}

impl<'info> UpdateReputation<'info> {
    pub fn update_reputation(
        &mut self,
        user: Pubkey,
        event_type: ReputationEvent,
        custom_amount: Option<i64>,
    ) -> Result<()> {
        // Determine the reputation change amount
        let delta = match event_type {
            ReputationEvent::Custom(_) => {
                custom_amount.ok_or(ReputationError::InvalidEventType)?
            },
            _ => event_type.get_score_delta(),
        };
        
        // Validate the change won't exceed bounds
        if delta > 0 {
            require!(
                self.user_reputation.can_increase_reputation(delta),
                ReputationError::ReputationOverflow
            );
        } else if delta < 0 {
            require!(
                self.user_reputation.can_decrease_reputation(delta),
                ReputationError::ReputationUnderflow
            );
        } else {
            return Err(ReputationError::ZeroReputationChange.into());
        }
        
        // Apply the reputation change
        let old_score = self.user_reputation.reputation_score;
        let old_tier = self.user_reputation.tier;
        
        self.user_reputation.reputation_score = if delta > 0 {
            self.user_reputation.reputation_score.saturating_add(delta)
        } else {
            self.user_reputation.reputation_score.saturating_sub(delta.abs())
        };
        
        // Ensure bounds are respected
        self.user_reputation.reputation_score = self.user_reputation.reputation_score
            .max(MIN_REPUTATION_SCORE)
            .min(MAX_REPUTATION_SCORE);
        
        // Update tier based on new score
        self.user_reputation.update_tier(&self.reputation_config);
        
        // Update metadata
        let clock = Clock::get()?;
        self.user_reputation.events_count = self.user_reputation.events_count
            .checked_add(1)
            .ok_or(ReputationError::ArithmeticOverflow)?;
        self.user_reputation.last_event_ts = clock.unix_timestamp;
        
        emit!(ReputationUpdated {
            user,
            authority: self.authority.key(),
            event_type,
            old_score,
            new_score: self.user_reputation.reputation_score,
            old_tier,
            new_tier: self.user_reputation.tier,
            timestamp: clock.unix_timestamp,
        });
        
        Ok(())
    }
}

#[event]
pub struct ReputationUpdated {
    pub user: Pubkey,
    pub authority: Pubkey,
    pub event_type: ReputationEvent,
    pub old_score: i64,
    pub new_score: i64,
    pub old_tier: ReputationTier,
    pub new_tier: ReputationTier,
    pub timestamp: i64,
}
