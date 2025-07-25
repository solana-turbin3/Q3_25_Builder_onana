use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};

use crate::{state::Bet, error::DiceError};

pub const REFUND_TIMEOUT_SLOTS: u64 = 150; // ~1 minute at 400ms per slot

#[derive(Accounts)]
#[instruction(bumps: RefundBumps)]
pub struct Refund<'info> {
    #[account(
        mut,
        has_one = player,
        seeds = [b"bet", bet.vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump = bumps.bet,
        close = player,
    )]
    pub bet: Account<'info, Bet>,
    
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump = bumps.vault,
    )]
    pub vault: Account<'info, Vault>,
    
    /// CHECK: The house account that owns the vault
    pub house: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub player: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RefundBumps {
    pub bet: u8,
    pub vault: u8,
}

impl<'info> Refund<'info> {
    pub fn refund_bet(&mut self, bumps: &RefundBumps) -> Result<()> {
        let current_slot = Clock::get()?.slot;
        
        // Check if enough time has passed since the bet was placed
        // This prevents immediate refunds and gives time for normal resolution
        require!(
            current_slot >= self.bet.slot.saturating_add(REFUND_TIMEOUT_SLOTS),
            DiceError::TimeoutNotReached
        );
        
        // Check that the bet hasn't been resolved yet (roll should be 0 for unresolved bets)
        require!(
            self.bet.roll == 0,
            DiceError::BetAlreadySettled
        );
        
        // Transfer the bet amount back to the player from the vault
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.player.to_account_info(),
        };
        
        // Create the signer seeds for the vault PDA
        let seeds = [
            b"vault",
            &self.house.key().to_bytes()[..],
            &[bumps.vault],
        ];
        let signer_seeds = &[&seeds[..]][..];
        
        // Create CPI context with signer (vault PDA can sign)
        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        
        // Execute the refund transfer
        transfer(ctx, self.bet.amount)?;
        
        msg!(
            "Refunded {} lamports to player {}",
            self.bet.amount,
            self.player.key()
        );
        
        Ok(())
    }
}
