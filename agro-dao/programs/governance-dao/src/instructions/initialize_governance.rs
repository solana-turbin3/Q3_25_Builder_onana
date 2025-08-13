use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::error::*;

#[derive(Accounts)]
pub struct InitializeGovernance<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + GovernanceConfig::INIT_SPACE,
        seeds = [GOVERNANCE_SEED],
        bump
    )]
    pub governance_config: Account<'info, GovernanceConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: AGRO token mint address, validated in instruction
    pub agro_token_mint: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeGovernance<'info> {
    pub fn initialize_governance(
        &mut self,
        bump: u8,
        agro_token_mint: Pubkey,
        governance_authority: Pubkey,
        quorum_threshold_bps: u16,
        approval_threshold_bps: u16,
        parameter_change_threshold_bps: u16,
        min_agro_to_propose: u64,
        min_agro_to_vote: u64,
        max_reputation_weight_bps: u16,
    ) -> Result<()> {
        // Validate thresholds
        require!(
            quorum_threshold_bps <= BASIS_POINTS_MAX,
            GovernanceError::InvalidThreshold
        );
        require!(
            approval_threshold_bps <= BASIS_POINTS_MAX,
            GovernanceError::InvalidThreshold
        );
        require!(
            parameter_change_threshold_bps <= BASIS_POINTS_MAX,
            GovernanceError::InvalidThreshold
        );
        require!(
            max_reputation_weight_bps <= BASIS_POINTS_MAX,
            GovernanceError::InvalidThreshold
        );

        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        self.governance_config.set_inner(GovernanceConfig {
            bump,
            agro_token_mint,
            governance_authority,
            quorum_threshold_bps,
            approval_threshold_bps,
            parameter_change_threshold_bps,
            min_agro_to_propose,
            min_agro_to_vote,
            max_reputation_weight_bps,
            total_proposals: 0,
            emergency_pause: false,
            created_at: current_time,
            updated_at: current_time,
        });

        emit!(GovernanceInitialized {
            governance_config: self.governance_config.key(),
            agro_token_mint,
            governance_authority,
            initialized_at: current_time,
        });

        Ok(())
    }
}

#[event]
pub struct GovernanceInitialized {
    pub governance_config: Pubkey,
    pub agro_token_mint: Pubkey,
    pub governance_authority: Pubkey,
    pub initialized_at: i64,
}

