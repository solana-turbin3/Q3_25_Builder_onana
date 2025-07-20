use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    // the seed is used to identify the pool
    pub seed: u64,
    // the bump is used to identify the pool
    pub pool_bump: u8,
    // the mint of the x token
    pub mint_x: Pubkey,
    // the mint of the y token
    pub mint_y: Pubkey,
    // the authority of the pool
    pub authority : Option<Pubkey>,
    // the fee of the pool
    pub fee: u16,
    // the locked state of the pool
    pub locked: bool,
    // the bump of the lp mint
    pub lp_bump: u8,     
}