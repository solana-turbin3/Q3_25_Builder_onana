use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::instruction::Instruction;
use crate::state::{Proposal, GovernanceConfig, ProposalStatus, ProposalType};
use crate::constants::*;
use crate::error::GovernanceError;

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub executor: Signer<'info>,

    #[account(
        mut,
        seeds = [PROPOSAL_SEED, &proposal_id.to_le_bytes()],
        bump = proposal.bump,
        constraint = proposal.proposal_status == ProposalStatus::Approved @ GovernanceError::ProposalNotApproved,
        constraint = Clock::get()?.unix_timestamp >= proposal.execution_available_at @ GovernanceError::ExecutionNotYetAvailable,
        constraint = Clock::get()?.unix_timestamp <= proposal.execution_expires_at @ GovernanceError::ExecutionWindowExpired
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(
        seeds = [GOVERNANCE_CONFIG_SEED],
        bump = governance_config.bump,
        constraint = !governance_config.emergency_pause @ GovernanceError::GovernancePaused
    )]
    pub governance_config: Account<'info, GovernanceConfig>,

    /// CHECK: This will be the treasury program account
    #[account(mut)]
    pub treasury_program: UncheckedAccount<'info>,

    /// CHECK: This will be the research program account  
    #[account(mut)]
    pub research_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ExecuteProposal<'info> {
    pub fn execute_proposal(&mut self, proposal_id: u64) -> Result<()> {
        let clock = Clock::get()?;

        // Validate execution timing
        require!(
            clock.unix_timestamp >= self.proposal.execution_available_at,
            GovernanceError::ExecutionNotYetAvailable
        );
        require!(
            clock.unix_timestamp <= self.proposal.execution_expires_at,
            GovernanceError::ExecutionWindowExpired
        );

        // Execute based on proposal type
        match self.proposal.proposal_type {
            ProposalType::Treasury => {
                self.execute_treasury_proposal(proposal_id)?;
            },
            ProposalType::Parameter => {
                self.execute_parameter_proposal(proposal_id)?;
            },
            ProposalType::Emergency => {
                self.execute_emergency_proposal(proposal_id)?;
            }
        }

        // Mark proposal as executed
        self.proposal.proposal_status = ProposalStatus::Executed;
        self.proposal.executed_at = Some(clock.unix_timestamp);
        self.proposal.executed_by = Some(self.executor.key());

        emit!(ProposalExecutedEvent {
            proposal_id,
            proposal_type: self.proposal.proposal_type.clone(),
            executor: self.executor.key(),
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    fn execute_treasury_proposal(&self, proposal_id: u64) -> Result<()> {
        // Parse treasury-specific data from proposal_data
        // For now, we'll implement a basic structure
        // In a full implementation, this would deserialize specific treasury instructions
        
        msg!("Executing treasury proposal: {}", proposal_id);
        
        // Create governance authority seeds for CPI
        let governance_seeds = &[
            GOVERNANCE_CONFIG_SEED,
            &[self.governance_config.bump],
        ];

        // Example: Treasury funding instruction
        // This would be replaced with actual instruction data from the proposal
        let treasury_instruction_data = self.proposal.instruction_data.clone()
            .ok_or(GovernanceError::NoInstructionData)?;

        // Create the instruction for treasury program
        let treasury_instruction = Instruction {
            program_id: self.treasury_program.key(),
            accounts: vec![], // This would be populated with actual account metas
            data: treasury_instruction_data,
        };

        // Execute CPI to treasury program
        invoke_signed(
            &treasury_instruction,
            &[], // This would include actual account infos
            &[governance_seeds],
        ).map_err(|_| GovernanceError::ExecutionFailed)?;

        Ok(())
    }

    fn execute_parameter_proposal(&mut self, proposal_id: u64) -> Result<()> {
        msg!("Executing parameter change proposal: {}", proposal_id);
        
        // Parse parameter change data
        let param_data = self.proposal.instruction_data.clone()
            .ok_or(GovernanceError::NoInstructionData)?;

        // For demonstration, we'll show how parameters could be updated
        // In practice, this would deserialize specific parameter change instructions
        
        // Example parameter updates (this would be data-driven):
        if param_data.len() >= 8 {
            let new_quorum = u16::from_le_bytes([param_data[0], param_data[1]]);
            let new_approval = u16::from_le_bytes([param_data[2], param_data[3]]);
            
            // Validate new parameters
            require!(new_quorum <= 10000, GovernanceError::InvalidParameter);
            require!(new_approval <= 10000, GovernanceError::InvalidParameter);
            require!(new_quorum >= 100, GovernanceError::InvalidParameter); // Min 1%
            require!(new_approval >= 5000, GovernanceError::InvalidParameter); // Min 50%

            // Update governance config (would need mutable reference)
            msg!("Would update quorum to {} and approval to {}", new_quorum, new_approval);
        }

        Ok(())
    }

    fn execute_emergency_proposal(&self, proposal_id: u64) -> Result<()> {
        msg!("Executing emergency proposal: {}", proposal_id);
        
        // Emergency proposals might pause/unpause systems, emergency withdrawals, etc.
        let emergency_data = self.proposal.instruction_data.clone()
            .ok_or(GovernanceError::NoInstructionData)?;

        // Example emergency actions
        if !emergency_data.is_empty() {
            match emergency_data[0] {
                1 => {
                    // Emergency pause
                    msg!("Emergency pause would be executed");
                },
                2 => {
                    // Emergency unpause
                    msg!("Emergency unpause would be executed");
                },
                3 => {
                    // Emergency withdrawal
                    msg!("Emergency withdrawal would be executed");
                },
                _ => {
                    return Err(GovernanceError::InvalidEmergencyAction.into());
                }
            }
        }

        Ok(())
    }
}

#[event]
pub struct ProposalExecutedEvent {
    pub proposal_id: u64,
    pub proposal_type: ProposalType,
    pub executor: Pubkey,
    pub timestamp: i64,
}
