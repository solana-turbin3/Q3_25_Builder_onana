use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;
use crate::constants::*;
use crate::error::TreasuryError;

#[derive(Accounts)]
pub struct DistributeProposalFunds<'info> {
    #[account(
        seeds = [TREASURY_CONFIG_SEED],
        bump = treasury_config.bump,
        has_one = authority @ TreasuryError::Unauthorized,
        constraint = !treasury_config.emergency_pause @ TreasuryError::TreasuryPaused
    )]
    pub treasury_config: Account<'info, TreasuryConfig>,

    #[account(
        mut,
        seeds = [PROPOSAL_FUNDING_SEED, proposal_funding.proposal_id.as_bytes()],
        bump = proposal_funding.bump,
        constraint = proposal_funding.status == ProposalFundingStatus::Approved @ TreasuryError::ProposalNotApproved
    )]
    pub proposal_funding: Account<'info, ProposalFunding>,

    // Cross-program verification: check research proposal and milestone status
    #[account(
        seeds = [b"proposal", research_proposal.researcher.as_ref(), &research_proposal.proposal_id.to_le_bytes()],
        bump,
        seeds::program = research_dao_program.key(),
        constraint = research_proposal.key() == proposal_funding.proposal_pda @ TreasuryError::ProposalNotFound
    )]
    pub research_proposal: Account<'info, ResearchProposal>,

    // Verify recipient is the researcher who owns the proposal
    #[account(
        seeds = [b"researcher", research_proposal.researcher.as_ref()],
        bump,
        seeds::program = research_dao_program.key(),
        constraint = researcher_profile.is_verified @ TreasuryError::ResearcherNotVerified,
        constraint = researcher_profile.researcher == research_proposal.researcher @ TreasuryError::Unauthorized
    )]
    pub researcher_profile: Account<'info, ResearcherProfile>,

    /// CHECK: This is the researcher account that will receive the funds
    #[account(
        constraint = recipient.key() == researcher_profile.researcher @ TreasuryError::Unauthorized
    )]
    pub recipient: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [TOKEN_VAULT_SEED, distribution_token_mint.key().as_ref()],
        bump = token_vault.bump,
    )]
    pub token_vault: Account<'info, TokenVault>,

    #[account(
        mut,
        seeds = [b"vault_ata", distribution_token_mint.key().as_ref()],
        bump,
        token::mint = distribution_token_mint,
        token::authority = token_vault,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub distribution_token_mint: Account<'info, anchor_spl::token::Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = distribution_token_mint,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: This is the research DAO program for CPI
    pub research_dao_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> DistributeProposalFunds<'info> {
    pub fn distribute_proposal_funds(&mut self, milestone_index: u8, amount: u64) -> Result<()> {
        // Validate inputs
        require!(amount > 0, TreasuryError::InvalidAmount);
        require!(
            amount <= MAX_DISTRIBUTION_AMOUNT,
            TreasuryError::DistributionAmountTooLarge
        );

        // Check if milestone exists and is completed
        let milestones = &self.research_proposal.milestones;
        require!(
            (milestone_index as usize) < milestones.len(),
            TreasuryError::InvalidMilestone
        );

        let milestone = &milestones[milestone_index as usize];
        require!(
            milestone.status == MilestoneStatus::Completed || milestone.status == MilestoneStatus::Verified,
            TreasuryError::InvalidMilestone
        );

        // Check if milestone has already been distributed
        let already_distributed = self.proposal_funding.milestone_distributions
            .iter()
            .any(|d| d.milestone_index == milestone_index);
        require!(!already_distributed, TreasuryError::MilestoneAlreadyDistributed);

        // Validate distribution amount doesn't exceed committed funds
        let remaining_funds = self.proposal_funding.total_committed
            .checked_sub(self.proposal_funding.total_distributed)
            .ok_or(TreasuryError::ArithmeticUnderflow)?;
        require!(amount <= remaining_funds, TreasuryError::InsufficientBalance);

        // Check treasury has sufficient balance for this token
        require!(
            self.token_vault.available_balance >= amount,
            TreasuryError::InsufficientBalance
        );

        // Calculate milestone funding percentage and validate
        let milestone_percentage = milestone.funding_percentage as u64;
        let expected_amount = self.proposal_funding.total_committed
            .checked_mul(milestone_percentage)
            .ok_or(TreasuryError::ArithmeticOverflow)?
            .checked_div(100)
            .ok_or(TreasuryError::ArithmeticOverflow)?;
        
        // Allow some flexibility in distribution amount (±5%)
        let tolerance = expected_amount.checked_div(20).unwrap_or(0); // 5%
        require!(
            amount >= expected_amount.saturating_sub(tolerance) && 
            amount <= expected_amount.saturating_add(tolerance),
            TreasuryError::InvalidAmount
        );

        let clock = Clock::get()?;

        // Transfer tokens from vault to recipient
        let transfer_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.vault_token_account.to_account_info(),
                to: self.recipient_token_account.to_account_info(),
                authority: self.token_vault.to_account_info(),
            },
        );

        let token_mint_key = self.distribution_token_mint.key();
        let vault_seeds = &[
            TOKEN_VAULT_SEED,
            token_mint_key.as_ref(),
            &[self.token_vault.bump],
        ];
        let vault_signer = &[&vault_seeds[..]];

        token::transfer(transfer_ctx.with_signer(vault_signer), amount)?;

        // Update token vault balances
        self.token_vault.available_balance = self.token_vault.available_balance
            .checked_sub(amount)
            .ok_or(TreasuryError::ArithmeticUnderflow)?;

        // Update proposal funding
        self.proposal_funding.total_distributed = self.proposal_funding.total_distributed
            .checked_add(amount)
            .ok_or(TreasuryError::ArithmeticOverflow)?;

        self.proposal_funding.milestone_distributions.push(MilestoneDistribution {
            milestone_index,
            amount_distributed: amount,
            token_mint: self.distribution_token_mint.key(),
            recipient: self.recipient.key(),
            distributed_at: clock.unix_timestamp,
        });

        self.proposal_funding.last_updated = clock.unix_timestamp;

        // Mark proposal as distributed if all funds have been distributed
        if self.proposal_funding.total_distributed >= self.proposal_funding.total_committed {
            self.proposal_funding.status = ProposalFundingStatus::Distributed;
        }

        emit!(FundsDistributed {
            proposal_id: self.proposal_funding.proposal_id.clone(),
            recipient: self.recipient.key(),
            milestone_index,
            amount,
            token_mint: self.distribution_token_mint.key(),
            total_distributed: self.proposal_funding.total_distributed,
            research_proposal: self.research_proposal.key(),
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct FundsDistributed {
    pub proposal_id: String,
    pub recipient: Pubkey,
    pub milestone_index: u8,
    pub amount: u64,
    pub token_mint: Pubkey,
    pub total_distributed: u64,
    pub research_proposal: Pubkey,
    pub timestamp: i64,
}
