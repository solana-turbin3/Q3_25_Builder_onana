use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use crate::state::{GovernanceConfig, Proposal, Vote, VoteChoice};
use crate::constants::*;
use crate::error::GovernanceError;

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(
        mut,
        seeds = [PROPOSAL_SEED, &proposal_id.to_le_bytes()],
        bump = proposal.bump,
        constraint = proposal.is_voting_active(Clock::get()?.unix_timestamp) @ GovernanceError::ProposalNotActive
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(
        init,
        payer = voter,
        space = 8 + Vote::INIT_SPACE,
        seeds = [VOTE_SEED, &proposal_id.to_le_bytes(), voter.key().as_ref()],
        bump
    )]
    pub vote: Account<'info, Vote>,

    #[account(
        constraint = voter_agro_account.mint == governance_config.agro_token_mint @ GovernanceError::InsufficientAgroToVote,
        constraint = voter_agro_account.owner == voter.key() @ GovernanceError::InsufficientAgroToVote,
        constraint = voter_agro_account.amount >= governance_config.min_agro_to_vote @ GovernanceError::InsufficientAgroToVote
    )]
    pub voter_agro_account: Account<'info, TokenAccount>,

    #[account(
        seeds = [GOVERNANCE_CONFIG_SEED],
        bump = governance_config.bump,
        constraint = !governance_config.emergency_pause @ GovernanceError::GovernancePaused
    )]
    pub governance_config: Account<'info, GovernanceConfig>,

    #[account(
        constraint = agro_mint.key() == governance_config.agro_token_mint
    )]
    pub agro_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> CastVote<'info> {
    pub fn cast_vote(
        &mut self,
        proposal_id: u64,
        vote_choice: VoteChoice,
        bump: u8,
    ) -> Result<()> {
        let clock = Clock::get()?;

        // Validate voting is currently active
        require!(
            clock.unix_timestamp >= self.proposal.voting_start_time,
            GovernanceError::VotingPeriodNotStarted
        );
        require!(
            clock.unix_timestamp <= self.proposal.voting_end_time,
            GovernanceError::VotingPeriodEnded
        );

        // For now, we'll use a simple reputation calculation
        // In a full implementation, this would involve CPI to a reputation program
        let reputation_balance = self.calculate_reputation_balance()?;

        // Calculate voting weight
        let total_weight = Vote::calculate_voting_weight(
            self.voter_agro_account.amount,
            reputation_balance,
            self.governance_config.max_reputation_weight_bps,
        )?;

        // Record the vote
        self.vote.set_inner(Vote {
            bump,
            proposal_id,
            voter: self.voter.key(),
            vote_choice: vote_choice.clone(),
            agro_weight: self.voter_agro_account.amount,
            reputation_weight: reputation_balance,
            total_weight,
            cast_at: clock.unix_timestamp,
            is_delegate_vote: false,
        });

        // Update proposal tallies
        match vote_choice {
            VoteChoice::For => {
                self.proposal.total_votes_for = self.proposal.total_votes_for
                    .checked_add(total_weight)
                    .ok_or(GovernanceError::ArithmeticOverflow)?;
            },
            VoteChoice::Against => {
                self.proposal.total_votes_against = self.proposal.total_votes_against
                    .checked_add(total_weight)
                    .ok_or(GovernanceError::ArithmeticOverflow)?;
            },
            VoteChoice::Abstain => {
                self.proposal.total_abstain_votes = self.proposal.total_abstain_votes
                    .checked_add(total_weight)
                    .ok_or(GovernanceError::ArithmeticOverflow)?;
            }
        }

        self.proposal.total_voters = self.proposal.total_voters
            .checked_add(1)
            .ok_or(GovernanceError::ArithmeticOverflow)?;

        // Check if quorum is reached
        let total_agro_supply = self.agro_mint.supply;
        let total_participation = self.proposal.total_votes_for
            .checked_add(self.proposal.total_votes_against)
            .ok_or(GovernanceError::ArithmeticOverflow)?
            .checked_add(self.proposal.total_abstain_votes)
            .ok_or(GovernanceError::ArithmeticOverflow)?;

        let quorum_threshold = total_agro_supply
            .checked_mul(self.governance_config.quorum_threshold_bps as u64)
            .ok_or(GovernanceError::ArithmeticOverflow)?
            .checked_div(10000)
            .ok_or(GovernanceError::ArithmeticOverflow)?;

        if total_participation >= quorum_threshold {
            self.proposal.quorum_reached = true;
        }

        emit!(VoteCastEvent {
            proposal_id,
            voter: self.voter.key(),
            vote_choice,
            agro_weight: self.voter_agro_account.amount,
            reputation_weight: reputation_balance,
            total_weight,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    // Simplified reputation calculation
    // In a full implementation, this would be a CPI call to reputation program
    fn calculate_reputation_balance(&self) -> Result<u64> {
        // For now, return a simple calculation based on AGRO balance
        // This should be replaced with actual reputation program integration
        let base_reputation = self.voter_agro_account.amount / 1000; // 1 reputation per 1000 AGRO
        Ok(std::cmp::min(base_reputation, 1000)) // Cap at 1000 reputation
    }
}

#[event]
pub struct VoteCastEvent {
    pub proposal_id: u64,
    pub voter: Pubkey,
    pub vote_choice: VoteChoice,
    pub agro_weight: u64,
    pub reputation_weight: u64,
    pub total_weight: u64,
    pub timestamp: i64,
}
