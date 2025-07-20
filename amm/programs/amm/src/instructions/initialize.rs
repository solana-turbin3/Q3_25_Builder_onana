use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
// import the pool state
use crate::state::Pool;

// initialize the pool
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        init,
        payer = admin,
        seeds = [b"lp", pool.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = pool,
    )]
    pub mint_lp: Account<'info, Mint>,
    #[account(
        init,
        payer = admin,
        seeds = [b"pool", seed.to_le_bytes().as_ref()],
        bump,
        space = 8 + Pool::INIT_SPACE,
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = mint_x,
        associated_token::authority = pool
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = mint_y,
        associated_token::authority = pool
    )]
    pub vault_y: Account<'info, TokenAccount>,


    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, seed: u64, fee: u16, authority: Option<Pubkey>, bumps: InitializeBumps) -> Result<()> {
        // set the pool state
        self.pool.set_inner(
            Pool { 
                seed, 
                authority, 
                mint_x:self.mint_x.key(), 
                mint_y: self.mint_y.key(), 
                fee, 
                locked: false, 
                pool_bump: bumps.pool, 
                lp_bump: bumps.mint_lp, 
            });

            // set the vaults
            self.vault_x.amount = 0;
            self.vault_y.amount = 0;

            // set the lp mint

            Ok(())
    }
}