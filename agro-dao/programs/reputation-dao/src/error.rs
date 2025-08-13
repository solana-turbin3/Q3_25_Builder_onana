use anchor_lang::prelude::*;

#[error_code]
pub enum ReputationError {
    #[msg("Reputation system is not active")]
    SystemNotActive,
    
    #[msg("Unauthorized access - only authority can perform this action")]
    Unauthorized,
    
    #[msg("Invalid reputation score - exceeds bounds")]
    InvalidReputationScore,
    
    #[msg("User reputation account not found")]
    UserReputationNotFound,
    
    #[msg("Invalid tier threshold configuration")]
    InvalidTierThreshold,
    
    #[msg("Reputation score would exceed maximum allowed value")]
    ReputationOverflow,
    
    #[msg("Reputation score would exceed minimum allowed value")]
    ReputationUnderflow,
    
    #[msg("Invalid event type")]
    InvalidEventType,
    
    #[msg("Reputation change amount is zero")]
    ZeroReputationChange,
    
    #[msg("User does not exist or is not initialized")]
    UserNotInitialized,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("Cannot decrease reputation - user already at minimum")]
    AlreadyAtMinimum,
}
