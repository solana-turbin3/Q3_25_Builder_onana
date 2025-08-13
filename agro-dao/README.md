# Agricultural DAO - Solana Programs

A decentralized autonomous organization (DAO) for agricultural research funding and management on Solana.

## 🌾 Overview

This project implements a multi-program DAO system consisting of:

- **Research DAO**: Manages researcher profiles and research proposals
- **Treasury DAO**: Handles funding, token staking, and distribution
- **Agro DAO**: Main coordination program
- **Governance DAO**: Voting and governance mechanisms
- **Reputation DAO**: Reputation scoring system

## 🚀 Deployed Programs (Devnet)

- **Research DAO**: `DF1y7PHHo7ekNEKztCMTDsZ3TrYdLAhgBCFQPzoi3PHw`
- **Treasury DAO**: `9cDozwVvb4EHtzVwtbseAkuWRXjPfSmhuoiLiVK8yMY8`
- **Agro DAO**: `5tBzz5Fi88JKejLdDx9XsWHyvwdMimQjvShwpR2mgKgU`

## 🛠 Setup Instructions

### Prerequisites

- Node.js 16+
- Rust 1.70+
- Solana CLI 1.16+
- Anchor CLI 0.29+

### Installation

```bash
# Clone the repository
git clone <your-repo-url>
cd agro-dao

# Install dependencies
yarn install

# Build programs
anchor build
```

### Key Generation (IMPORTANT)

⚠️ **Never commit private keys to version control!**

Generate your own keypairs for deployment:

```bash
# Generate keypairs for each program
solana-keygen new --outfile governance-keypair.json
solana-keygen new --outfile reputation-keypair.json
solana-keygen new --outfile treasury-keypair.json

# Update Anchor.toml with your program IDs
solana address -k governance-keypair.json
solana address -k reputation-keypair.json
solana address -k treasury-keypair.json
```



```

## 🏗 Architecture

### Cross-Program Communication

The treasury DAO uses a **pure CPI approach** to interact with the research DAO, eliminating struct duplication:

```rust
// Treasury DAO validates proposals via research DAO CPI
validate_proposal_for_funding_cpi(
    research_dao_program,
    research_proposal,
    researcher_profile,
    proposal_id,
    amount,
)?;
```

### Key Features

- **Milestone-based funding**: Researchers receive funding as they complete milestones
- **Token staking**: Stakeholders burn AGRO tokens to fund proposals
- **Cross-program validation**: Treasury validates research data through CPI
- **Reputation system**: Track researcher performance and credibility

## 📜 Smart Contract Functions

### Research DAO
- `create_researcher_profile()`: Register as a researcher
- `create_proposal()`: Submit research proposals
- `validate_proposal_for_funding()`: CPI validation for treasury

### Treasury DAO
- `fund_proposal()`: Stake tokens to fund research
- `distribute_proposal_funds()`: Release milestone payments
- `deposit_stake_tokens()`: Deposit tokens for staking


```




## 📄 License

MIT License - see LICENSE file for details.

## ⚠️ Disclaimer

This is experimental software for educational purposes. Use at your own risk.
