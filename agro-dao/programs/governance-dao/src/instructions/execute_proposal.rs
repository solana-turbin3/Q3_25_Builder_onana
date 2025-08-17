use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use crate::state::*;
use crate::constants::*;
use crate::error::*;

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct ExecuteProposal<'info> {
    #[account(
        seeds = [GOVERNANCE_SEED],
        bump = governance_config.bump
    )]
    pub governance_config: Account<'info, GovernanceConfig>,

    #[account(
        mut,
        seeds = [PROPOSAL_SEED, &proposal_id.to_le_bytes()],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Approved @ GovernanceError::ProposalNotApproved
    )]
    pub proposal: Account<'info, Proposal>,

    /// CHECK: Authority can be anyone for execution
    pub authority: Signer<'info>,

    /// Treasury program for treasury proposals
    /// CHECK: Verified by address constraint
    #[account(
        address = TREASURY_PROGRAM_ID
    )]
    pub treasury_program: UncheckedAccount<'info>,

    /// Research program for research proposals
    /// CHECK: Verified by address constraint
    #[account(
        address = RESEARCH_PROGRAM_ID
    )]
    pub research_program: UncheckedAccount<'info>,

    /// Reputation program for reputation updates
    /// CHECK: Verified by address constraint
    #[account(
        address = REPUTATION_PROGRAM_ID
    )]
    pub reputation_program: UncheckedAccount<'info>,

    /// CHECK: Treasury config account for treasury proposals
    pub treasury_config: UncheckedAccount<'info>,

    /// CHECK: Proposal funding account for treasury proposals  
    pub proposal_funding: UncheckedAccount<'info>,

    /// CHECK: Stake account for treasury proposals
    pub stake_account: UncheckedAccount<'info>,

    /// CHECK: AGRO mint account for treasury proposals
    pub agro_mint: UncheckedAccount<'info>,

    /// CHECK: Stakeholder AGRO token account for treasury proposals
    pub stakeholder_agro_account: UncheckedAccount<'info>,

    /// CHECK: Research proposal account for research proposals
    pub research_proposal: UncheckedAccount<'info>,

    /// CHECK: Researcher profile for research proposals
    pub researcher_profile: UncheckedAccount<'info>,

    /// Token program for token operations
    pub token_program: Program<'info, Token>,

    /// Rent sysvar for account creation
    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

impl<'info> ExecuteProposal<'info> {
    pub fn execute_proposal(&mut self, proposal_id: u64) -> Result<()> {
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        // Check if execution window is still open
        require!(
            self.proposal.is_execution_window_open(current_time),
            GovernanceError::ExecutionWindowExpired
        );

        // Check if proposal has already been executed
        require!(
            self.proposal.executed_at.is_none(),
            GovernanceError::ProposalAlreadyExecuted
        );

        // Execute based on proposal type
        match self.proposal.proposal_type {
            ProposalType::Treasury => {
                self.execute_treasury_proposal(proposal_id)?;
            },
            ProposalType::Research => {
                self.execute_research_proposal(proposal_id)?;
            },
            ProposalType::ParameterChange => {
                self.execute_parameter_change(proposal_id)?;
            },
            ProposalType::Emergency => {
                self.execute_emergency_action(proposal_id)?;
            }
        }

        // Mark proposal as executed
        self.proposal.status = ProposalStatus::Executed;
        self.proposal.executed_at = Some(current_time);

        emit!(ProposalExecuted {
            proposal_id,
            executed_by: self.authority.key(),
            executed_at: current_time,
            proposal_type: self.proposal.proposal_type.clone(),
        });

        Ok(())
    }

    fn execute_treasury_proposal(&mut self, proposal_id: u64) -> Result<()> {
        msg!("Executing treasury proposal: {}", proposal_id);
        
        // Parse instruction data for treasury operations
        if let Some(ref instruction_data) = self.proposal.instruction_data {
            let treasury_instruction = crate::cpi_helpers::InstructionParser::parse_treasury_instruction(instruction_data)?;
            
            match treasury_instruction {
                crate::cpi_helpers::TreasuryInstruction::FundProposal { proposal_id: fund_id, amount } => {
                    // Call treasury program with full account setup for actual CPI
                    crate::cpi_helpers::GovernanceCpi::fund_treasury_proposal(
                        &self.treasury_program.to_account_info(),
                        &self.treasury_config.to_account_info(),
                        &self.proposal_funding.to_account_info(),
                        &self.stake_account.to_account_info(),
                        &self.agro_mint.to_account_info(),
                        &self.stakeholder_agro_account.to_account_info(),
                        &self.authority.to_account_info(),
                        &self.research_proposal.to_account_info(),
                        &self.researcher_profile.to_account_info(),
                        &self.research_program.to_account_info(),
                        &self.token_program.to_account_info(),
                        &self.system_program.to_account_info(),
                        &self.rent.to_account_info(),
                        fund_id,
                        amount,
                        &[&[GOVERNANCE_SEED, &[self.governance_config.bump]]],
                    )?;
                },
                _ => {
                    msg!("Treasury instruction type not yet implemented");
                }
            }
        }
        
        Ok(())
    }

