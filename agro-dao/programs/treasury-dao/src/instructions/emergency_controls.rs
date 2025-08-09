use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::error::TreasuryError;

#[derive(Accounts)]
pub struct EmergencyPause<'info> {
    #[account(
        mut,
        seeds = [TREASURY_CONFIG_SEED],
        bump = treasury_config.bump,
        has_one = authority @ TreasuryError::Unauthorized
    )]
    pub treasury_config: Account<'info, TreasuryConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

impl<'info> EmergencyPause<'info> {
    pub fn emergency_pause(&mut self) -> Result<()> {
        require!(!self.treasury_config.emergency_pause, TreasuryError::TreasuryPaused);

        let clock = Clock::get()?;
        
        self.treasury_config.emergency_pause = true;
        self.treasury_config.emergency_pause_timestamp = clock.unix_timestamp;

        emit!(EmergencyPauseActivated {
            authority: self.authority.key(),
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct EmergencyUnpause<'info> {
    #[account(
        mut,
        seeds = [TREASURY_CONFIG_SEED],
        bump = treasury_config.bump,
        has_one = authority @ TreasuryError::Unauthorized,
        constraint = treasury_config.emergency_pause @ TreasuryError::TreasuryPaused
    )]
    pub treasury_config: Account<'info, TreasuryConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

impl<'info> EmergencyUnpause<'info> {
    pub fn emergency_unpause(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        
        // Check if minimum pause duration has passed (optional safety measure)
        let pause_duration = clock.unix_timestamp - self.treasury_config.emergency_pause_timestamp;
        require!(
            pause_duration >= 3600, // Minimum 1 hour pause
            TreasuryError::InvalidAmount
        );

        self.treasury_config.emergency_pause = false;
        self.treasury_config.emergency_pause_timestamp = 0;

        emit!(EmergencyPauseDeactivated {
            authority: self.authority.key(),
            pause_duration,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct EmergencyPauseActivated {
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct EmergencyPauseDeactivated {
    pub authority: Pubkey,
    pub pause_duration: i64,
    pub timestamp: i64,
}
