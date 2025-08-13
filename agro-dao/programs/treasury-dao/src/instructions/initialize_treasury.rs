

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use crate::state::*;
use crate::constants::*;
use crate::error::TreasuryError;

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(
        init,
        seeds = [TREASURY_CONFIG_SEED],
        bump,
        payer = initializer,
        space = 8 + TreasuryConfig::INIT_SPACE
    )]
    pub treasury_config: Account<'info, TreasuryConfig>,

    #[account(
        init,
        seeds = [AGRO_MINT_SEED],
        bump,
        payer = initializer,
        mint::decimals = 9,
        mint::authority = agro_mint,
    )]
    pub agro_mint: Account<'info, Mint>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitializeTreasury<'info> {
    pub fn initialize_treasury(
        &mut self,
        authority: Pubkey,
        fee_rate_bps: u16,
        min_reserve_ratio_bps: u16,
        bump: u8,
    ) -> Result<()> {
        // Validate parameters
        require!(
            fee_rate_bps <= MAX_FEE_RATE_BPS,
            TreasuryError::InvalidFeeRate
        );
        require!(
            min_reserve_ratio_bps >= MIN_RESERVE_RATIO_BPS && min_reserve_ratio_bps <= MAX_RESERVE_RATIO_BPS,
            TreasuryError::InvalidReserveRatio
        );

        let clock = Clock::get()?;

        self.treasury_config.set_inner(TreasuryConfig {
            bump,
            authority,
            agro_mint: self.agro_mint.key(),
            fee_rate_bps,
            min_reserve_ratio_bps,
            emergency_pause: false,
            emergency_pause_timestamp: 0,
            supported_tokens: Vec::new(),
            total_agro_minted: 0,
            total_fees_collected: 0,
            created_at: clock.unix_timestamp,
        });

        emit!(TreasuryInitialized {
            treasury_config: self.treasury_config.key(),
            authority,
            agro_mint: self.agro_mint.key(),
            fee_rate_bps,
            min_reserve_ratio_bps,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct TreasuryInitialized {
    pub treasury_config: Pubkey,
    pub authority: Pubkey,
    pub agro_mint: Pubkey,
    pub fee_rate_bps: u16,
    pub min_reserve_ratio_bps: u16,
    pub timestamp: i64,
}