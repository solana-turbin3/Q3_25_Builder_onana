use anchor_lang::prelude::*;
use crate::state::{Proposal, GovernanceConfig, ProposalStatus, ProposalType};
use crate::constants::*;
use crate::error::GovernanceError;

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct TallyVotes<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [PROPOSAL_SEED, &proposal_id.to_le_bytes()],
        bump = proposal.bump,
        constraint = proposal.proposal_status == ProposalStatus::Active @ GovernanceError::ProposalNotActive,
        constraint = Clock::get()?.unix_timestamp > proposal.voting_end_time @ GovernanceError::VotingPeriodNotEnded
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(
        seeds = [GOVERNANCE_CONFIG_SEED],
        bump = governance_config.bump,
        constraint = !governance_config.emergency_pause @ GovernanceError::GovernancePaused
    )]
    pub governance_config: Account<'info, GovernanceConfig>,
}

impl<'info> TallyVotes<'info> {
    pub fn tally_votes(&mut self, proposal_id: u64) -> Result<()> {
        let clock = Clock::get()?;

        // Ensure voting period has ended
        require!(
            clock.unix_timestamp > self.proposal.voting_end_time,
            GovernanceError::VotingPeriodNotEnded
        );

        // Ensure proposal hasn't already been tallied
        require!(
            self.proposal.proposal_status == ProposalStatus::Active,
            GovernanceError::ProposalAlreadyTallied
        );

        // Check if quorum is met
        if !self.proposal.quorum_reached {
            self.proposal.proposal_status = ProposalStatus::Failed;
            self.proposal.failure_reason = Some("Quorum not reached".to_string());
            
            emit!(ProposalTalliedEvent {
                proposal_id,
                outcome: ProposalOutcome::Failed,
                reason: "Quorum not reached".to_string(),
                votes_for: self.proposal.total_votes_for,
                votes_against: self.proposal.total_votes_against,
                abstain_votes: self.proposal.total_abstain_votes,
                total_voters: self.proposal.total_voters,
                timestamp: clock.unix_timestamp,
            });

            return Ok(());
        }

        // Calculate voting outcome
        let total_decisive_votes = self.proposal.total_votes_for
            .checked_add(self.proposal.total_votes_against)
            .ok_or(GovernanceError::ArithmeticOverflow)?;

        // Require minimum participation for decisive votes
        if total_decisive_votes == 0 {
            self.proposal.proposal_status = ProposalStatus::Failed;
            self.proposal.failure_reason = Some("No decisive votes cast".to_string());
            
            emit!(ProposalTalliedEvent {
                proposal_id,
                outcome: ProposalOutcome::Failed,
                reason: "No decisive votes cast".to_string(),
                votes_for: self.proposal.total_votes_for,
                votes_against: self.proposal.total_votes_against,
                abstain_votes: self.proposal.total_abstain_votes,
                total_voters: self.proposal.total_voters,
                timestamp: clock.unix_timestamp,
            });

            return Ok(());
        }

        // Calculate approval percentage
        let approval_percentage_bps = self.proposal.total_votes_for
            .checked_mul(10000)
            .ok_or(GovernanceError::ArithmeticOverflow)?
            .checked_div(total_decisive_votes)
            .ok_or(GovernanceError::ArithmeticOverflow)?;

        // Determine approval threshold based on proposal type
        let required_approval_bps = match self.proposal.proposal_type {
            ProposalType::Treasury => self.governance_config.approval_threshold_bps,
            ProposalType::Parameter => self.governance_config.parameter_change_threshold_bps,
            ProposalType::Emergency => {
                // Emergency proposals require higher approval
                std::cmp::max(
                    self.governance_config.approval_threshold_bps + 1000, // +10%
                    7500 // Minimum 75%
                )
            }
        };

        // Determine outcome
        if approval_percentage_bps >= required_approval_bps as u64 {
            self.proposal.proposal_status = ProposalStatus::Approved;
            
            // Set execution window
            self.proposal.execution_available_at = clock.unix_timestamp + EXECUTION_DELAY;
            self.proposal.execution_expires_at = clock.unix_timestamp + EXECUTION_DELAY + EXECUTION_WINDOW;

            emit!(ProposalTalliedEvent {
                proposal_id,
                outcome: ProposalOutcome::Approved,
                reason: format!("Approved with {}% support", approval_percentage_bps / 100),
                votes_for: self.proposal.total_votes_for,
                votes_against: self.proposal.total_votes_against,
                abstain_votes: self.proposal.total_abstain_votes,
                total_voters: self.proposal.total_voters,
                timestamp: clock.unix_timestamp,
            });
        } else {
            self.proposal.proposal_status = ProposalStatus::Failed;
            self.proposal.failure_reason = Some(format!(
                "Insufficient approval: {}% (required: {}%)",
                approval_percentage_bps / 100,
                required_approval_bps / 100
            ));

            emit!(ProposalTalliedEvent {
                proposal_id,
                outcome: ProposalOutcome::Failed,
                reason: format!("Insufficient approval: {}%", approval_percentage_bps / 100),
                votes_for: self.proposal.total_votes_for,
                votes_against: self.proposal.total_votes_against,
                abstain_votes: self.proposal.total_abstain_votes,
                total_voters: self.proposal.total_voters,
                timestamp: clock.unix_timestamp,
            });
        }

        Ok(())
    }
}

#[event]
pub struct ProposalTalliedEvent {
    pub proposal_id: u64,
    pub outcome: ProposalOutcome,
    pub reason: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub abstain_votes: u64,
    pub total_voters: u32,
    pub timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalOutcome {
    Approved,
    Failed,
}
