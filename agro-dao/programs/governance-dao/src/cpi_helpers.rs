use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::*;

// Import CPI modules from other programs for cross-program calls
use treasury_dao::cpi::accounts::{FundProposal, EmergencyPause};
use treasury_dao::cpi::{fund_proposal, emergency_pause};
use reputation_dao::cpi::accounts::GetReputation;
use reputation_dao::cpi::get_reputation;

/// CPI helper functions for governance program to interact with other programs
/// This is the main interface for cross-program communication from governance
pub struct GovernanceCpi;

impl GovernanceCpi {
    /// Get user reputation from the reputation program
    /// First tries to read directly from account data for performance
    /// Falls back to CPI call if direct read fails or account is uninitialized
    pub fn get_user_reputation<'info>(
        reputation_program: &AccountInfo<'info>,
        reputation_config: &AccountInfo<'info>,
        user_reputation: &AccountInfo<'info>,
        user_pubkey: &Pubkey,
        seeds: &[&[&[u8]]],
    ) -> Result<u64> {
        msg!("Fetching reputation for user: {}", user_pubkey);
        
        // Try to read reputation directly from account data first (faster)
        if !user_reputation.data_is_empty() {
            // Get the raw account data
            let account_data = user_reputation.try_borrow_data()
                .map_err(|_| GovernanceError::Unauthorized)?;
            
            // UserReputation account layout: [discriminator(8)][bump(1)][user(32)][score(8)]...
            if account_data.len() >= 49 {
                // Skip to the reputation score field at byte offset 41
                let reputation_bytes: [u8; 8] = account_data[41..49].try_into()
                    .map_err(|_| GovernanceError::Unauthorized)?;
                let reputation_score = i64::from_le_bytes(reputation_bytes);
                
                // Convert negative scores to 0 for safety
                let final_score = if reputation_score < 0 { 0 } else { reputation_score as u64 };
                
                msg!("Read reputation directly from account: {} for user: {}", final_score, user_pubkey);
                return Ok(final_score);
            }
        }
        
        // If direct read fails, use CPI to reputation program
        let cpi_accounts = GetReputation {
            reputation_config: reputation_config.clone(),
            user_reputation: user_reputation.clone(),
        };
        
        // Create CPI context with governance program as signer
        let cpi_context = CpiContext::new_with_signer(
            reputation_program.clone(),
            cpi_accounts,
            seeds
        );
        
        // Make the CPI call (this will initialize account if needed)
        let result = get_reputation(cpi_context, *user_pubkey);
        
        match result {
            Ok(_) => {
                msg!("CPI call successful, reading updated account data");
                
                // After successful CPI, read the updated reputation from the account
                // Use consistent byte offset (41-49) matching the direct read path
                let account_data = user_reputation.try_borrow_data()
                    .map_err(|_| GovernanceError::Unauthorized)?;
                
                if account_data.len() >= 49 {
                    let reputation_bytes: [u8; 8] = account_data[41..49].try_into()
                        .map_err(|_| GovernanceError::Unauthorized)?;
                    let reputation_score = i64::from_le_bytes(reputation_bytes);
                    let final_score = if reputation_score < 0 { 0 } else { reputation_score as u64 };
                    Ok(final_score)
                } else {
                    // Fallback to default if account data is incomplete
                    Ok(50)
                }
            },
            Err(e) => {
                msg!("CPI call failed: {:?}, using fallback", e);
                // Read the account data after CPI call
                Self::get_reputation_fallback(user_reputation)
            }
        }
    }
    
    /// Fallback method to read reputation data directly from account
    /// Used when CPI calls fail or as a secondary read method
    fn get_reputation_fallback(user_reputation: &AccountInfo) -> Result<u64> {
        if user_reputation.data_is_empty() {
            return Ok(50); // Default reputation for new users
        }
        
        let account_data = user_reputation.try_borrow_data()
            .map_err(|_| GovernanceError::Unauthorized)?;
        
        // Use consistent byte offset (41-49) matching the main read path
        if account_data.len() >= 49 {
            let reputation_bytes: [u8; 8] = account_data[41..49].try_into()
                .map_err(|_| GovernanceError::Unauthorized)?;
            let reputation_score = i64::from_le_bytes(reputation_bytes);
            let final_score = if reputation_score < 0 { 0 } else { reputation_score as u64 };
            Ok(final_score)
        } else {
            Ok(50) // Default fallback reputation
        }
    }

    /// Fund a research proposal through the treasury program using CPI
    /// This is the main funding mechanism that governance uses to allocate treasury funds
    /// Requires all the treasury and research accounts to be properly set up
    pub fn fund_treasury_proposal<'info>(
        treasury_program: &AccountInfo<'info>,
        treasury_config: &AccountInfo<'info>,
        proposal_funding: &AccountInfo<'info>,
        stake_account: &AccountInfo<'info>,
        agro_mint: &AccountInfo<'info>,
        stakeholder_agro_account: &AccountInfo<'info>,
        authority: &AccountInfo<'info>,
        research_proposal: &AccountInfo<'info>,
        researcher_profile: &AccountInfo<'info>,
        research_dao_program: &AccountInfo<'info>,
        token_program: &AccountInfo<'info>,
        system_program: &AccountInfo<'info>,
        rent: &AccountInfo<'info>,
        proposal_id: u64,
        amount: u64,
        seeds: &[&[&[u8]]],
    ) -> Result<()> {
        msg!("Executing treasury proposal funding via actual CPI");
        msg!("Proposal ID: {}, Amount: {} tokens", proposal_id, amount);
        
        // Build the complete account structure for treasury fund_proposal CPI
        let cpi_accounts = FundProposal {
            treasury_config: treasury_config.clone(),
            proposal_funding: proposal_funding.clone(),
            stake_account: stake_account.clone(),
            agro_mint: agro_mint.clone(),
            stakeholder_agro_account: stakeholder_agro_account.clone(),
            stakeholder: authority.clone(),
            research_proposal: research_proposal.clone(),
            researcher_profile: researcher_profile.clone(),
            research_dao_program: research_dao_program.clone(),
            token_program: token_program.clone(),
            system_program: system_program.clone(),
            rent: rent.clone(),
        };
        
        // Create CPI context with governance program seeds for signing authority
        let cpi_ctx = CpiContext::new_with_signer(treasury_program.clone(), cpi_accounts, seeds);
        
        // Execute the actual CPI call to treasury program
        fund_proposal(cpi_ctx, proposal_id.to_string(), proposal_id, amount)?;
        
        msg!("Treasury funding executed successfully via actual CPI");
        msg!("  - Proposal ID: {}", proposal_id);
        msg!("  - Amount: {} tokens", amount);
        msg!("  - Authority: {}", authority.key());
        
        Ok(())
    }

    /// Update system parameters across multiple programs using CPI calls
    /// Currently only supports emergency pause functionality
    /// Treasury emergency pause is the only implemented cross-program operation
    pub fn update_system_parameters<'info>(
        treasury_program: &AccountInfo<'info>,
        treasury_config: &AccountInfo<'info>,
        _reputation_program: &AccountInfo<'info>,
        authority: &AccountInfo<'info>,
        parameter_data: &[u8],
        seeds: &[&[&[u8]]],
    ) -> Result<()> {
        msg!("Updating system parameters across programs via actual CPI");
        
        // Must have at least one byte for parameter type
        if parameter_data.is_empty() {
            return Err(GovernanceError::InvalidInstruction.into());
        }
        
        let param_type = parameter_data[0];
        
        match param_type {
            1 => {
                // Emergency pause: halt operations across all programs
                msg!(" Executing cross-program emergency pause via actual CPI");
                msg!("Authority: {}", authority.key());
                
                // Pause treasury operations first (most critical)
                msg!(" Triggering emergency pause on Treasury DAO:");
                
                let treasury_cpi_accounts = EmergencyPause {
                    treasury_config: treasury_config.clone(),
                    authority: authority.clone(),
                };
                let treasury_ctx = CpiContext::new_with_signer(treasury_program.clone(), treasury_cpi_accounts, seeds);
                emergency_pause(treasury_ctx)?;
                msg!("  Treasury DAO: Emergency operations halted via actual CPI");
                
                msg!("Emergency pause executed (Treasury only)");
            },
            _ => {
                msg!("Unsupported parameter type: {}", param_type);
                msg!("Currently only emergency pause (type 1) is supported");
                return Err(GovernanceError::InvalidInstruction.into());
            }
        }
        
        msg!("System parameter update completed");
        Ok(())
    }
}

