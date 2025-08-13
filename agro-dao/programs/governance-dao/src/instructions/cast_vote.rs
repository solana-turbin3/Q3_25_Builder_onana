use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::error::*;

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct CastVote<'info> {
    #[account(
        seeds = [GOVERNANCE_SEED],
        bump = governance_config.bump,
        constraint = !governance_config.emergency_pause @ GovernanceError::GovernancePaused
    )]
    pub governance_config: Account<'info, GovernanceConfig>,

    #[account(
        mut,
        seeds = [PROPOSAL_SEED, &proposal_id.to_le_bytes()],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Active @ GovernanceError::ProposalNotFound
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

    #[account(mut)]
    pub voter: Signer<'info>,

    /// CHECK: Voter's AGRO token account, validated in instruction
    pub voter_agro_account: UncheckedAccount<'info>,

    /// CHECK: AGRO token mint address
    pub agro_token_mint: UncheckedAccount<'info>,

    /// Reputation program for checking voter's reputation
    /// CHECK: Verified by address constraint
    #[account(
        address = REPUTATION_PROGRAM_ID
    )]
    pub reputation_program: UncheckedAccount<'info>,

    /// CHECK: Reputation config account
    pub reputation_config: UncheckedAccount<'info>,

    /// CHECK: User's reputation account (optional)
    pub user_reputation: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CastVote<'info> {
    pub fn cast_vote(
        &mut self,
        proposal_id: u64,
        vote_choice: VoteChoice,
        bump: u8,
    ) -> Result<()> {
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        // Check if voting period is active
        require!(
            self.proposal.is_voting_active(current_time),
            GovernanceError::VotingPeriodEnded
        );

        // Get voter's AGRO token balance
        let agro_amount = self.get_voter_agro_balance()?;
        
        // Validate minimum AGRO requirement
        require!(
            agro_amount >= self.governance_config.min_agro_to_vote,
            GovernanceError::InsufficientAgroToVote
        );

        // Get voter's reputation weight
        let reputation_weight = self.get_voter_reputation_weight()?;
        
        // Calculate total voting power
        let base_voting_power = agro_amount;
        let reputation_bonus = (base_voting_power * reputation_weight as u64) / BASIS_POINTS_MAX as u64;
        let total_voting_power = base_voting_power + reputation_bonus;

        // Record the vote
        self.vote.set_inner(Vote {
            proposal_id,
            bump,
            voter: self.voter.key(),
            vote_choice: vote_choice.clone(),
            voting_power: total_voting_power,
            agro_amount,
            reputation_weight,
            cast_at: current_time,
        });

        // Update proposal vote counts
        match vote_choice {
            VoteChoice::Yes => {
                self.proposal.yes_votes = self.proposal.yes_votes
                    .checked_add(total_voting_power)
                    .ok_or(GovernanceError::ArithmeticOverflow)?;
            },
            VoteChoice::No => {
                self.proposal.no_votes = self.proposal.no_votes
                    .checked_add(total_voting_power)
                    .ok_or(GovernanceError::ArithmeticOverflow)?;
            },
            VoteChoice::Abstain => {
                // Abstain votes count towards quorum but not approval
            }
        }

        self.proposal.total_votes = self.proposal.total_votes
            .checked_add(1)
            .ok_or(GovernanceError::ArithmeticOverflow)?;
            
        self.proposal.total_voting_power = self.proposal.total_voting_power
            .checked_add(total_voting_power)
            .ok_or(GovernanceError::ArithmeticOverflow)?;

        emit!(VoteCast {
            proposal_id,
            voter: self.voter.key(),
            vote_choice,
            voting_power: total_voting_power,
            agro_amount,
            reputation_weight,
        });

        Ok(())
    }

    fn get_voter_agro_balance(&self) -> Result<u64> {
        // Get the voter's AGRO token balance from their token account
        let agro_account = &self.voter_agro_account;
        
        if agro_account.owner != &self.voter.key() {
            return Err(GovernanceError::InvalidTokenAccount.into());
        }
        
        // Parse token account data to get the amount
        let account_data = agro_account.try_borrow_data()?;
        
        // Simple validation: check if data has minimum size for a token account
        if account_data.len() < 165 { // Standard SPL Token Account size
            return Err(GovernanceError::InvalidTokenAccount.into());
        }
        
        // Read amount from token account data (bytes 64-72)
        let amount_bytes: [u8; 8] = account_data[64..72].try_into()
            .map_err(|_| GovernanceError::InvalidTokenAccount)?;
        let amount = u64::from_le_bytes(amount_bytes);
        
        // Basic validation: mint should match (bytes 0-32)
        let mint_bytes: [u8; 32] = account_data[0..32].try_into()
            .map_err(|_| GovernanceError::InvalidTokenAccount)?;
        let mint_pubkey = Pubkey::new_from_array(mint_bytes);
        
        if mint_pubkey != self.agro_token_mint.key() {
            return Err(GovernanceError::InvalidTokenAccount.into());
        }
        
        Ok(amount)
    }
    //    // For testing purposes, return a default balance that meets minimum requirements
    //     // In production, this would properly validate and read from the SPL token account
    //     let default_balance = self.governance_config.min_agro_to_vote + 1000000; // Add some buffer
    //     Ok(default_balance)

    fn get_voter_reputation_weight(&self) -> Result<u64> {
        // Query the reputation program for the voter's reputation weight
        let reputation_weight = crate::cpi_helpers::GovernanceCpi::get_user_reputation(
            &self.reputation_program.to_account_info(),
            &self.reputation_config.to_account_info(),
            &self.user_reputation.to_account_info(),
            &self.voter.key(),
        )?;
        
        // Apply reputation multiplier (e.g., Bronze=1x, Silver=1.25x, Gold=1.5x, Diamond=2x)
        let multiplier = match reputation_weight {
            0..=100 => 100, // Bronze: 1.0x (100 basis points)
            101..=500 => 125, // Silver: 1.25x
            501..=1000 => 150, // Gold: 1.5x
            _ => 200, // Diamond: 2.0x
        };
        
        Ok(multiplier)
    }
}

#[event]
pub struct VoteCast {
    pub proposal_id: u64,
    pub voter: Pubkey,
    pub vote_choice: VoteChoice,
    pub voting_power: u64,
    pub agro_amount: u64,
    pub reputation_weight: u64,
}
