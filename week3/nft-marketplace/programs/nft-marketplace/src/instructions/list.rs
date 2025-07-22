use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token::{transfer_checked, Token, TransferChecked},
    token_interface::{Mint, TokenAccount},
};

use crate::{
    error::MarketplaceError,
    state::*,
};

#[derive(Accounts)]
pub struct ListNft<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    /// CHECK: This is not dangerous because we don't read or write from this account
    // pub admin: AccountInfo<'info>,
    
    pub nft: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = seller,
        space = 8 + Listing::INIT_SPACE,
        seeds = [
            b"listing",
            marketplace.key().as_ref(),
            seller.key().as_ref(),
            nft.key().as_ref(),
        ],
        bump,
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        init,
        payer = seller,
        associated_token::mint = nft,
        associated_token::authority = listing,
        associated_token::token_program = token_program,
    )]
    pub listing_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [b"marketplace", marketplace.admin.key().as_ref()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
       #[account(
        mut,
        associated_token::mint = nft,
        associated_token::authority = seller,
        associated_token::token_program = token_program,
        constraint = seller_token_account.owner == seller.key()
    )]
    pub seller_token_account: InterfaceAccount<'info, TokenAccount>,

    pub collection_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            nft.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint= metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint= metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [
            b"master_edition",
            metadata_program.key().as_ref(),
            nft.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,
    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> ListNft<'info> {
    pub fn transfer(&mut self) -> Result<()> {
        // Transfer the NFT from the seller to the marketplace
        let cpi_accounts = TransferChecked {
            from: self.seller_token_account.to_account_info(),
            mint: self.nft.to_account_info(),
            to: self.listing_token_account.to_account_info(),
            authority: self.seller.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        transfer_checked(
            cpi_ctx,
            1, // Assuming 1 token is being transferred
            self.nft.decimals,
        )?;

        Ok(())
    }

    pub fn initialize_listing(
        &mut self,
        price: u64,
        bumps: &ListNftBumps,
    ) -> Result<()> {
        // Validate price is reasonable (greater than 0)
        require!(
            price > 0,
            MarketplaceError::InvalidPrice
        );

        // Initialize listing state
        self.listing.set_inner(Listing {
            seller: self.seller.key(),
            mint: self.nft.key(),
            price,
            bump: bumps.listing,
            is_active: true,
        });

        Ok(())
    }
}


