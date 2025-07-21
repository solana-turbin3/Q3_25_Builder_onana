use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};
use crate::state::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]

    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program

    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

     #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program

    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

     #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program

    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        has_one = mint_a,
        has_one = mint_b,
        has_one = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,

}

pub fn take(ctx: Context<Take>) -> Result<()> {
    let signer_seeds: [&[&[u8]]; 1] = [&[
        b"escrow",
        ctx.accounts.maker.to_account_info().key.as_ref(),
        &ctx.accounts.escrow.seed.to_le_bytes()[..],
        &[ctx.accounts.escrow.bump],
    ]];

    let transfer_accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        mint: ctx.accounts.mint_a.to_account_info(),
        to: ctx.accounts.taker_ata_a.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info()
    };

    let transfer_cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), transfer_accounts, &signer_seeds);

    transfer_checked(transfer_cpi_ctx, ctx.accounts.vault.amount, ctx.accounts.mint_a.decimals)?;

    let transfer_accounts = TransferChecked {
        from: ctx.accounts.taker_ata_b.to_account_info(),
        mint: ctx.accounts.mint_b.to_account_info(),
        to: ctx.accounts.maker_ata_b.to_account_info(),
        authority: ctx.accounts.taker.to_account_info()
    };

    let transfer_cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_accounts);

    transfer_checked(transfer_cpi_ctx, ctx.accounts.escrow.receive, ctx.accounts.mint_b.decimals)?;

    let close_accounts = CloseAccount {
        account: ctx.accounts.vault.to_account_info(),
        destination: ctx.accounts.taker.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info()
    };

    let close_cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), close_accounts, &signer_seeds);

    close_account(close_cpi_ctx)?;

    Ok(())
}