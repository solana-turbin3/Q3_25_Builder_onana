use anchor_lang::prelude::*;
use crate::state::{ReputationConfig, UserReputation, ReputationTier};
use crate::constants::*;
use crate::error::*;

#[derive(Accounts)]
pub struct InitializeReputationConfig<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ReputationConfig::INIT_SPACE,
        seeds = [REPUTATION_CONFIG_SEED],
        bump
    )]
    pub reputation_config: Account<'info, ReputationConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeReputationConfig<'info> {
    pub fn initialize_reputation_config(
        &mut self,
        bump: u8,
        tier_threshold_bronze: Option<u64>,
        tier_threshold_silver: Option<u64>,
        tier_threshold_gold: Option<u64>,
        tier_threshold_platinum: Option<u64>,
        tier_threshold_diamond: Option<u64>,
    ) -> Result<()> {
        let clock = Clock::get()?;
        
        // Use provided thresholds or defaults
        let bronze = tier_threshold_bronze.unwrap_or(DEFAULT_BRONZE_THRESHOLD);
        let silver = tier_threshold_silver.unwrap_or(DEFAULT_SILVER_THRESHOLD);
        let gold = tier_threshold_gold.unwrap_or(DEFAULT_GOLD_THRESHOLD);
        let platinum = tier_threshold_platinum.unwrap_or(DEFAULT_PLATINUM_THRESHOLD);
        let diamond = tier_threshold_diamond.unwrap_or(DEFAULT_DIAMOND_THRESHOLD);
        
        // Validate tier thresholds are in ascending order
        require!(
            bronze < silver && silver < gold && gold < platinum && platinum < diamond,
            ReputationError::InvalidTierThreshold
        );
        
        self.reputation_config.set_inner(ReputationConfig {
            bump,
            authority: self.authority.key(),
            tier_threshold_bronze: bronze,
            tier_threshold_silver: silver,
            tier_threshold_gold: gold,
            tier_threshold_platinum: platinum,
            tier_threshold_diamond: diamond,
            is_active: true,
            created_at: clock.unix_timestamp,
            updated_at: clock.unix_timestamp,
        });
        
        emit!(ReputationConfigInitialized {
            authority: self.authority.key(),
            tier_thresholds: [bronze, silver, gold, platinum, diamond],
            timestamp: clock.unix_timestamp,
        });
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUserReputation<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + UserReputation::INIT_SPACE,
        seeds = [USER_REPUTATION_SEED, user.key().as_ref()],
        bump
    )]
    pub user_reputation: Account<'info, UserReputation>,
    
    /// CHECK: The user for whom reputation is being initialized
    pub user: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUserReputation<'info> {
    pub fn initialize_user_reputation(&mut self, bump: u8) -> Result<()> {
        let clock = Clock::get()?;
        
        self.user_reputation.set_inner(UserReputation {
            bump,
            user: self.user.key(),
            reputation_score: 0,
            events_count: 0,
            last_event_ts: clock.unix_timestamp,
            created_at: clock.unix_timestamp,
            tier: ReputationTier::None,
        });
        
        emit!(UserReputationInitialized {
            user: self.user.key(),
            timestamp: clock.unix_timestamp,
        });
        
        Ok(())
    }
}

#[event]
pub struct ReputationConfigInitialized {
    pub authority: Pubkey,
    pub tier_thresholds: [u64; 5], // bronze, silver, gold, platinum, diamond
    pub timestamp: i64,
}

#[event]
pub struct UserReputationInitialized {
    pub user: Pubkey,
    pub timestamp: i64,
}