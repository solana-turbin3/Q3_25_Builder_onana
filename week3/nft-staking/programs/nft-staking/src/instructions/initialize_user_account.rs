use anchor_lang::prelude::*;

use crate::{
    error::CustomError,
    state::*,
};

#[derive(Accounts)]
pub struct InitializeUserAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + UserAccount::INIT_SPACE,
        seeds = [b"user", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeUserAccount>) -> Result<()> {
    ctx.accounts.user_account.set_inner(UserAccount {
        points: 0,
        amount_staked: 0,
        bump: ctx.bumps.user_account,
    });
    Ok(())
}