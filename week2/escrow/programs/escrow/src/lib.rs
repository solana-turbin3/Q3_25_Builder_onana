#![allow(unexpected_cfgs)]
#![allow(deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8KiiqftKSSHTE1zF1XmtcWf1zvppaFf9C7z4mmA46p3H");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        instructions::make::make(ctx, seed, deposit, receive)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        instructions::refund::refund(ctx)
    }

    pub fn take(ctx: Context<Take>, seed: u64) -> Result<()> {
        instructions::take::take(ctx)
    }
}