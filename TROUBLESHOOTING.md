# 🔧 Troubleshooting Guide - Solana Patent NFT Marketplace

This guide helps you diagnose and fix common issues when building, deploying, and using the Solana Patent NFT Marketplace.

---

## 📋 Table of Contents

1. [Build Errors](#-build-errors)
2. [Deployment Issues](#-deployment-issues)
3. [Transaction Failures](#-transaction-failures)
4. [Account Errors](#-account-errors)
5. [PDA Derivation Issues](#-pda-derivation-issues)
6. [Testing Problems](#-testing-problems)
7. [RPC Connection Issues](#-rpc-connection-issues)
8. [Wallet Integration](#-wallet-integration)

---

## 🔨 Build Errors

### Error: `cargo-build-sbf not found`

**Symptom:**
```
error: no such subcommand: `build-sbf`
```

**Cause:** Solana CLI not installed or outdated

**Solution:**
```bash
# Install/update Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Verify installation
solana --version  # Should be v1.17.0 or higher
cargo-build-sbf --version
```

---

### Error: `anchor: command not found`

**Symptom:**
```
bash: anchor: command not found
```

**Cause:** Anchor CLI not installed

**Solution:**
```bash
# Install Anchor CLI
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest

# Verify installation
anchor --version  # Should be 0.29.0 or higher
```

---

### Error: `error: package `solana-program v1.x.x` cannot be built`

**Symptom:**
```
error: package `solana-program v1.16.0` cannot be built because it requires rustc 1.68.0 or newer
```

**Cause:** Rust version too old

**Solution:**
```bash
# Update Rust
rustup update stable
rustc --version  # Should be 1.68.0 or higher
```

---

### Error: `error: linking with 'rust-lld' failed`

**Symptom:**
```
error: linking with 'rust-lld' failed: exit status: 1
```

**Cause:** Program size exceeds limit or dependency issues

**Solution:**
```bash
# 1. Clean build artifacts
anchor clean
cargo clean

# 2. Rebuild
anchor build

# 3. If still failing, optimize Cargo.toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
```

---

## 🚀 Deployment Issues

### Error: `Insufficient funds for deployment`

**Symptom:**
```
Error: Account <WALLET> has insufficient funds for spend (0.5 SOL) + fee (0.00001 SOL)
```

**Cause:** Wallet doesn't have enough SOL

**Solution:**
```bash
# Check balance
solana balance

# For devnet, request airdrop
solana airdrop 2

# For mainnet, transfer SOL to wallet
# (Use Phantom, Solflare, or exchange)
```

---

### Error: `Program already deployed`

**Symptom:**
```
Error: Program <PROGRAM_ID> is already deployed
```

**Cause:** Trying to deploy with existing program ID

**Solution:**
```bash
# Option 1: Upgrade existing program
anchor upgrade target/deploy/patent_nft.so --program-id <PROGRAM_ID>

# Option 2: Deploy as new program (generates new ID)
solana-keygen new -o target/deploy/patent_nft-keypair.json
anchor deploy
```

---

### Error: `Program not initialized`

**Symptom:**
```
Error: Account <STATE_PDA> does not exist
```

**Cause:** Program deployed but not initialized

**Solution:**
```bash
# Run initialization script
anchor run initialize

# Or manually call initialize instruction
# (See tests/patent-nft.ts for example)
```

---

### Error: `Invalid program ID in Anchor.toml`

**Symptom:**
```
Error: Program ID mismatch. Expected <ID1>, found <ID2>
```

**Cause:** Program ID in Anchor.toml doesn't match deployed program

**Solution:**
```bash
# 1. Get deployed program ID
solana address -k target/deploy/patent_nft-keypair.json

# 2. Update Anchor.toml
[programs.devnet]
patent_nft = "<PROGRAM_ID_FROM_STEP_1>"

# 3. Update lib.rs
declare_id!("<PROGRAM_ID_FROM_STEP_1>");

# 4. Rebuild
anchor build
```

---

## ❌ Transaction Failures

### Error: `Transaction simulation failed: Insufficient funds`

**Symptom:**
```
Error: Transaction simulation failed: Attempt to debit an account but found no record of a prior credit
```

**Cause:** Account doesn't have enough SOL for transaction + rent

**Solution:**
```bash
# Check account balance
solana balance <ACCOUNT_ADDRESS>

# Ensure account has:
# - Transaction fee (~0.0005 SOL)
# - Rent-exempt balance (~0.001-0.01 SOL per account)
# - Any SOL being transferred

# Add more SOL
solana airdrop 1 <ACCOUNT_ADDRESS>  # Devnet only
```

---

### Error: `custom program error: 0x0` (Patent Already Exists)

**Symptom:**
```
Error: failed to send transaction: Transaction simulation failed: Error processing Instruction 0: custom program error: 0x1770
```

**Cause:** Trying to mint a patent that's already been minted

**Solution:**
```typescript
// Check if patent exists before minting
const patentHash = hashPatentNumber(patentNumber);
const [registryPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("patent"), patentHash],
  programId
);

const accountInfo = await connection.getAccountInfo(registryPda);
if (accountInfo) {
  console.log("Patent already minted!");
  return;
}

// Proceed with minting
await sdk.mintPatentNFT(...);
```

---

### Error: `custom program error: 0x1` (Insufficient Payment)

**Symptom:**
```
Error: custom program error: 0x1771
```

**Cause:** Sent less than 0.05 SOL for minting

**Solution:**
```typescript
// Ensure you send at least 0.05 SOL
const mintFee = 0.05 * LAMPORTS_PER_SOL;

await sdk.mintPatentNFT(
  patentNumber,
  name,
  symbol,
  uri,
  // Transaction will include 0.05 SOL transfer
);
```

---

### Error: `Transaction too large`

**Symptom:**
```
Error: Transaction size (1500 bytes) exceeds maximum (1232 bytes)
```

**Cause:** Too many instructions or accounts in one transaction

**Solution:**
```typescript
// Split into multiple transactions
const tx1 = new Transaction().add(instruction1, instruction2);
const tx2 = new Transaction().add(instruction3, instruction4);

await wallet.sendTransaction(tx1, connection);
await wallet.sendTransaction(tx2, connection);

// Or use versioned transactions with lookup tables
```

---

## 🗂️ Account Errors

### Error: `Account not found`

**Symptom:**
```
Error: Account <ADDRESS> not found
```

**Cause:** Account doesn't exist or wrong address

**Solution:**
```typescript
// 1. Verify account address
console.log("Looking for account:", accountAddress.toBase58());

// 2. Check if account exists
const accountInfo = await connection.getAccountInfo(accountAddress);
if (!accountInfo) {
  console.log("Account does not exist. Creating...");
  // Create account or derive correct PDA
}

// 3. For PDAs, verify derivation
const [pda, bump] = PublicKey.findProgramAddressSync(
  [Buffer.from("state")],
  programId
);
console.log("Derived PDA:", pda.toBase58());
```

---

### Error: `Account already in use`

**Symptom:**
```
Error: Account <ADDRESS> already in use
```

**Cause:** Trying to create an account that already exists

**Solution:**
```typescript
// Check if account exists before creating
const accountInfo = await connection.getAccountInfo(accountAddress);

if (accountInfo) {
  console.log("Account already exists, skipping creation");
  return;
}

// Create account
await program.methods.initialize().accounts({ ... }).rpc();
```

---

### Error: `Invalid account data`

**Symptom:**
```
Error: Account data too small for deserialization
```

**Cause:** Account exists but has wrong data or size

**Solution:**
```bash
# 1. Check account data
solana account <ACCOUNT_ADDRESS>

# 2. If wrong, close and recreate
# (Add close_account instruction to program)

# 3. Or deploy new program with correct account size
```

---

## 🔑 PDA Derivation Issues

### Error: `Seeds constraint violated`

**Symptom:**
```
Error: AnchorError caused by account: state. Error Code: ConstraintSeeds. Error Message: A seeds constraint was violated
```

**Cause:** PDA seeds don't match expected derivation

**Solution:**
```typescript
// Ensure seeds match program's derivation
// WRONG:
const [pda] = PublicKey.findProgramAddressSync(
  [Buffer.from("wrong_seed")],
  programId
);

// CORRECT (must match program):
const [pda] = PublicKey.findProgramAddressSync(
  [Buffer.from("state")],  // Matches program's seeds
  programId
);
```

**Check program code:**
```rust
#[account(
    seeds = [b"state"],  // These seeds must match!
    bump,
)]
pub state: Account<'info, State>,
```

---

### Error: `Invalid bump seed`

**Symptom:**
```
Error: Invalid bump seed
```

**Cause:** Using wrong bump value for PDA

**Solution:**
```typescript
// Always use findProgramAddressSync to get correct bump
const [pda, bump] = PublicKey.findProgramAddressSync(seeds, programId);

// Don't hardcode bump values!
// WRONG: const bump = 255;
// RIGHT: Use bump from findProgramAddressSync
```

---

### Error: `PDA would exceed max seed length`

**Symptom:**
```
Error: Max seed length exceeded
```

**Cause:** Seed data too long (max 32 bytes per seed)

**Solution:**
```typescript
// Hash long data before using as seed
import { createHash } from 'crypto';

const patentNumber = "US1234567890123456789012345678901234567890";
const hash = createHash('sha256').update(patentNumber).digest();

const [pda] = PublicKey.findProgramAddressSync(
  [Buffer.from("patent"), hash],  // Hash is 32 bytes
  programId
);
```

---

## 🧪 Testing Problems

### Error: `Test validator failed to start`

**Symptom:**
```
Error: Failed to start test validator
```

**Cause:** Port already in use or previous validator still running

**Solution:**
```bash
# 1. Kill existing validator
pkill -9 solana-test-validator

# 2. Clean test ledger
rm -rf .anchor/test-ledger

# 3. Run tests again
anchor test
```

---

### Error: `Airdrop failed in test`

**Symptom:**
```
Error: Airdrop of 1000000000 lamports failed
```

**Cause:** Test validator not running or rate limit

**Solution:**
```typescript
// Add retry logic for airdrops
async function airdropWithRetry(connection, address, amount, retries = 3) {
  for (let i = 0; i < retries; i++) {
    try {
      const sig = await connection.requestAirdrop(address, amount);
      await connection.confirmTransaction(sig);
      return;
    } catch (error) {
      if (i === retries - 1) throw error;
      await new Promise(resolve => setTimeout(resolve, 1000));
    }
  }
}
```

---

### Error: `Test timeout`

**Symptom:**
```
Error: Timeout of 30000ms exceeded
```

**Cause:** Transaction taking too long or not confirming

**Solution:**
```typescript
// Increase timeout in test
it("Mints patent NFT", async () => {
  // ...
}).timeout(60000);  // 60 seconds

// Or use faster confirmation
const tx = await program.methods.mintPatentNft(...)
  .accounts({ ... })
  .rpc({ commitment: "confirmed" });  // Don't wait for finalized
```

---

### Error: `Account discriminator mismatch`

**Symptom:**
```
Error: Account discriminator mismatch
```

**Cause:** Account has wrong type or wasn't initialized properly

**Solution:**
```typescript
// Ensure account is initialized before fetching
await program.methods.initialize().accounts({ ... }).rpc();

// Then fetch
const state = await program.account.state.fetch(statePda);

// Check account type matches
console.log("Account owner:", accountInfo.owner.toBase58());
console.log("Expected program:", programId.toBase58());
```

---

## 🌐 RPC Connection Issues

### Error: `429 Too Many Requests`

**Symptom:**
```
Error: 429 Too Many Requests
```

**Cause:** Exceeded RPC rate limit

**Solution:**
```typescript
// 1. Use paid RPC provider (QuickNode, Alchemy, Helius)
const connection = new Connection("https://your-rpc-url.com");

// 2. Implement rate limiting
import pLimit from 'p-limit';
const limit = pLimit(5);  // Max 5 concurrent requests

const promises = accounts.map(account =>
  limit(() => connection.getAccountInfo(account))
);
await Promise.all(promises);

// 3. Add retry with exponential backoff
async function fetchWithRetry(fn, retries = 3) {
  for (let i = 0; i < retries; i++) {
    try {
      return await fn();
    } catch (error) {
      if (error.message.includes('429') && i < retries - 1) {
        await new Promise(r => setTimeout(r, 1000 * Math.pow(2, i)));
        continue;
      }
      throw error;
    }
  }
}
```

---

### Error: `Connection timeout`

**Symptom:**
```
Error: Connection timeout
```

**Cause:** RPC node unreachable or slow

**Solution:**
```typescript
// 1. Increase timeout
const connection = new Connection(rpcUrl, {
  commitment: 'confirmed',
  confirmTransactionInitialTimeout: 60000,  // 60 seconds
});

// 2. Use multiple RPC endpoints with fallback
const rpcEndpoints = [
  "https://api.devnet.solana.com",
  "https://devnet.genesysgo.net",
];

async function getConnectionWithFallback() {
  for (const endpoint of rpcEndpoints) {
    try {
      const conn = new Connection(endpoint);
      await conn.getVersion();  // Test connection
      return conn;
    } catch (error) {
      continue;
    }
  }
  throw new Error("All RPC endpoints failed");
}
```

---

### Error: `Blockhash not found`

**Symptom:**
```
Error: Blockhash not found
```

**Cause:** Transaction took too long to send, blockhash expired

**Solution:**
```typescript
// Get fresh blockhash before sending
const { blockhash, lastValidBlockHeight } =
  await connection.getLatestBlockhash('finalized');

transaction.recentBlockhash = blockhash;
transaction.lastValidBlockHeight = lastValidBlockHeight;

// Send immediately
const signature = await wallet.sendTransaction(transaction, connection);

// Confirm with timeout
await connection.confirmTransaction({
  signature,
  blockhash,
  lastValidBlockHeight
}, 'confirmed');
```

---

## 👛 Wallet Integration

### Error: `Wallet not connected`

**Symptom:**
```
Error: Wallet not connected
```

**Cause:** User hasn't connected wallet or wallet disconnected

**Solution:**
```typescript
import { useWallet } from '@solana/wallet-adapter-react';

function MintButton() {
  const { connected, connect, publicKey } = useWallet();

  const handleMint = async () => {
    if (!connected) {
      await connect();
      return;
    }

    // Proceed with minting
    await sdk.mintPatentNFT(...);
  };

  return (
    <button onClick={handleMint}>
      {connected ? 'Mint NFT' : 'Connect Wallet'}
    </button>
  );
}
```

---

### Error: `User rejected the request`

**Symptom:**
```
Error: User rejected the request
```

**Cause:** User clicked "Reject" in wallet popup

**Solution:**
```typescript
try {
  const signature = await wallet.sendTransaction(transaction, connection);
  console.log("Transaction sent:", signature);
} catch (error) {
  if (error.message.includes('User rejected')) {
    console.log("User cancelled transaction");
    // Show friendly message, don't treat as error
    return;
  }
  throw error;  // Re-throw other errors
}
```

---

### Error: `Wallet adapter not found`

**Symptom:**
```
Error: Wallet adapter for 'phantom' not found
```

**Cause:** Wallet not installed or adapter not configured

**Solution:**
```typescript
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import { PhantomWalletAdapter, SolflareWalletAdapter } from '@solana/wallet-adapter-wallets';
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';

const wallets = [
  new PhantomWalletAdapter(),
  new SolflareWalletAdapter(),
];

function App() {
  return (
    <ConnectionProvider endpoint={rpcUrl}>
      <WalletProvider wallets={wallets} autoConnect>
        {/* Your app */}
      </WalletProvider>
    </ConnectionProvider>
  );
}
```

---

## 🔍 Debugging Tips

### Enable Verbose Logging

```bash
# Set environment variable
export RUST_LOG=solana_runtime::system_instruction_processor=trace,solana_runtime::message_processor=debug,solana_bpf_loader=debug,solana_rbpf=debug

# Run with logging
anchor test
```

### Add Custom Logs to Program

```rust
use anchor_lang::prelude::*;

pub fn mint_patent_nft(ctx: Context<MintPatentNFT>, patent_number: String) -> Result<()> {
    msg!("Minting patent: {}", patent_number);
    msg!("Payer: {}", ctx.accounts.payer.key());
    msg!("Mint: {}", ctx.accounts.mint.key());

    // Your logic

    msg!("Mint successful!");
    Ok(())
}
```

### View Transaction Logs

```bash
# Get transaction signature from error
solana confirm <SIGNATURE> -v

# Or view in Solana Explorer
# https://explorer.solana.com/tx/<SIGNATURE>?cluster=devnet
```

### Inspect Account Data

```bash
# View account details
solana account <ACCOUNT_ADDRESS> --output json

# Decode account data (if you know the structure)
anchor account <PROGRAM_NAME>.<ACCOUNT_TYPE> <ACCOUNT_ADDRESS>
```

---

## 📚 Additional Resources

- **Solana Cookbook**: https://solanacookbook.com/
- **Anchor Errors**: https://www.anchor-lang.com/docs/errors
- **Solana Stack Exchange**: https://solana.stackexchange.com/
- **Discord**: Solana Tech Discord, Anchor Discord

---

## 🆘 Still Stuck?

If you're still experiencing issues:

1. **Check the FAQ**: [FAQ_SOLANA.md](./FAQ_SOLANA.md) has 60 Q&A
2. **Review execution flows**: [TEACHME.md](./TEACHME.md) shows detailed flows
3. **Run the quiz**: Test your understanding with `./solana_project_quiz.py`
4. **Search Solana Stack Exchange**: Someone may have had the same issue
5. **Ask in Discord**: Solana and Anchor communities are very helpful

**When asking for help, include:**
- Full error message
- Transaction signature (if applicable)
- Code snippet showing the issue
- What you've already tried

---

**Built with ❤️ for Solana developers**
