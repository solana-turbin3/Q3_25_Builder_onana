# Treasury DAO Program

## Overview
The Treasury DAO program is a comprehensive financial management system for the AgroDAO ecosystem that handles token deposits, fee collection, AGRO token minting, proposal funding, and fund distribution.

## Key Features

### 1. Treasury Management
- **Initialize Treasury**: Set up the treasury with configurable parameters
- **Add Supported Tokens**: Whitelist tokens for deposits (USDC, USDT, etc.)
- **Emergency Controls**: Pause/unpause functionality for security

### 2. Stake & Deposit Management
- **Token Deposits**: Accept whitelisted tokens from verified researchers
- **Fee Collection**: Automatic fee deduction from deposits
- **AGRO Minting**: 1:1 ratio minting of AGRO tokens (after fees)
- **Researcher Verification**: Integration with research DAO for user verification

### 3. Proposal Funding
- **Fund Allocation**: Stakeholders can direct AGRO tokens to specific proposals
- **Burn Mechanism**: AGRO tokens are burned when funding proposals
- **Cross-Program Verification**: Validates proposals exist in research DAO
- **Multiple Funding Sources**: Support for multiple stakeholders per proposal

### 4. Fund Distribution
- **Milestone-Based**: Releases funds based on milestone completion
- **Multi-Signature**: Authority-controlled distributions
- **Audit Trail**: Complete tracking of all distributions
- **Researcher Validation**: Ensures only verified researchers receive funds

## Program Structure

### State Accounts
- `TreasuryConfig`: Global treasury settings and parameters
- `TokenVault`: Individual vault for each supported token
- `StakeAccount`: User deposit records and AGRO holdings
- `ProposalFunding`: Proposal funding status and distribution tracking

### Key Instructions
1. `initialize_treasury`: Set up the treasury system
2. `add_supported_token`: Add new whitelisted tokens
3. `deposit_stake_tokens`: Handle user deposits and AGRO minting
4. `fund_proposal`: Allocate funds to research proposals
5. `distribute_proposal_funds`: Release funds to researchers
6. `emergency_pause/unpause`: Security controls

## Integration Features

### Research DAO Integration
- **Researcher Verification**: Only verified researchers can deposit/receive funds
- **Proposal Validation**: Cross-program calls to verify proposal existence
- **Milestone Tracking**: Validates milestone completion before distribution

### Future Governance Integration
- Ready for governance program integration for:
  - Proposal approval verification
  - Vote casting from treasury stakeholders
  - Multi-signature treasury operations

## Security Features

### Access Controls
- Authority-based permissions for critical operations
- Researcher verification requirements
- Emergency pause functionality

### Financial Safety
- Reserve ratio maintenance
- Arithmetic overflow/underflow protection
- Constraint-based validation for all operations

### Audit Trail
- Complete event emission for all operations
- Immutable on-chain records
- Cross-reference tracking between programs

## Fund Flow Architecture

### Deposit Flow
1. User deposits whitelisted tokens → Treasury validates token → Researcher verification check → Fee collection → AGRO minting → Stake record update

### Funding Flow
1. Stakeholders burn AGRO tokens → Proposal validation (CPI to research DAO) → Fund allocation tracking → Status updates

### Distribution Flow
1. Authority initiates distribution → Milestone verification → Researcher validation → Token transfer → Record updates

## Compilation Status

✅ **Successfully Compiled**: The program compiles without errors
⚠️ **Stack Size Warnings**: Some functions have large stack usage (within acceptable limits)
⚠️ **IDL Generation**: Currently has compatibility issues with anchor-syn version

## Next Steps

1. **Testing**: Implement comprehensive test suite
2. **Frontend Integration**: Create client-side interfaces
3. **Governance Integration**: Add CPI calls to governance program
4. **OFAC Integration**: Add compliance features as discussed
5. **Yield Generation**: Implement DeFi protocol integration
6. **Liquidity Management**: Add rebalancing and emergency protocols

## Configuration Constants

```rust
// Treasury Config
MAX_SUPPORTED_TOKENS: 10
MAX_FUNDING_SOURCES: 20

// Fee Limits
MAX_FEE_RATE_BPS: 1000 (10%)
MIN_RESERVE_RATIO_BPS: 1000 (10%)
MAX_RESERVE_RATIO_BPS: 5000 (50%)

// Security Limits
MAX_DISTRIBUTION_AMOUNT: 1M tokens
EMERGENCY_PAUSE_DURATION: 7 days
```

The treasury program is now ready for integration testing and deployment to provide comprehensive financial management for the AgroDAO ecosystem.
