use crate::state::Bet;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};

#[derive(Accounts)]
#[instructions(seed: u128)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    // CHECK: Ensure the player has enough balance
    pub house: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        init,
        payer = player,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = Bet::LEN
    )]
    pub bet: Account<'info, Bet>,
    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn place_bet(&mut self, amount: u64, roll: u64, seed: u128, bumps: &PlaceBetBumps) -> Result<()> {
        self.bet.set_inner(Bet {
            amount,
            player: self.player.key(),
            slot: Clock::get()?.slot,
            seed,
            roll,
            bump: bumps.bet,
        });

        let ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.player.to_account_info(),
                to: self.vault.to_account_info(),
            },
        );
        transfer(ctx, amount)?;

        Ok(())
    }
}