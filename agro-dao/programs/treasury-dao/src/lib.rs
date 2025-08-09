pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9B8K3XrYzqVvE2mhf4q1VX7bNzRtJ5AqG3pL6dFm8sWr");

#[program]
pub mod treasury_dao {
    use super::*;

    // Treasury Management
    pub fn initialize_treasury(
        ctx: Context<InitializeTreasury>,
        authority: Pubkey,
        fee_rate_bps: u16,
        min_reserve_ratio_bps: u16,
    ) -> Result<()> {
        ctx.accounts.initialize_treasury(authority, fee_rate_bps, min_reserve_ratio_bps, ctx.bumps.treasury_config)
    }

    pub fn add_supported_token(
        ctx: Context<AddSupportedToken>,
        token_mint: Pubkey,
    ) -> Result<()> {
        ctx.accounts.add_supported_token(token_mint, ctx.bumps.token_vault)
    }

    // Stake & Deposit Management
    pub fn deposit_stake_tokens(
        ctx: Context<DepositStakeTokens>,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.deposit_stake_tokens(amount, &ctx.bumps)
    }

    // Proposal Funding
    pub fn fund_proposal(
        ctx: Context<FundProposal>,
        proposal_id: String,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.fund_proposal(proposal_id, amount, &ctx.bumps)
    }

    // Fund Distribution
    pub fn distribute_proposal_funds(
        ctx: Context<DistributeProposalFunds>,
        milestone_index: u8,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.distribute_proposal_funds(milestone_index, amount)
    }

    // Emergency Controls
    pub fn emergency_pause(ctx: Context<EmergencyPause>) -> Result<()> {
        ctx.accounts.emergency_pause()
    }

    pub fn emergency_unpause(ctx: Context<EmergencyUnpause>) -> Result<()> {
        ctx.accounts.emergency_unpause()
    }
}
