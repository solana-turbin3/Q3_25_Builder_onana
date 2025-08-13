use anchor_lang::prelude::*;

#[error_code]
pub enum TreasuryError {
    #[msg("Treasury is currently paused")]
    TreasuryPaused,
    
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Token not supported")]
    UnsupportedToken,
    
    #[msg("Invalid fee rate")]
    InvalidFeeRate,
    
    #[msg("Invalid reserve ratio")]
    InvalidReserveRatio,
    
    #[msg("Insufficient balance")]
    InsufficientBalance,
    
    #[msg("Insufficient reserves")]
    InsufficientReserves,
    
    #[msg("Invalid amount")]
    InvalidAmount,
    
    #[msg("Proposal not found")]
    ProposalNotFound,
    
    #[msg("Proposal already funded")]
    ProposalAlreadyFunded,
    
    #[msg("Proposal not approved")]
    ProposalNotApproved,
    
    #[msg("Invalid milestone")]
    InvalidMilestone,
    
    #[msg("Milestone already distributed")]
    MilestoneAlreadyDistributed,
    
    #[msg("Researcher not verified")]
    ResearcherNotVerified,
    
    #[msg("Proposal ID too long")]
    ProposalIdTooLong,
    
    #[msg("Distribution amount too large")]
    DistributionAmountTooLarge,
    
    #[msg("Maximum supported tokens reached")]
    MaxSupportedTokensReached,
    
    #[msg("Maximum funding sources reached")]
    MaxFundingSourcesReached,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("Arithmetic underflow")]
    ArithmeticUnderflow,
}