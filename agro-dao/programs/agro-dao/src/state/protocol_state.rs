use anchor_lang::prelude::*;

/// Main protocol state that manages the high-level configuration and counters
/// for the entire Agro DAO ecosystem
#[account]
#[derive(InitSpace)]
pub struct ProtocolState {
    /// Authority that can update protocol parameters (governance multisig)
    pub authority: Pubkey,
    
    /// Unique identifier counter for new research proposals
    pub proposal_id_counter: u64,
    
    /// Minimum amount of funds a research proposal must reach to be viable
    pub min_funding_threshold: u64,
    
    /// Fee structure for research proposal operations
    pub research_proposal_fee: u64,
    
    /// IPFS hash pointing to the central agricultural data index
    pub ipfs_hash_of_agri_data: [u8; 32],
    
    /// Counter tracking the number of agricultural data submissions
    pub research_data_counter: u64,
    
    /// Timestamp of protocol initialization
    pub creation_timestamp: i64,
    
    /// Minimum amount required to be staked for certain actions
    pub minimum_staked_amount: u64,
    
    /// Protocol version for future upgrades
    pub protocol_version: u8,
    
    /// Whether the protocol is currently paused
    pub is_paused: bool,
    pub bump: u8,
    
    /// Reserved space for future parameters
    pub reserved: [u8; 128],
}
