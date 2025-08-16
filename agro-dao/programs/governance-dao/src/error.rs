use anchor_lang::prelude::*;

#[error_code]
pub enum GovernanceError {
    #[msg("Governance is currently paused")]
    GovernancePaused,
    
    #[msg("Proposal voting period has not ended")]
    VotingPeriodActive,
    
    #[msg("Proposal voting period has ended")]
    VotingPeriodEnded,
    
    #[msg("Proposal has not been approved")]
    ProposalNotApproved,
    
    #[msg("Proposal execution window has expired")]
    ExecutionWindowExpired,
    
    #[msg("Proposal has already been executed")]
    ProposalAlreadyExecuted,
    
    #[msg("Insufficient AGRO tokens to create proposal")]
    InsufficientAgroToPropose,
    
    #[msg("Insufficient AGRO tokens to vote")]
    InsufficientAgroToVote,
    
    #[msg("Proposal title too long")]
    TitleTooLong,
    
    #[msg("Proposal description too long")]
    DescriptionTooLong,
    
    #[msg("Instruction data too long")]
    InstructionDataTooLong,
    
    #[msg("Invalid threshold value")]
    InvalidThreshold,
    
    #[msg("Proposal not found")]
    ProposalNotFound,
    
    #[msg("Vote already cast")]
    VoteAlreadyCast,
    
    #[msg("User has already voted on this proposal")]
    AlreadyVoted,
    
    #[msg("Insufficient quorum to approve proposal")]
    InsufficientQuorum,
    
    #[msg("Proposal did not meet approval threshold")]
    InsufficientApproval,
    
    #[msg("Unauthorized operation")]
    Unauthorized,
    
    #[msg("Invalid voting period")]
    InvalidVotingPeriod,
    
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("Invalid instruction data")]
    InvalidInstruction,
}
