use anchor_lang::prelude::*;

#[error_code]
pub enum GovernanceError {
    #[msg("Governance is currently paused")]
    GovernancePaused,
    
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Invalid voting period")]
    InvalidVotingPeriod,
    
    #[msg("Invalid quorum threshold")]
    InvalidQuorumThreshold,
    
    #[msg("Invalid approval threshold")]
    InvalidApprovalThreshold,
    
    #[msg("Invalid reputation weight")]
    InvalidReputationWeight,
    
    #[msg("Insufficient AGRO tokens to propose")]
    InsufficientAgroToPropose,
    
    #[msg("Insufficient AGRO tokens to vote")]
    InsufficientAgroToVote,
    
    #[msg("Proposal not found")]
    ProposalNotFound,
    
    #[msg("Proposal not active")]
    ProposalNotActive,
    
    #[msg("Voting period has not started")]
    VotingPeriodNotStarted,
    
    #[msg("Voting period has ended")]
    VotingPeriodEnded,
    
    #[msg("Voting period is still active")]
    VotingPeriodActive,
    
    #[msg("Already voted on this proposal")]
    AlreadyVoted,
    
    #[msg("Proposal has not passed")]
    ProposalNotPassed,
    
    #[msg("Proposal not ready for execution")]
    NotReadyForExecution,
    
    #[msg("Proposal already executed")]
    ProposalAlreadyExecuted,
    
    #[msg("Execution delay not met")]
    ExecutionDelayNotMet,
    
    #[msg("Quorum not reached")]
    QuorumNotReached,
    
    #[msg("Unsupported proposal type")]
    UnsupportedProposalType,
    
    #[msg("Invalid proposal parameters")]
    InvalidProposalParameters,
    
    #[msg("Title too long")]
    TitleTooLong,
    
    #[msg("Description too long")]
    DescriptionTooLong,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("Arithmetic underflow")]
    ArithmeticUnderflow,
    
    #[msg("Invalid timestamp")]
    InvalidTimestamp,
    
    #[msg("Cross-program invocation failed")]
    CpiCallFailed,
    
    #[msg("Proposal already tallied")]
    ProposalAlreadyTallied,
    
    #[msg("Voting period not ended")]
    VotingPeriodNotEnded,
    
    #[msg("Proposal not approved")]
    ProposalNotApproved,
    
    #[msg("Execution not yet available")]
    ExecutionNotYetAvailable,
    
    #[msg("Execution window expired")]
    ExecutionWindowExpired,
    
    #[msg("Execution failed")]
    ExecutionFailed,
    
    #[msg("No instruction data provided")]
    NoInstructionData,
    
    #[msg("Invalid parameter")]
    InvalidParameter,
    
    #[msg("Invalid emergency action")]
    InvalidEmergencyAction,
    
    #[msg("Unauthorized governance update")]
    UnauthorizedGovernanceUpdate,
}
