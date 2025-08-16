# AgroDAO - Decentralized Agricultural Research Platform

A comprehensive decentralized autonomous organization (DAO) built on Solana for funding and managing agricultural research projects. AgroDAO connects researchers, funders, and stakeholders through a transparent, governance-driven ecosystem.

## 🌾 What is AgroDAO?

AgroDAO revolutionizes agricultural research funding by creating a decentralized platform where:

- **Researchers** can submit proposals and receive milestone-based funding
- **Funders** can stake tokens to support promising research projects  
- **Community** participates in governance decisions through voting
- **Reputation system** tracks researcher credibility and performance
- **Cross-program architecture** ensures secure, validated interactions

## 🏗️ Architecture Overview

AgroDAO consists of 5 interconnected Anchor programs:

| Program | Purpose | Key Features |
|---------|---------|--------------|
| **AgroDAO** | Main coordinator | Protocol initialization, cross-program management |
| **Research DAO** | Research management | Researcher profiles, proposal creation, validation |
| **Treasury DAO** | Financial operations | Token staking, milestone funding, distribution |
| **Governance DAO** | Voting system | Proposal voting, governance decisions |
| **Reputation DAO** | Credibility tracking | Researcher scoring, performance metrics |

## 🚀 Live Deployment (Devnet)

The system is deployed and operational on Solana Devnet:

```
AgroDAO:       HWjwngNibn1coAzqLZhg4huw5pH5gNZY8zxJaK7s3Hbj
Research DAO:  FUpDQNRZyx2u8uEnerDP9Y6gRT4HUaTZcU7ViziYxWQp  
Treasury DAO:  BT9K4n1w56VP6pL9fAwZesLCJWJ9rmaJ2d3XZxGuGkYB
Governance:    DyazGQj7kUgXoSxcReQk1cDfRQ9QJhQaZRs3C2NXgVMy
Reputation:    WZ13w2w964gyDhpd3GWpFuCJQWYaGNAgybt3rrrUuxD
```

## 🛠️ Quick Start

### Prerequisites

- **Node.js** 18+ 
- **Rust** 1.70+
- **Solana CLI** 1.18+
- **Anchor CLI** 0.31+

### Installation

```bash
# Clone repository
git clone https://github.com/solana-turbin3/Q3_25_Builder_onana.git
cd agro-dao

# Install dependencies
npm install
# or
yarn install

# Build programs
anchor build
```

### Development Setup

1. **Start local validator**:
```bash
solana-test-validator
```

2. **Deploy to local** (optional):
```bash
anchor deploy
```

3. **Run tests**:
```bash
anchor test
```

### Devnet Interaction

The programs are already deployed on devnet. To interact:

```bash
# Configure Solana CLI for devnet
solana config set --url devnet

# Airdrop SOL for testing
solana airdrop 2

# Run tests against devnet deployment
anchor test --skip-deploy
```

## 💡 Core Features

### 🔬 Research Management
- **Researcher Profiles**: Comprehensive researcher registration and verification
- **Proposal System**: Structured research proposal creation with milestones
- **Peer Review**: Community-driven proposal evaluation

### 💰 Treasury & Funding
- **Token Staking**: AGRO token staking to fund research proposals
- **Milestone Funding**: Automated release of funds upon milestone completion
- **Transparent Distribution**: On-chain tracking of all fund movements

### 🗳️ Governance
- **Proposal Voting**: Community governance for protocol decisions
- **Weighted Voting**: Reputation and stake-based voting power
- **Transparent Decisions**: All governance actions recorded on-chain

### ⭐ Reputation System
- **Dynamic Scoring**: Performance-based researcher reputation tracking
- **Milestone Completion**: Reputation increases with successful project delivery
- **Community Trust**: Reputation influences funding opportunities

## 🔧 Technical Implementation

### Cross-Program Communication (CPI)

AgroDAO uses Cross-Program Invocation for secure inter-program communication:

```rust
// Treasury validates proposals through Research DAO
validate_proposal_for_funding_cpi(
    research_dao_program,
    research_proposal,
    researcher_profile, 
    proposal_id,
    amount,
)?;
```

### Key Smart Contract Functions

#### Research DAO
```rust
create_researcher_profile()     // Register as researcher
create_proposal()              // Submit research proposal
validate_proposal_for_funding() // CPI validation for treasury
```

#### Treasury DAO  
```rust
fund_proposal()               // Stake tokens to fund research
distribute_proposal_funds()   // Release milestone payments
deposit_stake_tokens()        // Deposit staking tokens
```

#### Governance DAO
```rust
create_vote()                // Create governance proposal
cast_vote()                  // Vote on proposals
execute_proposal()           // Execute passed proposals
```

## 📊 Program States

### Research Proposal Lifecycle
```
DRAFT → SUBMITTED → UNDER_REVIEW → APPROVED → FUNDED → IN_PROGRESS → COMPLETED
```

### Funding Mechanism
1. **Proposal Creation**: Researcher submits detailed proposal
2. **Community Review**: Stakeholders evaluate proposal merit
3. **Token Staking**: Funders stake AGRO tokens for approved proposals
4. **Milestone Tracking**: Funds released as milestones are completed
5. **Reputation Update**: Researcher reputation adjusted based on performance

## 🧪 Testing

The project includes comprehensive tests covering:

- **Unit Tests**: Individual program functionality
- **Integration Tests**: Cross-program interactions  
- **End-to-End Tests**: Complete user workflows
- **Error Handling**: Edge cases and failure scenarios

```bash
# Run all tests
anchor test

# Run specific test suite
anchor test tests/treasury-dao

# Run with logs
anchor test --skip-deploy -- --nocapture
```

## 🔐 Security Considerations

- **No Private Keys**: Never commit keypairs or private keys
- **CPI Security**: All cross-program calls are validated
- **Access Control**: Role-based permissions throughout
- **Audit Ready**: Code structure supports security audits

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This software is experimental and for educational purposes. It has not been audited. Use at your own risk in production environments.

## 🔗 Resources

- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework](https://anchor-lang.com/)
-

---

**Built with ❤️ for the future of decentralized agricultural research**
