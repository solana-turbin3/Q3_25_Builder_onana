use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ResearcherProfile {
    pub researcher: Pubkey,
    #[max_len(50)]  
    pub name: String,
    #[max_len(200)]
    pub bio: String,
    #[max_len(100)]
    pub specialization: String,
    pub reputation_score: u64,
    pub total_proposals: u32,
    pub completed_projects: u32,
    pub total_funding_received: u64,
    pub creation_timestamp: i64,
    pub is_verified: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct ResearchProposal {
    pub id: u64,
    pub researcher: Pubkey,
    #[max_len(100)]
    pub title: String,
    #[max_len(500)]
    pub description: String,
    pub category: ResearchCategory,
    pub funding_target: u64,
    pub current_funding: u64,
    pub status: ProposalStatus,
    #[max_len(10)]
    pub milestones: Vec<Milestone>,
    pub creation_timestamp: i64,
    pub funding_deadline: i64,
    pub ipfs_hash: [u8; 32],
    pub findings_ipfs_hash: Option<[u8; 32]>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Milestone {
    #[max_len(200)]
    pub description: String,
    pub target_date: i64,
    pub completion_date: Option<i64>,
    pub is_completed: bool,
    pub ipfs_evidence_hash: Option<[u8; 32]>,
}   

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum ResearchCategory {
    CropImprovement,    
    SustainableFarming,
    PestControl,
    SoilHealth,
    ClimateAdaptation,
    WaterManagement,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq)]
pub enum ProposalStatus {   
    Draft,
    SubmittedForFunding,
    FundingActive,
    Funded,
    InProgress,
    Completed,
    Cancelled,
}
