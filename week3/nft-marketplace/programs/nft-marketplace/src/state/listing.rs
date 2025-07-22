use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    // The seller of the NFT
    pub seller: Pubkey,
    // The NFT being sold
    pub mint: Pubkey,
    // The price of the NFT
    pub price: u64,
    // The bump seed for the listing account
    pub bump: u8,
    // Whether the listing is active or not
    // set to false when the NFT is sold or the listing is canceled
    pub is_active: bool,
}