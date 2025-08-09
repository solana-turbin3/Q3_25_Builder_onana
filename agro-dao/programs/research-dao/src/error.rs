use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    // Research errors
    #[msg("Name too long")]
    NameTooLong,
    #[msg("Bio too long")]
    BioTooLong,
    #[msg("Specialization too long")]
    SpecializationTooLong,
    #[msg("Unauthorized researcher")]
    UnauthorizedResearcher,
    #[msg("Title too long")]    
    TitleTooLong,
    #[msg("Description too long")]
    DescriptionTooLong,
    #[msg("Insufficient funding target")]
    InsufficientFundingTarget,
    #[msg("Too many milestones")]
    TooManyMilestones,
    #[msg("Invalid funding deadline")]
    InvalidFundingDeadline,
    #[msg("Invalid proposal status")]
    InvalidProposalStatus,
    #[msg("Funding deadline expired")]
    FundingDeadlineExpired,
    #[msg("Insufficient reputation")]
    InsufficientReputation,
    #[msg("Invalid milestone index")]
    InvalidMilestoneIndex,
    #[msg("Milestone already completed")]
    MilestoneAlreadyCompleted,
    #[msg("Findings already published")]
    FindingsAlreadyPublished,
    #[msg("Researcher already verified")]
    AlreadyVerified,
}
