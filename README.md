# 🏛️ Patent NFT Marketplace - Solana Edition

A **Solana-based Web3 application** that demonstrates how patents can be tokenized, traded, and discovered through decentralized technology using Rust and the Anchor framework.

This is a **complete Solana port** of the original Ethereum-based NFT_Patents project, rebuilt from the ground up using:
- **Rust** for on-chain programs
- **Anchor Framework** (v0.29+) for Solana program development
- **Metaplex Token Metadata** for NFT standards
- **SPL Token** for fungible tokens (PSP - Patent Search Pennies)

⚠️ **Disclaimer:** These NFTs are **representations of publicly available patents** and **do not grant legal ownership** of the underlying intellectual property. This marketplace is a **technical proof-of-concept** showcasing how patents *could* be managed on-chain using Solana's high-performance blockchain.

## 🎓 Proof-of-Concept Status

This project is a **demonstration/portfolio project** built to showcase Solana development skills. It includes:

✅ **Fully Implemented**:
- Four complete Solana programs (1,808 lines of Rust)
- Patent NFT minting with Metaplex standard
- On-chain patent registry with duplicate prevention
- SPL Token (PSP) with purchase/redeem functionality
- Escrow-based NFT marketplace with 2.5% platform fees
- Multi-token payment system (SOL/USDC/PSP)
- TypeScript SDK for frontend integration (503 lines)
- Comprehensive test suite
- Complete documentation and deployment guides

🚀 **Performance Advantages Over Ethereum**:
- **400ms transaction finality** vs 15 seconds on Ethereum
- **~$0.001 minting cost** vs $50-200 on Ethereum
- **~$0.0005 trading cost** vs $30-100 on Ethereum
- **Parallel transaction processing** for high throughput
- **Low, predictable fees** regardless of network congestion

👉 For a detailed architectural walkthrough, see [TEACHME.md](./TEACHME.md).
👉 For migration details from Ethereum, see [MIGRATION_GUIDE.md](./MIGRATION_GUIDE.md).

---

## 📋 Table of Contents