/// Helper struct for managing program seeds
/// Seeds are used for PDAs and cross-program invocation signing
pub struct ProgramSeeds;

impl ProgramSeeds {
    /// Get the governance program seed
    pub fn governance() -> Vec<u8> {
        GOVERNANCE_SEED.to_vec()
    }

    /// Get seeds for a specific proposal PDA
    pub fn proposal(proposal_id: u64) -> Vec<Vec<u8>> {
        vec![
            PROPOSAL_SEED.to_vec(),
            proposal_id.to_le_bytes().to_vec(),
        ]
    }

    /// Get seeds for a vote PDA (proposal + voter)
    pub fn vote(proposal_id: u64, voter: &Pubkey) -> Vec<Vec<u8>> {
        vec![
            VOTE_SEED.to_vec(),
            proposal_id.to_le_bytes().to_vec(),
            voter.to_bytes().to_vec(),
        ]
    }
}

/// Instruction parser for different proposal types
/// This helps decode proposal instruction data to determine what action to take
pub struct InstructionParser;

/// Treasury instruction types that governance can execute
#[derive(Debug)]
pub enum TreasuryInstruction {
    FundProposal { proposal_id: u64, amount: u64 },
    StakeTokens { amount: u64 },
    Withdraw { amount: u64 },
}

