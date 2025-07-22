pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("Gek57L7kMSyPodYRwvCZri4yhCxbCNeEmk1d8i1mYpvV");

#[program]
pub mod nft_marketplace {
    use super::*;

    // Entry point function
    pub fn initialize_marketplace(ctx: Context<InitializeMarketplace>, fee_percent: u8) -> Result<()> {
        ctx.accounts.initialize_marketplace(fee_percent, ctx.bumps)
    }

    pub fn purchase_nft(ctx: Context<PurchaseNft>) -> Result<()> {
        // Transfer NFT to buyer
        ctx.accounts.transfer_nft()?;
        
        // Transfer SOL to seller (with marketplace fee)
        ctx.accounts.transfer_sol()?;
        
        // Mark listing as inactive
        ctx.accounts.delist_nft()?;
        
        Ok(())
    }
    pub fn list_nft(ctx: Context<ListNft>, price: u64) -> Result<()> {
        // Transfer NFT to listing account
        ctx.accounts.transfer()?;
        
        // Initialize the listing
        ctx.accounts.initialize_listing(price, &ctx.bumps)?;
        
        Ok(())
    }
    pub fn transfer_back_nft(ctx: Context<DelistNft>) -> Result<()> {
    ctx.accounts.transfer_back_nft()
}
}

