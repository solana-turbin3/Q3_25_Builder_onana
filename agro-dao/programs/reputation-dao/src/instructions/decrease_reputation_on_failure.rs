use anchor_lang::prelude::*;
use crate::state::{ReputationConfig, UserReputation, ReputationTier};
use crate::constants::*;
use crate::error::*;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct DecreaseReputationOnFailure<'info> {
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
    
    /// CHECK: Authority calling this instruction (usually research management program)
    pub authority: Signer<'info>,
}

impl<'info> DecreaseReputationOnFailure<'info> {
    pub fn decrease_reputation_on_failure(
        &mut self,
        user: Pubkey,
        failure_type: FailureType,
        custom_penalty: Option<i64>,
    ) -> Result<()> {
        // Determine penalty amount based on failure type
        let penalty = match failure_type {
            FailureType::MissedDeadline => MILESTONE_FAILURE_PENALTY,
            FailureType::ProjectAbandonment => PROJECT_ABANDONMENT_PENALTY,
            FailureType::DisputeResolution => DISPUTE_RESOLUTION_PENALTY,
            FailureType::Custom => {
                custom_penalty
                    .filter(|&p| p < 0) // Ensure it's negative
                    .ok_or(ReputationError::InvalidEventType)?
            }
        };
        
        // Check if user is already at minimum reputation
        if self.user_reputation.reputation_score <= MIN_REPUTATION_SCORE {
            return Err(ReputationError::AlreadyAtMinimum.into());
        }
        
        // Validate the decrease won't go below minimum
        require!(
            self.user_reputation.can_decrease_reputation(penalty),
            ReputationError::ReputationUnderflow
        );
        
        let old_score = self.user_reputation.reputation_score;
        let old_tier = self.user_reputation.tier;
        
        // Apply the penalty
        self.user_reputation.reputation_score = self.user_reputation.reputation_score
            .saturating_sub(penalty.abs())
            .max(MIN_REPUTATION_SCORE);
        
        // Update tier based on new score
        self.user_reputation.update_tier(&self.reputation_config);
        
        // Update metadata
        let clock = Clock::get()?;
        self.user_reputation.events_count = self.user_reputation.events_count
            .checked_add(1)
            .ok_or(ReputationError::ArithmeticOverflow)?;
        self.user_reputation.last_event_ts = clock.unix_timestamp;
        
        emit!(ReputationDecreased {
            user,
            authority: self.authority.key(),
            failure_type,
            penalty_amount: penalty,
            old_score,
            new_score: self.user_reputation.reputation_score,
            old_tier,
            new_tier: self.user_reputation.tier,
            timestamp: clock.unix_timestamp,
        });
        
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum FailureType {
    MissedDeadline,
    ProjectAbandonment,
    DisputeResolution,
    Custom,
}

#[event]
pub struct ReputationDecreased {
    pub user: Pubkey,
    pub authority: Pubkey,
    pub failure_type: FailureType,
    pub penalty_amount: i64,
    pub old_score: i64,
    pub new_score: i64,
    pub old_tier: ReputationTier,
    pub new_tier: ReputationTier,
    pub timestamp: i64,
}
