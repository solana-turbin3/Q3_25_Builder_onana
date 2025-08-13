use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::error::*;

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct TallyVotes<'info> {
    #[account(
        seeds = [GOVERNANCE_SEED],
        bump = governance_config.bump
    )]
    pub governance_config: Account<'info, GovernanceConfig>,

    #[account(
        mut,
        seeds = [PROPOSAL_SEED, &proposal_id.to_le_bytes()],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Active @ GovernanceError::ProposalNotFound
    )]
    pub proposal: Account<'info, Proposal>,

    /// CHECK: Authority can be anyone for tallying
    pub authority: Signer<'info>,
}

impl<'info> TallyVotes<'info> {
    pub fn tally_votes(&mut self, proposal_id: u64) -> Result<()> {
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        // Check if voting period has ended
        require!(
            self.proposal.is_voting_ended(current_time),
            GovernanceError::VotingPeriodActive
        );

        // Calculate quorum threshold based on total AGRO supply
        // For now, we'll use total voting power as the base
        let quorum_threshold = (self.proposal.total_voting_power * self.governance_config.quorum_threshold_bps as u64) / BASIS_POINTS_MAX as u64;
        
        // Check if quorum is met
        let total_participating_power = self.proposal.yes_votes + self.proposal.no_votes;
        require!(
            total_participating_power >= quorum_threshold,
            GovernanceError::InsufficientQuorum
        );

        // Calculate approval threshold
        let approval_threshold = (total_participating_power * self.governance_config.approval_threshold_bps as u64) / BASIS_POINTS_MAX as u64;
        
        // Determine if proposal is approved
        let is_approved = self.proposal.yes_votes >= approval_threshold;

        // Update proposal status
        self.proposal.status = if is_approved {
            ProposalStatus::Approved
        } else {
            ProposalStatus::Rejected
        };

        emit!(ProposalTallied {
            proposal_id,
            status: self.proposal.status.clone(),
            yes_votes: self.proposal.yes_votes,
            no_votes: self.proposal.no_votes,
            total_voting_power: self.proposal.total_voting_power,
            quorum_reached: total_participating_power >= quorum_threshold,
            approval_reached: is_approved,
        });

        Ok(())
    }
}

#[event]
pub struct ProposalTallied {
    pub proposal_id: u64,
    pub status: ProposalStatus,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub total_voting_power: u64,
    pub quorum_reached: bool,
    pub approval_reached: bool,
}
