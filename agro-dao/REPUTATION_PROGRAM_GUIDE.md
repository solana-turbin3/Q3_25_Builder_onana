# Reputation Program Documentation

## Overview

The Reputation Program is a utility program within the AgroDAO ecosystem that manages researcher reputation scores and tier classifications. It is designed to be called by other programs via Cross-Program Invocation (CPI).

## Key Features

### 1. Modular Architecture
- **State Management**: Centralized reputation configuration and user reputation accounts
- **Error Handling**: Comprehensive error types for validation and bounds checking
- **Event Emission**: All reputation changes emit events for tracking and auditing

### 2. Core Instructions

#### `initialize_reputation_config`
- **Purpose**: Initialize the global reputation system configuration
- **Parameters**: 
  - Optional tier thresholds (Bronze, Silver, Gold, Platinum, Diamond)
  - Uses defaults if not specified
- **Authority**: System admin/deployer
- **Usage**: Called once during system deployment

#### `initialize_user_reputation`
- **Purpose**: Create a reputation account for a new user/researcher
- **Parameters**: User's public key
- **Initial Values**: 0 reputation score, None tier
- **Usage**: Called when a researcher first joins the platform

#### `update_reputation`
- **Purpose**: Update a user's reputation score (main CPI endpoint)
- **Parameters**:
  - `user`: Public key of the user
  - `event_type`: Type of reputation event (see ReputationEvent enum)
  - `custom_amount`: Optional custom amount for Custom event type
- **Authority**: Any program that calls via CPI
- **Usage**: Called by Research Management, Governance, and other programs

#### `decrease_reputation_on_failure`
- **Purpose**: Specific instruction for handling researcher failures
- **Parameters**:
  - `user`: Public key of the user
  - `failure_type`: Type of failure (MissedDeadline, ProjectAbandonment, etc.)
  - `custom_penalty`: Optional custom penalty amount
- **Authority**: Research Management Program (typically)
- **Usage**: Called when researchers fail to meet obligations

#### `get_reputation` (Read-only)
- **Purpose**: Retrieve user's reputation data
- **Returns**: ReputationData struct with score, tier, events count, etc.
- **Usage**: Query endpoint for displaying reputation information

#### `get_tier_info` (Read-only)
- **Purpose**: Get tier threshold configuration
- **Returns**: TierInfo struct with all tier thresholds
- **Usage**: For UI display and tier eligibility checking

#### `check_tier_eligibility` (Read-only)
- **Purpose**: Check if a user qualifies for a specific tier
- **Parameters**: User public key and target tier
- **Returns**: Boolean indicating eligibility
- **Usage**: Validation before granting tier-based privileges

## State Structures

### ReputationConfig
```rust
pub struct ReputationConfig {
    pub bump: u8,
    pub authority: Pubkey,
    pub tier_threshold_bronze: u64,
    pub tier_threshold_silver: u64,
    pub tier_threshold_gold: u64,
    pub tier_threshold_platinum: u64,
    pub tier_threshold_diamond: u64,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
}
```

### UserReputation
```rust
pub struct UserReputation {
    pub bump: u8,
    pub user: Pubkey,
    pub reputation_score: i64,     // Range: -1000 to 10000
    pub events_count: u64,
    pub last_event_ts: i64,
    pub created_at: i64,
    pub tier: ReputationTier,
}
```

## Reputation Events and Scoring

### ReputationEvent Enum
- **MilestoneCompleted**: +100 points
- **MilestoneFailed**: -50 points
- **ProjectCompleted**: +200 points
- **ProjectAbandoned**: -100 points
- **PeerReviewPositive**: +25 points
- **DisputeResolved**: -75 points
- **Custom(i64)**: Variable amount

### Tier System
- **None**: < Bronze threshold (or negative score)
- **Bronze**: 100+ points (default)
- **Silver**: 500+ points (default)
- **Gold**: 1500+ points (default)
- **Platinum**: 3000+ points (default)
- **Diamond**: 5000+ points (default)

## Integration with Other Programs

### Research Management Program
```rust
// Example: When a milestone is completed
let cpi_accounts = reputation_dao::cpi::accounts::UpdateReputation {
    reputation_config: ctx.accounts.reputation_config.to_account_info(),
    user_reputation: ctx.accounts.user_reputation.to_account_info(),
    authority: ctx.accounts.research_program.to_account_info(),
};

let cpi_ctx = CpiContext::new_with_signer(
    ctx.accounts.reputation_program.to_account_info(),
    cpi_accounts,
    &[&research_program_seeds],
);

reputation_dao::cpi::update_reputation(
    cpi_ctx,
    researcher_pubkey,
    ReputationEvent::MilestoneCompleted,
    None,
)?;
```

### Governance Program
- Can call `update_reputation` when governance events occur
- Can use `get_reputation` to validate voting eligibility based on tier
- Can implement tier-based voting weights

### Treasury Program
- Can call `get_reputation` to validate funding eligibility
- Can implement tier-based funding limits or priorities

## Account Derivation

### Seeds
- **Reputation Config**: `["reputation_config"]`
- **User Reputation**: `["user_reputation", user_pubkey]`

### Program ID
```
HtzmTdZL8j5VSSDMSPYpwvHZLNCbP2b27KNxHzHi52Bw
```

## Error Handling

The program includes comprehensive error handling for:
- System not active
- Unauthorized access
- Invalid reputation scores (bounds checking)
- User not found/initialized
- Invalid tier thresholds
- Arithmetic overflow/underflow

## Best Practices for Integration

1. **Always check reputation bounds** before calling update functions
2. **Use specific event types** rather than Custom when possible
3. **Handle errors gracefully** in calling programs
4. **Initialize user reputation** before first reputation update
5. **Use tier-based logic** for feature access control
6. **Emit additional events** in calling programs for complete audit trail

## Future Enhancements

- **Time-based reputation decay**: Reduce reputation over time without activity
- **Peer review system**: Allow researchers to review each other
- **Achievement system**: Special reputation bonuses for milestones
- **Dispute resolution**: Formal process for reputation disputes
- **Reputation staking**: Allow researchers to stake reputation on outcomes

This reputation system provides a robust foundation for managing researcher credibility and incentives within the AgroDAO ecosystem.
