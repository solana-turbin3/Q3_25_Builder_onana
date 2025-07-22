use anchor_lang::prelude::*;
use crate::{error::MarketplaceError, state::Marketplace};

#[derive(Accounts)]
pub struct InitializeMarketplace <'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + Marketplace::INIT_SPACE,
        seeds = [b"marketplace", admin.key().as_ref()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,
    pub system_program: Program<'info, System>,


}

impl <'info> InitializeMarketplace <'info> {
    pub fn initialize_marketplace(&mut self, fee_percent: u8, bumps: InitializeMarketplaceBumps) -> Result<()>{
            // Validate fee percentage is reasonable (0-100%)
        require!(
            fee_percent <= 100,
            MarketplaceError::InvalidFeePercentage
        );

        // Initialize marketplace state with provided parameters
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee_percent,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury, // Fixed: should be treasury bump, not marketplace bump
        });

        Ok(())
    }
}
