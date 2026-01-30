# Quick Start Guide - NFT Patents Solana

Get up and running with the NFT Patents Solana project in 5 minutes!

## Prerequisites

Make sure you have these installed:
- **Rust** (latest stable)
- **Solana CLI** (v1.17+)
- **Anchor CLI** (v0.29+)
- **Node.js** (v18+) and **Yarn**

## Installation

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Install Solana CLI
```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

### 3. Install Anchor CLI
```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

### 4. Clone and Setup Project
```bash
cd NFT_Patents_Solana
yarn install
```

## Build and Test

### Build All Programs
```bash
anchor build
```

This compiles all four Solana programs:
- `patent_nft`
- `psp_token`
- `nft_marketplace`
- `search_payment`

### Run Tests
```bash
anchor test
```

This will:
1. Start a local Solana validator
2. Deploy all programs
3. Run the test suite
4. Shut down the validator

## Local Development

### Start Local Validator
```bash
# Terminal 1
solana-test-validator
```

### Deploy Programs
```bash
# Terminal 2
solana config set --url localhost
anchor deploy --provider.cluster localnet
```

### Get Program IDs
```bash
solana-keygen pubkey target/deploy/patent_nft-keypair.json
solana-keygen pubkey target/deploy/psp_token-keypair.json
solana-keygen pubkey target/deploy/nft_marketplace-keypair.json
solana-keygen pubkey target/deploy/search_payment-keypair.json
```

## Using the SDK

### Basic Setup
```typescript
import { Connection, PublicKey } from '@solana/web3.js';
import { AnchorProvider, Program } from '@coral-xyz/anchor';
import { NFTPatentsSolanaSDK } from './app/sdk';

// Connect to localnet
const connection = new Connection('http://localhost:8899', 'confirmed');

// Set up provider (with wallet)
const provider = new AnchorProvider(connection, wallet, {});

// Load programs
const patentNFTProgram = new Program(patentNFTIdl, patentNFTProgramId, provider);
const pspTokenProgram = new Program(pspTokenIdl, pspTokenProgramId, provider);
const marketplaceProgram = new Program(marketplaceIdl, marketplaceProgramId, provider);
const searchPaymentProgram = new Program(searchPaymentIdl, searchPaymentProgramId, provider);

// Initialize SDK
const sdk = new NFTPatentsSolanaSDK(connection, wallet, {
  patentNFT: patentNFTProgram,
  pspToken: pspTokenProgram,
  marketplace: marketplaceProgram,
  searchPayment: searchPaymentProgram,
});
```

### Mint a Patent NFT
```typescript
const tx = await sdk.patentNFT.mintPatentNFT(
  "US1234567A",
  "My Patent",
  "PAT",
  "https://api.example.com/metadata/US1234567A"
);
console.log("Minted NFT:", tx);
```

### Purchase PSP Tokens
```typescript
import { BN } from '@coral-xyz/anchor';

const tx = await sdk.pspToken.purchaseTokens(
  new BN(100_000_000) // 0.1 SOL
);
console.log("Purchased PSP:", tx);
```

### List NFT on Marketplace
```typescript
const tx = await sdk.marketplace.listNFT(
  nftMintPublicKey,
  new BN(1_000_000_000) // 1 SOL
);
console.log("Listed NFT:", tx);
```

### Pay for Search
```typescript
const tx = await sdk.searchPayment.payWithSOL();
console.log("Paid for search:", tx);
```

## Common Commands

```bash
# Build programs
anchor build

# Run tests
anchor test

# Deploy to localnet
anchor deploy --provider.cluster localnet

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Clean build artifacts
anchor clean

# Check Solana config
solana config get

# Check wallet balance
solana balance

# Airdrop SOL (devnet only)
solana airdrop 2
```

## Project Structure

```
NFT_Patents_Solana/
├── programs/           # Rust programs (smart contracts)
│   ├── patent-nft/
│   ├── psp-token/
│   ├── nft-marketplace/
│   └── search-payment/
├── tests/             # Anchor tests
├── app/               # TypeScript SDK
├── target/            # Build output
└── Anchor.toml        # Anchor config
```

## Troubleshooting

### "anchor: command not found"
```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
```

### "solana: command not found"
```bash
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

### Build errors
```bash
anchor clean
cargo clean
anchor build
```

### Test failures
```bash
# Make sure validator is running
solana-test-validator

# In another terminal
anchor test --skip-local-validator
```

## Next Steps

1. **Read the full README.md** for detailed documentation
2. **Check DEPLOYMENT.md** for deployment instructions
3. **Review MIGRATION_GUIDE.md** if coming from Ethereum
4. **Explore the SDK** in `app/sdk.ts`
5. **Write more tests** in `tests/`

## Resources

- [Solana Docs](https://docs.solana.com/)
- [Anchor Book](https://book.anchor-lang.com/)
- [Metaplex Docs](https://docs.metaplex.com/)
- [Solana Cookbook](https://solanacookbook.com/)

## Support

For issues or questions:
1. Check the README.md
2. Review the MIGRATION_GUIDE.md
3. Consult Solana/Anchor documentation
4. Open an issue on GitHub

---

Happy building on Solana! 🚀

