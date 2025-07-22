use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Token, TransferChecked},
    token_interface::{Mint, TokenAccount},
};

use crate::{
    error::MarketplaceError,
    state::{Listing, Marketplace},
};
#[derive(Accounts)]
pub struct PurchaseNft<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
     #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = nft,
        associated_token::authority = buyer
    )]
    pub buyer_token_account: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: This account is verified to match the seller in the listing during execution
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [
            b"listing",
            marketplace.key().as_ref(),
            seller.key().as_ref(),
            nft.key().as_ref(),
        ],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        seeds = [b"marketplace", marketplace.admin.key().as_ref()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        mut,
        associated_token::mint = nft,
        associated_token::authority = listing,
    )]
    pub listing_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,
    // The NFT mint account being purchased
    pub nft: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> PurchaseNft<'info> {
    /// Transfer the NFT from listing to buyer
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error from the transfer
    pub fn transfer_nft(&mut self) -> Result<()> {
        // Validate listing is active and seller matches
        require!(
            self.listing.is_active && self.listing.seller == self.seller.key(),
            MarketplaceError::ListingNotActive
        );

        // Create seeds for PDA signing
        let marketplace = self.marketplace.key();
        let seller = self.seller.key();
        let nft = self.nft.key();
        let listing_seeds: &[&[u8]] = &[
            b"listing",
            marketplace.as_ref(),
            seller.as_ref(),
            nft.as_ref(),
            &[self.listing.bump],
        ];
        let signer = &[listing_seeds];

        // Create CPI context with PDA signer
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            TransferChecked {
                from: self.listing_token_account.to_account_info(),
                mint: self.nft.to_account_info(),
                to: self.buyer_token_account.to_account_info(),
                authority: self.listing.to_account_info(),
            },
            signer,
        );

        // Transfer the NFT to the buyer
        transfer_checked(cpi_ctx, 1, self.nft.decimals)
    }

    /// Transfer SOL payment from buyer to seller and treasury
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error from the transfers
    pub fn transfer_sol(&mut self) -> Result<()> {
        // Calculate marketplace fee (percentage of listing price)
        let fee_lamports = (self.marketplace.fee_percent as u64)
            .checked_mul(self.listing.price)
            .ok_or(MarketplaceError::MathOverflow)?
            .checked_div(100)
            .ok_or(MarketplaceError::MathOverflow)?;

        // Calculate seller payment (listing price minus fees)
        let seller_lamports = self
            .listing
            .price
            .checked_sub(fee_lamports)
            .ok_or(MarketplaceError::MathOverflow)?;

        // Transfer fee to treasury
        let treasury_transfer_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.buyer.to_account_info(),
                to: self.treasury.to_account_info(),
            },
        );
        transfer(treasury_transfer_ctx, fee_lamports)?;

        // Transfer remaining payment to seller
        let seller_transfer_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.buyer.to_account_info(),
                to: self.seller.to_account_info(),
            },
        );
        transfer(seller_transfer_ctx, seller_lamports)?;

        Ok(())
    }

    /// Mark the listing as inactive after successful purchase
    /// 
    /// # Returns
    /// * `Result<()>` - Always succeeds
    pub fn delist_nft(&mut self) -> Result<()> {
        // Mark listing as inactive (though it will be closed anyway)
        self.listing.is_active = false;
        Ok(())
    }
}