    fn execute_research_proposal(&mut self, proposal_id: u64) -> Result<()> {
        msg!("Executing research proposal: {}", proposal_id);
        
        // Research proposals are funded through the treasury system
        // Parse as treasury instruction since research funding goes through treasury
        if let Some(ref instruction_data) = self.proposal.instruction_data {
            let treasury_instruction = crate::cpi_helpers::InstructionParser::parse_treasury_instruction(instruction_data)?;
            
            match treasury_instruction {
                crate::cpi_helpers::TreasuryInstruction::FundProposal { proposal_id: fund_id, amount } => {
                    msg!("Funding research proposal {} with {} tokens", fund_id, amount);
                    
                    // Call treasury program to fund the research proposal
                    crate::cpi_helpers::GovernanceCpi::fund_treasury_proposal(
                        &self.treasury_program.to_account_info(),
                        &self.treasury_config.to_account_info(),
                        &self.proposal_funding.to_account_info(),
                        &self.stake_account.to_account_info(),
                        &self.agro_mint.to_account_info(),
                        &self.stakeholder_agro_account.to_account_info(),
                        &self.authority.to_account_info(),
                        &self.research_proposal.to_account_info(),
                        &self.researcher_profile.to_account_info(),
                        &self.research_program.to_account_info(),
                        &self.token_program.to_account_info(),
                        &self.system_program.to_account_info(),
                        &self.rent.to_account_info(),
                        fund_id,
                        amount,
                        &[&[GOVERNANCE_SEED, &[self.governance_config.bump]]],
                    )?;
                    
                    msg!("Research proposal funding executed successfully");
                },
                _ => {
                    msg!("Treasury instruction type not supported for research proposals");
                }
            }
        }
        
        Ok(())
    }

    fn execute_parameter_change(&mut self, proposal_id: u64) -> Result<()> {
        msg!("Executing parameter change proposal: {}", proposal_id);
        
        // Parameter changes modify governance, treasury, or system parameters
        if let Some(ref instruction_data) = self.proposal.instruction_data {
            let param_instruction = crate::cpi_helpers::InstructionParser::parse_parameter_instruction(instruction_data)?;
            
            match param_instruction {
                crate::cpi_helpers::ParameterInstruction::EmergencyAction { pause_system } => {
                    msg!("Emergency action: pause_system = {}", pause_system);
                    // Update emergency pause across all programs with full CPI  
                    crate::cpi_helpers::GovernanceCpi::update_system_parameters(
                        &self.treasury_program.to_account_info(),
                        &self.treasury_config.to_account_info(),
                        &self.reputation_program.to_account_info(),
                        &self.authority.to_account_info(),
                        instruction_data,
                        &[&[GOVERNANCE_SEED, &[self.governance_config.bump]]],
                    )?;
                },
            }
        }
        
        Ok(())
    }

    fn execute_emergency_action(&mut self, proposal_id: u64) -> Result<()> {
        msg!("Executing emergency proposal: {}", proposal_id);
        
        // Emergency actions might pause systems, update critical parameters, etc.
        if let Some(ref instruction_data) = self.proposal.instruction_data {
            // Emergency actions get special treatment and can bypass normal parameter validation
            crate::cpi_helpers::GovernanceCpi::update_system_parameters(
                &self.treasury_program.to_account_info(),
                &self.treasury_config.to_account_info(),
                &self.reputation_program.to_account_info(),
                &self.authority.to_account_info(),
                instruction_data,
                &[&[GOVERNANCE_SEED, &[self.governance_config.bump]]],
            )?;
        }
        
        Ok(())
    }
}

#[event]
pub struct ProposalExecuted {
    pub proposal_id: u64,
    pub executed_by: Pubkey,
    pub executed_at: i64,
    pub proposal_type: ProposalType,
}