/// Parameter instruction types for system-wide updates
#[derive(Debug)]
pub enum ParameterInstruction {
    EmergencyAction { pause_system: bool },
}

impl InstructionParser {
    /// Parse treasury instruction data from proposal
    /// Extracts proposal ID and amount from the instruction bytes
    pub fn parse_treasury_instruction(data: &[u8]) -> Result<TreasuryInstruction> {
        // Need at least 8 bytes for discriminator
        if data.len() < 8 {
            return Err(GovernanceError::InvalidInstruction.into());
        }
        
        // Extract instruction discriminator (first 8 bytes)
        let _discriminator = &data[0..8];
        
        // Treasury fund_proposal discriminator: [109, 142, 133, 205, 240, 28, 197, 245]
        // Parse funding instruction parameters
        if data.len() >= 16 {
            let proposal_id = u64::from_le_bytes(
                data[8..16].try_into().map_err(|_| GovernanceError::InvalidInstruction)?
            );
            let amount = if data.len() >= 24 {
                u64::from_le_bytes(
                    data[16..24].try_into().map_err(|_| GovernanceError::InvalidInstruction)?
                )
            } else {
                1000 // Default funding amount
            };
            
            Ok(TreasuryInstruction::FundProposal { proposal_id, amount })
        } else {
            // Fallback to default values if instruction data is incomplete
            Ok(TreasuryInstruction::FundProposal { 
                proposal_id: 0, 
                amount: 1000 
            })
        }
    }

    /// Parse parameter change instruction data
    /// Handles system-wide parameter updates and emergency actions
    pub fn parse_parameter_instruction(data: &[u8]) -> Result<ParameterInstruction> {
        // Need at least 1 byte for parameter type
        if data.len() < 1 {
            return Err(GovernanceError::InvalidInstruction.into());
        }
        
        let param_type = data[0];
        
        match param_type {
            1 => {
                // Emergency action - pause/unpause system
                let pause_system = data.get(1).copied().unwrap_or(1) != 0;
                Ok(ParameterInstruction::EmergencyAction { pause_system })
            },
            _ => {
                // Only emergency actions are currently supported
                Err(GovernanceError::InvalidInstruction.into())
            }
        }
    }
}