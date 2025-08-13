use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::error::*;

#[derive(Accounts)]
pub struct UpdateGovernanceConfig<'info> {
    #[account(
        mut,
        seeds = [GOVERNANCE_SEED],
        bump = governance_config.bump,
        constraint = governance_config.governance_authority == authority.key() @ GovernanceError::Unauthorized
    )]
    pub governance_config: Account<'info, GovernanceConfig>,

    pub authority: Signer<'info>,
}

impl<'info> UpdateGovernanceConfig<'info> {
    pub fn update_governance_config(
        &mut self,
        new_quorum_threshold_bps: Option<u16>,
        new_approval_threshold_bps: Option<u16>,
        new_parameter_change_threshold_bps: Option<u16>,
        new_min_agro_to_propose: Option<u64>,
        new_min_agro_to_vote: Option<u64>,
        new_max_reputation_weight_bps: Option<u16>,
        new_governance_authority: Option<Pubkey>,
        emergency_pause: Option<bool>,
    ) -> Result<()> {
        let clock = Clock::get()?;

        // Validate and update thresholds if provided
        if let Some(threshold) = new_quorum_threshold_bps {
            require!(threshold <= BASIS_POINTS_MAX, GovernanceError::InvalidThreshold);
            self.governance_config.quorum_threshold_bps = threshold;
        }

        if let Some(threshold) = new_approval_threshold_bps {
            require!(threshold <= BASIS_POINTS_MAX, GovernanceError::InvalidThreshold);
            self.governance_config.approval_threshold_bps = threshold;
        }

        if let Some(threshold) = new_parameter_change_threshold_bps {
            require!(threshold <= BASIS_POINTS_MAX, GovernanceError::InvalidThreshold);
            self.governance_config.parameter_change_threshold_bps = threshold;
        }

        if let Some(threshold) = new_max_reputation_weight_bps {
            require!(threshold <= BASIS_POINTS_MAX, GovernanceError::InvalidThreshold);
            self.governance_config.max_reputation_weight_bps = threshold;
        }

        // Update other parameters
        if let Some(min_agro) = new_min_agro_to_propose {
            self.governance_config.min_agro_to_propose = min_agro;
        }

        if let Some(min_agro) = new_min_agro_to_vote {
            self.governance_config.min_agro_to_vote = min_agro;
        }

        if let Some(new_authority) = new_governance_authority {
            self.governance_config.governance_authority = new_authority;
        }

        if let Some(pause) = emergency_pause {
            self.governance_config.emergency_pause = pause;
        }

        self.governance_config.updated_at = clock.unix_timestamp;

        emit!(GovernanceConfigUpdated {
            governance_config: self.governance_config.key(),
            updated_by: self.authority.key(),
            updated_at: clock.unix_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct GovernanceConfigUpdated {
    pub governance_config: Pubkey,
    pub updated_by: Pubkey,
    pub updated_at: i64,
}

