use anchor_lang::prelude::*;
use crate::state::GovernanceConfig;
use crate::constants::*;
use crate::error::GovernanceError;

#[derive(Accounts)]
pub struct UpdateGovernanceConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [GOVERNANCE_CONFIG_SEED],
        bump = governance_config.bump,
        constraint = governance_config.governance_authority == authority.key() @ GovernanceError::UnauthorizedGovernanceUpdate
    )]
    pub governance_config: Account<'info, GovernanceConfig>,
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

        // Validate parameter ranges before updating
        if let Some(quorum) = new_quorum_threshold_bps {
            require!(quorum >= 100 && quorum <= 10000, GovernanceError::InvalidParameter);
            self.governance_config.quorum_threshold_bps = quorum;
        }

        if let Some(approval) = new_approval_threshold_bps {
            require!(approval >= 5000 && approval <= 10000, GovernanceError::InvalidParameter);
            self.governance_config.approval_threshold_bps = approval;
        }

        if let Some(param_threshold) = new_parameter_change_threshold_bps {
            require!(param_threshold >= 6000 && param_threshold <= 10000, GovernanceError::InvalidParameter);
            self.governance_config.parameter_change_threshold_bps = param_threshold;
        }

        if let Some(min_propose) = new_min_agro_to_propose {
            require!(min_propose >= 1000 * 10_u64.pow(6), GovernanceError::InvalidParameter); // Min 1000 AGRO
            self.governance_config.min_agro_to_propose = min_propose;
        }

        if let Some(min_vote) = new_min_agro_to_vote {
            require!(min_vote >= 10 * 10_u64.pow(6), GovernanceError::InvalidParameter); // Min 10 AGRO
            self.governance_config.min_agro_to_vote = min_vote;
        }

        if let Some(max_rep_weight) = new_max_reputation_weight_bps {
            require!(max_rep_weight <= 5000, GovernanceError::InvalidParameter); // Max 50%
            self.governance_config.max_reputation_weight_bps = max_rep_weight;
        }

        if let Some(new_authority) = new_governance_authority {
            self.governance_config.governance_authority = new_authority;
        }

        if let Some(pause) = emergency_pause {
            self.governance_config.emergency_pause = pause;
        }

        self.governance_config.last_updated = clock.unix_timestamp;

        emit!(GovernanceConfigUpdatedEvent {
            authority: self.authority.key(),
            quorum_threshold_bps: self.governance_config.quorum_threshold_bps,
            approval_threshold_bps: self.governance_config.approval_threshold_bps,
            parameter_change_threshold_bps: self.governance_config.parameter_change_threshold_bps,
            min_agro_to_propose: self.governance_config.min_agro_to_propose,
            min_agro_to_vote: self.governance_config.min_agro_to_vote,
            max_reputation_weight_bps: self.governance_config.max_reputation_weight_bps,
            emergency_pause: self.governance_config.emergency_pause,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct GovernanceConfigUpdatedEvent {
    pub authority: Pubkey,
    pub quorum_threshold_bps: u16,
    pub approval_threshold_bps: u16,
    pub parameter_change_threshold_bps: u16,
    pub min_agro_to_propose: u64,
    pub min_agro_to_vote: u64,
    pub max_reputation_weight_bps: u16,
    pub emergency_pause: bool,
    pub timestamp: i64,
}
