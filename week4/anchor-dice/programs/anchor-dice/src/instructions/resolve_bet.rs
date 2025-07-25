use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};  
use solana_program::{
    ed25519_program, hash::hash, 
    sysvar::instructions::load_instructions_at_checked,

};

use crate::{state::Bet, error::DiceError};

pub const HOUSE_EDGE: u64 = 150; // 1.5% house edge

#[derive(Accounts)]
#[instruction(bumps: ResolveBetBumps)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,
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
        has_one = house,
    )]
    pub vault: Account<'info, Vault>,
    /// CHECK: Ensure the player has enough balance
    pub player: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        address = solana_program::sysvar::instructions::id(),
    )]
    pub instruction_sysvar: AccountInfo<'info>,
}


impl <'info> ResolveBet<'info> {
    pub fn verify_ed25519_signatures(&mut self, sig: &[u8]) -> Result<()> {
        let ix: Instruction = load_instructions_at_checked(0, &self.instruction_sysvar.to_account_info())?;
        require_keys_eq!(
            ix.program_id, 
            ed25519_program::id(), 
            DiceError::Ed25519Program
        );
        require_eq!(ix.accounts.len(), 1, DiceError::Ed25519Accounts);
        let signatures: Ed25519InstructionSignatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;
          require_eq!(
            signatures.len(),
            1,
            DiceError::Ed25519DataLength
        );
        let signature : &Ed25519InstructionSignature = &signatures[0];
        require!(signature.is_verifiable, DiceError::Ed25519Header);

        require_keys_eq!(
            signature.public_key.ok_or(DiceError::Ed25519Pubkey)?,
            self.house.key(),
            DiceError::Ed25519Pubkey
        );
        
        require!(
            &signature
            .signature
            .ok_or(DiceError::Ed25519Signature)?
            .eq(sig), 
            DiceError::Ed25519Signature
        );
        require!(
            signature
            .message.as_ref()
            .ok_or(DiceError::Ed25519Message)?,
            .eq(&self.bet.to_slice()),  
            DiceError::Ed25519Message
        );
        Ok(())
    }

    pub fn resolve_bet(&mut self, sig: &[u8], bumps: &ResolveBetBumps) -> Result<()> {
        let hash: [u8; 32] = hash(sig).to_bytes();

        let mut hash_16: [u8; 16] = [0; 16];
        hash_16.copy_from_slice(&hash[..16]);

        let lower: u128 = u128::from_le_bytes(hash_16);

        hash_16.copy_from_slice(&hash[16..32]);

        let upper: u128 = u128::from_le_bytes(hash_16);

        let roll: u8 = lower.wrapping_add(upper).wrapping_rem(100) as u8 + 1;

        if self.bet.roll < roll {
            let payout: u64 = (self.bet.amount as u128)
                .checked_mul(1000 - HOUSE_EDGE as u128)
                .ok_or(DiceError::Overflow)?
                .checked_div(self.bet.roll as u128 - 1)
                .ok_or(DiceError::Overflow)?
                .checked_div(100)
                .ok_or(DiceError::Overflow)? as u64;

            let accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            };
            let seeds = [
                b"vault",
                &self.house.key().to_bytes()[..],
                &[bumps.vault],
            ];
            let signerseeds = &[&seeds[..]][..];
            let ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                accounts,
                signerseeds,
            );
            transfer(ctx, payout)?;
        }

        Ok(())
    }
     
   
    
}