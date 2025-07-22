use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Marketplace {
    // The admin that can update the marketplace
    pub admin: Pubkey,
    // The fee charged for each transaction in the marketplace
    pub fee_percent: u8,
    // pda bump seed for the marketplace account
    pub bump: u8,
    // pda bump seed for the treasury account
    pub treasury_bump: u8,
}