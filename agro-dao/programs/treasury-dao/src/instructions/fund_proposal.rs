use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};
use crate::state::*;
use crate::constants::*;
use crate::error::TreasuryError;

#[derive(Accounts)]
#[instruction(proposal_id: String)]
pub struct FundProposal<'info> {
    #[account(
        seeds = [TREASURY_CONFIG_SEED],
        bump = treasury_config.bump,
        constraint = !treasury_config.emergency_pause @ TreasuryError::TreasuryPaused
    )]
    pub treasury_config: Account<'info, TreasuryConfig>,

    #[account(
        init_if_needed,
        seeds = [PROPOSAL_FUNDING_SEED, proposal_id.as_bytes()],
        bump,
        payer = stakeholder,
        space = 8 + ProposalFunding::INIT_SPACE
    )]
    pub proposal_funding: Account<'info, ProposalFunding>,

    // Cross-program verification: ensure proposal exists in research DAO
    #[account(
        seeds = [b"proposal", research_proposal.researcher.as_ref(), &research_proposal.proposal_id.to_le_bytes()],
        bump,
        seeds::program = research_dao_program.key(),
        constraint = research_proposal.status == ProposalStatus::Approved @ TreasuryError::ProposalNotApproved
    )]
    pub research_proposal: Account<'info, ResearchProposal>,

    // Verify stakeholder is verified researcher
    #[account(
        seeds = [b"researcher", stakeholder.key().as_ref()],
        bump,
        seeds::program = research_dao_program.key(),
        constraint = stakeholder_profile.is_verified @ TreasuryError::ResearcherNotVerified
    )]
    pub stakeholder_profile: Account<'info, ResearcherProfile>,

    #[account(
        mut,
        seeds = [STAKE_ACCOUNT_SEED, stakeholder.key().as_ref()],
        bump = stake_account.bump,
        constraint = stake_account.owner == stakeholder.key() @ TreasuryError::Unauthorized
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        mut,
        seeds = [AGRO_MINT_SEED],
        bump,
    )]
    pub agro_mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = agro_mint,
        token::authority = stakeholder,
    )]
    pub stakeholder_agro_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub stakeholder: Signer<'info>,

    /// CHECK: This is the research DAO program for CPI
    pub research_dao_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> FundProposal<'info> {
    pub fn fund_proposal(&mut self, proposal_id: String, amount: u64, bumps: &FundProposalBumps) -> Result<()> {
        // Validate inputs
        require!(amount > 0, TreasuryError::InvalidAmount);
        require!(
            proposal_id.len() <= MAX_PROPOSAL_ID_LENGTH,
            TreasuryError::ProposalIdTooLong
        );

        // Check if stakeholder has enough AGRO tokens
        require!(
            self.stakeholder_agro_account.amount >= amount,
            TreasuryError::InsufficientBalance
        );

        let clock = Clock::get()?;

        // Burn AGRO tokens from stakeholder
        let burn_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.agro_mint.to_account_info(),
                from: self.stakeholder_agro_account.to_account_info(),
                authority: self.stakeholder.to_account_info(),
            },
        );
        token::burn(burn_ctx, amount)?;

        // Initialize or update proposal funding
        if self.proposal_funding.proposal_id.is_empty() {
            // Initialize new proposal funding
            self.proposal_funding.set_inner(ProposalFunding {
                bump: bumps.proposal_funding,
                proposal_id: proposal_id.clone(),
                research_program_id: self.research_dao_program.key(),
                proposal_pda: self.research_proposal.key(),
                total_committed: amount,
                total_distributed: 0,
                funding_sources: vec![FundingSource {
                    stakeholder: self.stakeholder.key(),
                    token_mint: self.agro_mint.key(), // AGRO tokens were burned
                    amount,
                    agro_burned: amount,
                    timestamp: clock.unix_timestamp,
                }],
                status: ProposalFundingStatus::Active,
                milestone_distributions: Vec::new(),
                created_at: clock.unix_timestamp,
                last_updated: clock.unix_timestamp,
            });
        } else {
            // Update existing proposal funding
            require!(
                self.proposal_funding.proposal_id == proposal_id,
                TreasuryError::ProposalNotFound
            );
            
            require!(
                self.proposal_funding.status == ProposalFundingStatus::Active,
                TreasuryError::ProposalAlreadyFunded
            );

            // Check if we can add more funding sources
            require!(
                self.proposal_funding.funding_sources.len() < MAX_FUNDING_SOURCES,
                TreasuryError::MaxFundingSourcesReached
            );

            self.proposal_funding.total_committed = self.proposal_funding.total_committed
                .checked_add(amount)
                .ok_or(TreasuryError::ArithmeticOverflow)?;

            self.proposal_funding.funding_sources.push(FundingSource {
                stakeholder: self.stakeholder.key(),
                token_mint: self.agro_mint.key(),
                amount,
                agro_burned: amount,
                timestamp: clock.unix_timestamp,
            });

            self.proposal_funding.last_updated = clock.unix_timestamp;
        }

        emit!(ProposalFunded {
            proposal_id,
            stakeholder: self.stakeholder.key(),
            amount,
            total_committed: self.proposal_funding.total_committed,
            research_proposal: self.research_proposal.key(),
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }
}

#[event]
pub struct ProposalFunded {
    pub proposal_id: String,
    pub stakeholder: Pubkey,
    pub amount: u64,
    pub total_committed: u64,
    pub research_proposal: Pubkey,
    pub timestamp: i64,
}
