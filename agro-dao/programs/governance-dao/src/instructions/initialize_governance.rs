use anchor_lang::prelude::*;
use crate::state::GovernanceConfig;
use crate::constants::*;
use crate::error::GovernanceError;

#[derive(Accounts)]
pub struct InitializeGovernance<'info> {
    #[account(
        init,
        seeds = [GOVERNANCE_CONFIG_SEED],
        bump,
        payer = initializer,
        space = 8 + GovernanceConfig::INIT_SPACE
    )]
    pub governance_config: Account<'info, GovernanceConfig>,

    #[account(mut)]
    pub initializer: Signer<'info>,

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
        let clock = Clock::get()?;

        // Validate thresholds and params
        require!(
            quorum_threshold_bps >= MIN_QUORUM_THRESHOLD_BPS && quorum_threshold_bps <= MAX_QUORUM_THRESHOLD_BPS,
            GovernanceError::InvalidQuorumThreshold
        );
        require!(
            approval_threshold_bps >= MIN_APPROVAL_THRESHOLD_BPS && approval_threshold_bps <= MAX_APPROVAL_THRESHOLD_BPS,
            GovernanceError::InvalidApprovalThreshold
        );
        require!(
            parameter_change_threshold_bps >= MIN_APPROVAL_THRESHOLD_BPS && parameter_change_threshold_bps <= MAX_APPROVAL_THRESHOLD_BPS,
            GovernanceError::InvalidApprovalThreshold
        );
        require!(min_agro_to_propose >= MIN_AGRO_TO_PROPOSE, GovernanceError::InvalidProposalParameters);
        require!(min_agro_to_vote >= MIN_AGRO_TO_VOTE, GovernanceError::InvalidProposalParameters);
        require!(max_reputation_weight_bps <= MAX_REPUTATION_WEIGHT_BPS, GovernanceError::InvalidReputationWeight);

        self.governance_config.set_inner(GovernanceConfig {
            bump,
            governance_authority,
            agro_token_mint,
            treasury_program_id: Pubkey::default(),
            research_program_id: Pubkey::default(),
            min_agro_to_propose,
            min_agro_to_vote,
            quorum_threshold_bps,
            approval_threshold_bps,
            parameter_change_threshold_bps,
            max_reputation_weight_bps,
            emergency_pause: false,
            total_proposals_created: 0,
            created_at: clock.unix_timestamp,
            last_updated: clock.unix_timestamp,
        });

        emit!(GovernanceConfigInitializedEvent {
            governance_authority,
            agro_token_mint,
            quorum_threshold_bps,
            approval_threshold_bps,
            parameter_change_threshold_bps,
            min_agro_to_propose,
            min_agro_to_vote,
            max_reputation_weight_bps,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct GovernanceConfigInitializedEvent {
    pub governance_authority: Pubkey,
    pub agro_token_mint: Pubkey,
    pub quorum_threshold_bps: u16,
    pub approval_threshold_bps: u16,
    pub parameter_change_threshold_bps: u16,
    pub min_agro_to_propose: u64,
    pub min_agro_to_vote: u64,
    pub max_reputation_weight_bps: u16,
    pub timestamp: i64,
}