* [🎓 Proof-of-Concept Status](#-proof-of-concept-status)
* [🏗️ Architecture Overview](#-architecture-overview)
* [🚀 Key Differences from Ethereum](#-key-differences-from-ethereum-version)
* [📦 Installation](#-installation)
* [🛠️ Prerequisites](#-prerequisites)
* [🚀 Deployment](#-deployment)
* [🧪 Testing](#-testing)
* [📚 Program Usage Examples](#-program-usage-examples)
* [🔑 Program Derived Addresses (PDAs)](#-program-derived-addresses-pdas)
* [📁 Project Structure](#-project-structure)
* [🔒 Security Features](#-security-features)
* [🌐 Integration with Frontend](#-integration-with-frontend)
* [💡 Key Concepts for Ethereum Developers](#-key-concepts-for-ethereum-developers)
* [🔧 Configuration](#-configuration)
* [📊 Cost Comparison](#-cost-comparison)
* [🤝 Contributing](#-contributing)
* [📄 License](#-license)
* [🙏 Acknowledgments](#-acknowledgments)

---

## 🏗️ Architecture Overview

The Solana Patent NFT Marketplace consists of four interconnected programs and a TypeScript SDK for frontend integration.

### System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    FRONTEND LAYER (React + Vite)                │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────────┐ │
│  │   Pages      │  │  TypeScript  │  │     Components         │ │
│  │              │  │     SDK      │  │                        │ │
│  │ - Search     │  │              │  │ - NFT Cards            │ │
│  │ - Mint       │  │ - PatentNFT  │  │ - Wallet Connect       │ │
│  │ - Marketplace│  │ - PSPToken   │  │ - Modals               │ │
│  │ - Profile    │  │ - Marketplace│  │ - Header/Footer        │ │
│  └──────────────┘  │ - Payment    │  └────────────────────────┘ │
│                    └──────────────┘                              │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    SOLANA BLOCKCHAIN LAYER                      │
│                                                                 │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  PATENT NFT PROGRAM (patent_nft)                           │ │
│  │  ├─ Program State (authority, fees, counters)             │ │
│  │  ├─ Patent Registry PDAs (uniqueness tracking)            │ │
│  │  └─ Metaplex NFT Minting (Token Metadata standard)        │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                 │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  PSP TOKEN PROGRAM (psp_token)                             │ │
│  │  ├─ Program State (pricing, supply limits)                │ │
│  │  ├─ SPL Token Mint (Patent Search Pennies)                │ │
│  │  ├─ Token Accounts (user balances)                        │ │
│  │  └─ Spender Authorization PDAs (cross-program calls)      │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                 │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  NFT MARKETPLACE PROGRAM (nft_marketplace)                 │ │
│  │  ├─ Marketplace State (fees, configuration)               │ │
│  │  ├─ Listing PDAs (per-NFT listing data)                   │ │
│  │  └─ Escrow Accounts (NFT custody during sale)             │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                 │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  SEARCH PAYMENT PROGRAM (search_payment)                   │ │
│  │  ├─ Program State (pricing per token type)                │ │
│  │  ├─ User Stats PDAs (payment history tracking)            │ │
│  │  └─ Multi-token Support (SOL, USDC, PSP)                  │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    EXTERNAL SERVICES                            │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────────┐ │
│  │ Google       │  │ IPFS/Pinata  │  │ Solana RPC Nodes       │ │
│  │ Patents API  │  │ (Metadata)   │  │ (Devnet/Mainnet)       │ │
│  └──────────────┘  └──────────────┘  └────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Why This Architecture?

1. **Program Separation**: Each program has a single responsibility (minting, tokens, marketplace, payments)
2. **PDA-Based State**: Deterministic account addresses enable efficient lookups and cross-program calls
3. **Escrow Pattern**: NFTs are held in program-controlled accounts during trading for security
4. **Event Emission**: Programs emit events for off-chain indexing and UI updates
5. **TypeScript SDK**: Abstracts complex account management and transaction building

## 🚀 Key Differences from Ethereum Version

| Feature | Ethereum | Solana |
|---------|----------|--------|
| Smart Contract Language | Solidity | Rust (Anchor) |
| NFT Standard | ERC-721 | Metaplex Token Metadata |
| Token Standard | ERC-20 | SPL Token |
| Transaction Fees | High gas fees | Low transaction costs (~$0.00025) |
| Transaction Speed | ~15 seconds | ~400ms |
| Account Model | Account-based | Account-based with PDAs |
| Deployment | Hardhat | Anchor CLI |

## 📋 Architecture Overview

The Solana Patent NFT Marketplace consists of four main programs:

```
┌─────────────────────────────────────────────────────────┐
│                    Solana Programs                       │
├─────────────────────────────────────────────────────────┤
│  1. patent_nft        - Patent NFT minting (Metaplex)   │
│  2. psp_token         - PSP SPL Token (fungible)        │
│  3. nft_marketplace   - NFT trading with escrow         │
│  4. search_payment    - Multi-token payment system      │
└─────────────────────────────────────────────────────────┘
```

### Program Details

#### 1. **patent_nft** - Patent NFT Program
- Mints unique Patent NFTs using Metaplex Token Metadata standard
- Prevents duplicate minting via patent number normalization
- Supports both paid public minting and admin minting
- Stores patent registry on-chain with PDAs (Program Derived Addresses)
- Configurable minting price and platform fees

**Key Features:**
- Patent uniqueness enforcement
- Metadata URI storage pointing to backend API
- Royalty support via Metaplex
- Withdraw accumulated fees

#### 2. **psp_token** - Patent Search Pennies (PSP) Token
- SPL Token implementation for search payments
- 1 PSP = $0.01 USD equivalent
- Purchase tokens with SOL
- Redeem tokens back to SOL
- Burnable and mintable (admin only)
- Max supply: 10 million PSP
- Authorized spender system for other programs

**Key Features:**
- Token purchase/redemption
- Pausable for emergencies
- Authorized spender mechanism
- Price updates

#### 3. **nft_marketplace** - NFT Marketplace
- List Patent NFTs for sale
- Buy NFTs with SOL
- Escrow-based trading (NFTs held in PDA)
- Platform fee distribution (default 2.5%)
- Cancel listings
- Update listing prices

**Key Features:**
- Secure escrow system
- Automatic fee distribution
- Active listing tracking
- Pull-based payment pattern

#### 4. **search_payment** - Multi-Token Payment System
- Accept payments in SOL, USDC, or PSP tokens
- Track user payment statistics
- Allocate search credits
- Configurable pricing per token
- Pausable contract

**Key Features:**
- Multi-token support (SOL/USDC/PSP)
- User statistics tracking
- Event emission for backend processing
- Admin controls

## 🛠️ Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (latest stable): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Solana CLI** (v1.17+): `sh -c "$(curl -sSfL https://release.solana.com/stable/install)"`
- **Anchor CLI** (v0.29+): `cargo install --git https://github.com/coral-xyz/anchor avm --locked --force`
- **Node.js** (v18+) and **Yarn**: For testing and client SDK
- **Phantom Wallet** or **Solflare**: For browser interaction

## 📦 Installation

```bash
# Clone the repository
cd NFT_Patents_Solana

# Install dependencies
yarn install

# Build all programs
anchor build

# Run tests
anchor test
```

## 🚀 Deployment

### Local Development (Localnet)

```bash
# Terminal 1: Start local validator
solana-test-validator

# Terminal 2: Deploy programs
anchor deploy --provider.cluster localnet

# Verify deployment
solana program show <PROGRAM_ID>
```

### Devnet Deployment

```bash
# Configure Solana CLI for devnet
solana config set --url devnet

# Airdrop SOL for deployment (if needed)
solana airdrop 2

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

### Mainnet Deployment

```bash
# Configure for mainnet
solana config set --url mainnet-beta

# Deploy (ensure you have sufficient SOL)
anchor deploy --provider.cluster mainnet
```

## 🧪 Testing

```bash
# Run all tests
anchor test

# Run unit tests only
cargo test

# Run with verbose output
anchor test -- --nocapture
```

## 📚 Program Usage Examples

### Initialize Patent NFT Program

```typescript
await program.methods
  .initialize(
    new BN(50_000_000), // 0.05 SOL minting price
    250 // 2.5% platform fee
  )
  .accounts({
    state: statePDA,
    authority: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Mint a Patent NFT

```typescript
await program.methods
  .mintPatentNft(
    "US1234567A",
    "Patent US1234567A",
    "PAT",
    "https://api.example.com/metadata/US1234567A"
  )
  .accounts({
    state: statePDA,
    patentRegistry: patentRegistryPDA,
    payer: wallet.publicKey,
    // ... other accounts
  })
  .rpc();
```

### List NFT on Marketplace

```typescript
await program.methods
  .listNft(new BN(1_000_000_000)) // 1 SOL price
  .accounts({
    state: marketplaceStatePDA,
    listing: listingPDA,
    nftMint: nftMint,
    seller: wallet.publicKey,
    sellerNftAccount: sellerNftATA,
    escrowNftAccount: escrowNftATA,
    tokenProgram: TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
    rent: SYSVAR_RENT_PUBKEY,
  })
  .rpc();
```

### Purchase PSP Tokens

```typescript
await program.methods
  .purchaseTokens(new BN(100_000_000)) // 0.1 SOL
  .accounts({
    state: pspStatePDA,
    mint: pspMint,
    buyer: wallet.publicKey,
    buyerTokenAccount: buyerPspATA,
    tokenProgram: TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

## 🔑 Program Derived Addresses (PDAs)

PDAs are deterministic addresses derived from seeds:

| Program | PDA Purpose | Seeds |
|---------|-------------|-------|
| patent_nft | Program State | `["state"]` |
| patent_nft | Patent Registry | `["patent", patent_hash]` |
| psp_token | Program State | `["state"]` |
| psp_token | Spender Authorization | `["spender", spender_pubkey]` |
| nft_marketplace | Marketplace State | `["marketplace"]` |
| nft_marketplace | Listing | `["listing", nft_mint]` |
| search_payment | Program State | `["state"]` |
| search_payment | User Stats | `["user_stats", user_pubkey]` |

## 📁 Project Structure

```
NFT_Patents_Solana/
├── programs/
│   ├── patent-nft/          # Patent NFT minting program
│   ├── psp-token/           # PSP SPL Token program
│   ├── nft-marketplace/     # NFT marketplace program
│   └── search-payment/      # Multi-token payment program
├── tests/                   # Anchor integration tests
├── app/                     # TypeScript client SDK
├── Anchor.toml             # Anchor configuration
├── Cargo.toml              # Rust workspace config
└── package.json            # Node.js dependencies
```

## 🔒 Security Features

- **Reentrancy Protection**: Anchor's account validation
- **Access Control**: `has_one` constraints
- **Pausable Contracts**: Emergency pause
- **Input Validation**: Comprehensive checks
- **PDA Verification**: Automatic derivation
- **Escrow System**: Program-controlled accounts
- **Math Overflow Protection**: Checked arithmetic

## 💡 Key Concepts for Ethereum Developers

| Concept | Ethereum | Solana |
|---------|----------|--------|
| Data Storage | Contract storage | Accounts |
| Address Generation | CREATE2 | PDAs |
| Inheritance | Yes | No (use traits) |
| Account Passing | Implicit | Explicit |
| Events | Stored on-chain | Emitted only |
| Execution | Sequential | Parallel capable |

## 📊 Cost Comparison

| Operation | Ethereum | Solana |
|-----------|----------|--------|
| Mint NFT | ~$50-200 | ~$0.001 |
| List NFT | ~$20-80 | ~$0.0005 |
| Buy NFT | ~$30-100 | ~$0.0005 |
| Token Transfer | ~$5-20 | ~$0.00025 |

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Run tests: `anchor test`
4. Submit a pull request

## 📄 License

MIT License

## 🙏 Acknowledgments

- Anchor Framework
- Metaplex Foundation
- Solana Foundation
- Original NFT_Patents (Ethereum)

---

**Built with ❤️ for Solana**
