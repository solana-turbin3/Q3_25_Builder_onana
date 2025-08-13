use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::*;

/// CPI helper functions for governance program to interact with other programs
pub struct GovernanceCpi;

impl GovernanceCpi {
    /// Query user reputation from the reputation program
    pub fn get_user_reputation(
        _reputation_program: &AccountInfo,
        _reputation_config: &AccountInfo,
        user_reputation: &AccountInfo,
        _user_pubkey: &Pubkey,
    ) -> Result<u64> {
        // For now, return a calculated reputation based on account existence and data
        // In a full implementation, this would make an actual CPI call
        
        if user_reputation.data_is_empty() {
            // New user, default reputation
            return Ok(50);
        }
        
        // Try to read reputation data
        let account_data = user_reputation.try_borrow_data()
            .map_err(|_| GovernanceError::Unauthorized)?;
        
        if account_data.len() >= 8 {
            // Read reputation score from the first 8 bytes
            let reputation_bytes: [u8; 8] = account_data[0..8].try_into()
                .map_err(|_| GovernanceError::Unauthorized)?;
            let reputation = u64::from_le_bytes(reputation_bytes);
            Ok(reputation)
        } else {
            // Default for accounts with insufficient data
            Ok(50)
        }
    }

    /// Fund a proposal through the treasury program
    pub fn fund_proposal(
        _treasury_program: &AccountInfo,
        _treasury_config: &AccountInfo,
        _proposal_funding: &AccountInfo,
        authority: &AccountInfo,
        proposal_id: u64,
        amount: u64,
        _seeds: &[&[&[u8]]],
    ) -> Result<()> {
        msg!("Funding proposal {} with {} tokens via treasury", proposal_id, amount);
        
        // In a full implementation, this would construct and invoke a CPI call to the treasury
        // For now, we log the action
        msg!("Treasury CPI call would be made here with:");
        msg!("- Proposal ID: {}", proposal_id);
        msg!("- Amount: {}", amount);
        msg!("- Authority: {}", authority.key());
        
        // TODO: Implement actual CPI to treasury program
        // let cpi_accounts = treasury_dao::cpi::accounts::FundProposal {
        //     treasury_config: treasury_config.clone(),
        //     proposal_funding: proposal_funding.clone(),
        //     authority: authority.clone(),
        // };
        // let cpi_context = CpiContext::new_with_signer(treasury_program.clone(), cpi_accounts, seeds);
        // treasury_dao::cpi::fund_proposal(cpi_context, proposal_id, amount)?;
        
        Ok(())
    }

    /// Update research project status
    pub fn update_research_project(
        _research_program: &AccountInfo,
        _researcher_profile: &AccountInfo,
        _research_proposal: &AccountInfo,
        authority: &AccountInfo,
        proposal_id: u64,
        status: u8,
        _seeds: &[&[&[u8]]],
    ) -> Result<()> {
        msg!("Updating research proposal {} to status {}", proposal_id, status);
        
        // In a full implementation, this would construct and invoke a CPI call to research program
        msg!("Research CPI call would be made here with:");
        msg!("- Proposal ID: {}", proposal_id);
        msg!("- New Status: {}", status);
        msg!("- Authority: {}", authority.key());
        
        // TODO: Implement actual CPI to research program
        // let cpi_accounts = research_dao::cpi::accounts::UpdateProposal {
        //     researcher_profile: researcher_profile.clone(),
        //     research_proposal: research_proposal.clone(),
        //     authority: authority.clone(),
        // };
        // let cpi_context = CpiContext::new_with_signer(research_program.clone(), cpi_accounts, seeds);
        // research_dao::cpi::update_proposal_status(cpi_context, proposal_id, status)?;
        
        Ok(())
    }

    /// Update system parameters across multiple programs
    pub fn update_system_parameters(
        _treasury_program: &AccountInfo,
        _reputation_program: &AccountInfo,
        authority: &AccountInfo,
        parameter_data: &[u8],
        _seeds: &[&[&[u8]]],
    ) -> Result<()> {
        msg!("Updating system parameters across programs");
        
        // Parse parameter data to determine what to update
        if parameter_data.len() >= 1 {
            let param_type = parameter_data[0];
            
            match param_type {
                1 => {
                    // Emergency pause
                    msg!("Triggering emergency pause across all programs");
                    msg!("Authority: {}", authority.key());
                    // TODO: CPI calls to pause treasury and reputation programs
                },
                2 => {
                    // Threshold updates
                    msg!("Updating system thresholds");
                    msg!("Authority: {}", authority.key());
                    // TODO: CPI calls to update thresholds in relevant programs
                },
                _ => {
                    msg!("Unknown parameter type: {}", param_type);
                }
            }
        }
        
        Ok(())
    }
}

/// Helper struct for managing program seeds
pub struct ProgramSeeds;

impl ProgramSeeds {
    pub fn governance() -> Vec<u8> {
        GOVERNANCE_SEED.to_vec()
    }

    pub fn proposal(proposal_id: u64) -> Vec<Vec<u8>> {
        vec![
            PROPOSAL_SEED.to_vec(),
            proposal_id.to_le_bytes().to_vec(),
        ]
    }

    pub fn vote(proposal_id: u64, voter: &Pubkey) -> Vec<Vec<u8>> {
        vec![
            VOTE_SEED.to_vec(),
            proposal_id.to_le_bytes().to_vec(),
            voter.to_bytes().to_vec(),
        ]
    }
}

/// Instruction parser for different proposal types
pub struct InstructionParser;

#[derive(Debug)]
pub enum TreasuryInstruction {
    FundProposal { proposal_id: u64, amount: u64 },
    StakeTokens { amount: u64 },
    Withdraw { amount: u64 },
}

#[derive(Debug)]
pub enum ResearchInstruction {
    UpdateProposal { proposal_id: u64, status: u8 },
    FundResearch { research_id: u64, amount: u64 },
    CompleteResearch { research_id: u64 },
}

#[derive(Debug)]
pub enum ParameterInstruction {
    UpdateThresholds { quorum_bps: Option<u16>, approval_bps: Option<u16> },
    EmergencyAction { pause_system: bool },
    UpdateFees { new_fee_bps: u16 },
}

impl InstructionParser {
    pub fn parse_treasury_instruction(_data: &[u8]) -> Result<TreasuryInstruction> {
        // TODO: Implement proper instruction parsing
        // For now, return a default funding instruction
        Ok(TreasuryInstruction::FundProposal { 
            proposal_id: 1, 
            amount: 1000 
        })
    }

    pub fn parse_research_instruction(_data: &[u8]) -> Result<ResearchInstruction> {
        // TODO: Implement proper instruction parsing
        // For now, return a default update instruction
        Ok(ResearchInstruction::UpdateProposal { 
            proposal_id: 1, 
            status: 1 
        })
    }

    pub fn parse_parameter_instruction(_data: &[u8]) -> Result<ParameterInstruction> {
        // TODO: Implement proper instruction parsing
        // For now, return a default threshold update
        Ok(ParameterInstruction::UpdateThresholds { 
            quorum_bps: Some(5000), 
            approval_bps: Some(6000) 
        })
    }
}