use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, transfer, transfer_checked, Mint, MintTo, Token, TokenAccount, Transfer, TransferChecked}};
use constant_product_curve::{ConstantProduct, CurveError, LiquidityPair};

use crate::{state::Config, error::AmmError};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
   
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"pool", pool.seed.to_le_bytes().as_ref()],
        bump = pool.config_bump,
    )]
    pub pool: Account<'info, Config>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = pool
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = pool
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub user_x: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user
    )]
    pub user_y: Account<'info, TokenAccount>,


    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info>Swap<'info>{
    pub fn swap(&mut self, is_x:bool, amount: u64, min:u64) -> Result<()> {
        require!(self.pool.locked == false, AmmError::PoolLocked);
        require!(amount > 0, AmmError::InvalidAmount);

        let mut curve = ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.vault_x.amount,
            self.pool.fee,
            None,
        ).map_err(AmmError::from)?;

        let p : LiquidityPair = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        let res = curve.swap(
            p,
            amount,
            min,
        ).map_err(AmmError::from)?;

        require!(res.deposit != 0, AmmError::InvalidAmount);
        require!(res.withdraw != 0, AmmError::InvalidAmount);

        // deposit tokens 
        self.deposit_tokens(is_x, res.deposit)?;
        // withdraw tokens  
        self.withdraw_tokens(is_x, res.withdraw)?;


        Ok(())
    }
    pub fn deposit_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {
    let (from, to) = match is_x {
        true => (self.user_x.to_account_info(), self.vault_x.to_account_info()),
        false => (self.user_y.to_account_info(), self.vault_y.to_account_info()),
    };

    let cpi_program: AccountInfo<'_> = self.token_program.to_account_info();
    let cpi_accounts = Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: self.user.to_account_info(),
    };

    let binding = self.pool.seed.to_le_bytes();
    let seeds: [&[u8]; 3] = [
        b"pool",
        binding.as_ref(),
        &[self.pool.pool_bump],
    ];
    let signer_seeds: &[&[u8]] = &seeds;

    let signer_seeds_arr: &[&[&[u8]]] = &[&seeds];

    let cpi_ctx = CpiContext::new_with_signer(
        cpi_program,
        cpi_accounts,
        signer_seeds_arr,
    );

    transfer(cpi_ctx, amount)?;
    Ok(())
}
    pub fn withdraw_tokens(&mut self, is_x: bool, amount: u64) -> Result<()>{

        let (from, to, mint, decimals) = match is_x {
            true => (
                self.vault_x.to_account_info(),
                self.user_x.to_account_info(),
                self.mint_x.to_account_info(),
                self.mint_x.decimals,
            ),
            false => (
                self.vault_y.to_account_info(),
                self.user_y.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals,
            ),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from,
            to,
            mint,
            authority: self.pool.to_account_info(),
        };

          let signer_seeds: &[&[&[u8]]] = &[&[
            b"pool",
            &self.pool.seed.to_le_bytes(),
            &[self.pool.pool_bump],
        ]];
        let cpi_context = CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            signer_seeds,
        );
        transfer_checked(cpi_context, amount, decimals)
            
}


}
