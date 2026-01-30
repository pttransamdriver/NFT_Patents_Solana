# ❓ Frequently Asked Questions - Solana Edition

Comprehensive FAQ covering legal, technical, business, and Solana-specific questions about the Patent NFT Marketplace.

---

## 📋 Table of Contents

1. [Legal & Intellectual Property](#-legal--intellectual-property)
2. [Business Model & Economics](#-business-model--economics)
3. [Technical Architecture (Solana)](#-technical-architecture-solana)
4. [Security & Safety](#-security--safety)
5. [Deployment & Operations](#-deployment--operations)
6. [Ethereum vs Solana](#-ethereum-vs-solana)
7. [Development & Integration](#-development--integration)

---

## ⚖️ Legal & Intellectual Property

### Q1: Does buying a patent NFT give me legal ownership of the patent?

**A:** **NO.** The NFT is a **digital representation** of publicly available patent information. It does **NOT** grant:
- Legal ownership of the patent
- Rights to use the patented invention
- Rights to license or enforce the patent
- Any intellectual property rights

The actual patent rights remain with the original patent holder as registered with the patent office (USPTO, EPO, etc.).

### Q2: What legal rights DO I get when I buy a patent NFT?

**A:** You get:
- **Ownership of the NFT itself** (digital collectible)
- **Right to resell** the NFT on the marketplace
- **Access to curated patent information** stored in the NFT metadata
- **Proof of ownership** on the Solana blockchain

Think of it like owning a baseball card of a famous player - you own the card, not the player.

### Q3: Can I be sued for minting or trading patent NFTs?

**A:** The risk is **very low** because:
- Patents are **public information** (anyone can access them)
- You're not claiming ownership of the invention
- You're not using the patented technology
- You're creating a digital collectible, not infringing on IP

However, this is a **proof-of-concept** project. Consult a lawyer before commercial deployment.

### Q4: What if the patent expires or is invalidated?

**A:** The NFT remains valid as a **historical record**. The metadata can be updated to reflect the patent's current status. The NFT's value may change based on the patent's legal status.

### Q5: Can I mint an NFT for any patent in the world?

**A:** Technically yes, but:
- The program currently uses Google Patents API (covers USPTO, EPO, WIPO, etc.)
- Each patent can only be minted **once** (enforced by on-chain registry)
- You must pay the minting fee (0.05 SOL)

---

## 💰 Business Model & Economics

### Q6: How does the platform make money?

**A:** Three revenue streams:
1. **Minting fees**: 0.05 SOL per NFT (~$5-10 depending on SOL price)
2. **Marketplace fees**: 2.5% of each sale
3. **Search fees**: Users pay in SOL/USDC/PSP for AI-powered patent searches

### Q7: Why is Solana so much cheaper than Ethereum?

**A:** Solana's architecture enables:
- **Parallel transaction processing** (vs sequential on Ethereum)
- **Proof of History** consensus (more efficient than Proof of Work/Stake)
- **Fixed fee structure** (not auction-based like Ethereum gas)
- **Optimized runtime** (Sealevel vs EVM)

**Cost Comparison**:
| Operation | Ethereum | Solana | Savings |
|-----------|----------|--------|---------|
| Mint NFT | $50-200 | $0.001 | 99.999% |
| List NFT | $20-80 | $0.0005 | 99.999% |
| Buy NFT | $30-100 | $0.0005 | 99.999% |

### Q8: What is PSP (Patent Search Pennies)?

**A:** PSP is an **SPL token** (Solana's equivalent of ERC-20) used for:
- Paying for AI patent searches
- Discounted marketplace fees (future feature)
- Governance/voting (future feature)

**Tokenomics**:
- Max supply: 10,000,000 PSP
- Exchange rate: 1 SOL = 1,000 PSP (configurable)
- Burnable: Yes (deflationary)
- Mintable: Only by program authority

### Q9: How are platform fees distributed?

**A:** Fees go to the `fee_recipient` account set in the marketplace state. This can be:
- A treasury wallet
- A DAO (future)
- A multi-sig wallet for security

Fees can be withdrawn by the program authority using the `withdraw_fees` instruction.

### Q10: What happens to fees collected in the programs?

**A:** Fees accumulate in program-controlled accounts and can be withdrawn by the authority. The programs track:
- Total fees collected
- Fees per token type (SOL, USDC, PSP)
- Withdrawal history (via events)

---

## 🏗️ Technical Architecture (Solana)

### Q11: What are Program Derived Addresses (PDAs)?

**A:** PDAs are **deterministic addresses** derived from seeds without private keys. They enable:
- **Predictable account locations** (no need to store addresses)
- **Program signing** (programs can sign for PDAs)
- **Efficient lookups** (derive address from seeds)

**Example**:
```rust
let (state_pda, bump) = Pubkey::find_program_address(
    &[b"state"],
    program_id
);
```

### Q12: How does the patent uniqueness check work?

**A:** Each patent gets a **registry PDA** derived from its hash:

```rust
let patent_hash = hash(normalize(patent_number));
let (registry_pda, bump) = Pubkey::find_program_address(
    &[b"patent", patent_hash.as_ref()],
    program_id
);
```

If the account already exists, minting fails. This ensures **global uniqueness** across all users.

### Q13: What is the Metaplex Token Metadata standard?

**A:** Metaplex is Solana's **NFT standard** (like ERC-721 on Ethereum). It provides:
- **Metadata accounts** (name, symbol, URI)
- **Master editions** (limited supply NFTs)
- **Creator verification** (provenance)
- **Royalties** (creator fees on secondary sales)

Our program uses Metaplex for NFT compatibility with wallets and marketplaces.

### Q14: How does the escrow system work?

**A:** When listing an NFT:
1. **Listing PDA** is created with price and seller info
2. **NFT is transferred** from seller to escrow (ATA owned by listing PDA)
3. **Program controls** the NFT (seller can't transfer it)
4. **On purchase**: NFT transfers from escrow to buyer
5. **On cancel**: NFT returns to seller

This prevents sellers from listing and selling elsewhere simultaneously.

### Q15: What is Cross-Program Invocation (CPI)?

**A:** CPI allows programs to call other programs. Examples:
- **Token transfers**: Call SPL Token program
- **NFT minting**: Call Metaplex program
- **System operations**: Call System program

**Example**:
```rust
let cpi_ctx = CpiContext::new_with_signer(
    ctx.accounts.token_program.to_account_info(),
    Transfer { from, to, authority },
    &[&[b"listing", nft_mint.as_ref(), &[bump]]]
);
token::transfer(cpi_ctx, 1)?;
```

---

## 🔒 Security & Safety

### Q16: How are reentrancy attacks prevented?

**A:** Solana's architecture prevents reentrancy:
- **Explicit account passing** (no hidden state access)
- **Borrow checker** (Rust prevents multiple mutable borrows)
- **Anchor framework** (automatic checks and validations)

Unlike Ethereum, you don't need `nonReentrant` modifiers.

### Q17: What access controls are in place?

**A:** Multiple layers:
- **Signer checks**: `#[account(signer)]` ensures only authorized users
- **Ownership checks**: `#[account(has_one = authority)]` verifies account ownership
- **PDA validation**: Anchor verifies PDA derivation
- **Authority-only functions**: Only program authority can pause, update fees, etc.

### Q18: Can the program be paused in an emergency?

**A:** Yes! The PSP token and search payment programs have `pause()` and `unpause()` instructions:

```rust
pub fn pause(ctx: Context<Pause>) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.paused = true;
    Ok(())
}
```

When paused, critical functions (purchase, redeem, pay) are disabled.

### Q19: How is overflow/underflow prevented?

**A:** Rust's type system and Anchor's math operations:
- **Checked arithmetic**: `checked_add()`, `checked_sub()`, `checked_mul()`
- **Compile-time checks**: Rust prevents integer overflow in debug mode
- **Anchor's `require!` macro**: Runtime validation

**Example**:
```rust
let total = price.checked_mul(quantity)
    .ok_or(ErrorCode::Overflow)?;
```

### Q20: What happens if IPFS goes down?

**A:** The NFT metadata URI points to IPFS (via Pinata). If IPFS is unavailable:
- **NFT ownership** remains intact (on-chain)
- **Metadata** (name, image) may not load in wallets
- **Solution**: Use multiple IPFS gateways or Arweave for permanent storage

The on-chain data (patent number, owner, etc.) is always available.

---

## 🚀 Deployment & Operations

### Q21: How do I deploy to Solana devnet?

**A:** Follow these steps:

```bash
# 1. Configure Anchor for devnet
anchor build
solana config set --url devnet

# 2. Get devnet SOL
solana airdrop 2

# 3. Deploy programs
anchor deploy

# 4. Initialize programs
anchor run initialize
```

See [DEPLOYMENT.md](./DEPLOYMENT.md) for detailed instructions.

### Q22: What are the deployment costs?

**A:** Deployment costs on Solana:
- **Program deployment**: ~2-5 SOL per program (one-time, refundable)
- **Account rent**: ~0.001-0.01 SOL per account (rent-exempt)
- **Initialization**: ~0.01 SOL total

**Total for 4 programs**: ~10-20 SOL (~$100-200)

On Ethereum, deploying 4 contracts costs $5,000-20,000 in gas fees!

### Q23: How do I upgrade a deployed program?

**A:** Solana programs are upgradeable by default:

```bash
# Build new version
anchor build

# Upgrade program
anchor upgrade target/deploy/patent_nft.so --program-id <PROGRAM_ID>
```

**Important**: The program authority must sign the upgrade. Store the upgrade authority keypair securely!

### Q24: Can I make programs immutable?

**A:** Yes! Set the upgrade authority to `None`:

```bash
solana program set-upgrade-authority <PROGRAM_ID> --final
```

⚠️ **Warning**: This is **irreversible**. The program can never be upgraded again.

### Q25: How do I monitor program health?

**A:** Multiple approaches:
1. **Events**: Listen to program events (PatentMinted, NFTListed, etc.)
2. **Account watching**: Monitor state accounts for changes
3. **RPC calls**: Query program accounts periodically
4. **Solana Explorer**: View transactions and accounts
5. **Custom indexer**: Build off-chain database of program state

### Q26: What RPC providers should I use?

**A:** Options:
- **Devnet**: Public RPC (free, rate-limited)
- **Mainnet**:
  - QuickNode (paid, reliable)
  - Alchemy (paid, good free tier)
  - Helius (paid, optimized for Solana)
  - GenesysGo (paid, high performance)

**Never use public mainnet RPC for production** - it's rate-limited and unreliable.

### Q27: How do I back up program state?

**A:** Program state lives in accounts. Back up by:
1. **Exporting account data**: `solana account <ACCOUNT_ADDRESS> --output json`
2. **Indexing events**: Store all emitted events in a database
3. **Periodic snapshots**: Query and save all program accounts
4. **Transaction history**: Archive all transactions involving your programs

### Q28: What happens if I lose the authority keypair?

**A:** You lose control of the program:
- Can't pause/unpause
- Can't update fees
- Can't upgrade program
- Can't withdraw fees

**Prevention**:
- Use a **multi-sig wallet** (Squads Protocol)
- Store keypair in **hardware wallet** (Ledger)
- Keep **encrypted backups** in multiple locations
- Consider **DAO governance** for decentralization

---

## 🔄 Ethereum vs Solana

### Q29: What's the biggest difference in development?

**A:** **Account model**:
- **Ethereum**: Implicit state access (contract storage variables)
- **Solana**: Explicit account passing (must specify all accounts)

**Example**:
```solidity
// Ethereum - implicit state access
function mint() public {
    balances[msg.sender] += 1; // Accesses storage directly
}
```

```rust
// Solana - explicit accounts
pub fn mint(ctx: Context<Mint>) -> Result<()> {
    let state = &mut ctx.accounts.state; // Must pass account
    state.count += 1;
    Ok(())
}
```

### Q30: How do Solana transactions differ?

**A:** Key differences:

| Aspect | Ethereum | Solana |
|--------|----------|--------|
| **Gas** | Variable (auction) | Fixed (~0.0005 SOL) |
| **Speed** | ~15 seconds | ~400ms |
| **Finality** | Probabilistic | Deterministic |
| **Revert** | Entire transaction | Entire transaction |
| **Concurrency** | Sequential | Parallel |

### Q31: What replaces Solidity events?

**A:** Anchor's `emit!` macro:

```rust
#[event]
pub struct PatentMinted {
    pub patent_number: String,
    pub mint: Pubkey,
    pub owner: Pubkey,
}

// Emit event
emit!(PatentMinted {
    patent_number: patent_number.clone(),
    mint: ctx.accounts.mint.key(),
    owner: ctx.accounts.payer.key(),
});
```

Events are stored in transaction logs and can be indexed off-chain.

### Q32: How do I handle errors?

**A:** Anchor's error system:

```rust
#[error_code]
pub enum ErrorCode {
    #[msg("Patent already exists")]
    PatentAlreadyExists,
    #[msg("Insufficient payment")]
    InsufficientPayment,
}

// Use in code
require!(payment >= MIN_FEE, ErrorCode::InsufficientPayment);
```

Errors are returned as program errors with custom messages.

### Q33: What replaces OpenZeppelin?

**A:** Anchor provides similar functionality:
- **Access control**: `#[account(signer)]`, `has_one` constraints
- **Pausable**: Custom implementation (see PSP token program)
- **Ownable**: Authority field in state account
- **ReentrancyGuard**: Not needed (Solana prevents reentrancy)

### Q34: How do I test Solana programs?

**A:** Anchor's testing framework:

```typescript
import * as anchor from "@coral-xyz/anchor";

describe("patent-nft", () => {
  it("Mints a patent NFT", async () => {
    const tx = await program.methods
      .mintPatentNft("US1234567A", "Patent NFT", "PAT", "ipfs://...")
      .accounts({ /* accounts */ })
      .rpc();

    // Verify state
    const state = await program.account.state.fetch(statePda);
    assert.equal(state.totalMinted, 1);
  });
});
```

Run with: `anchor test`

---

## 💻 Development & Integration

### Q35: How do I integrate with a React frontend?

**A:** Use the TypeScript SDK:

```typescript
import { PatentNFTSDK } from './sdk';
import { useWallet, useConnection } from '@solana/wallet-adapter-react';

function MintPage() {
  const wallet = useWallet();
  const { connection } = useConnection();
  const sdk = new PatentNFTSDK(connection, wallet);

  const handleMint = async () => {
    const tx = await sdk.mintPatentNFT(
      "US1234567A",
      "My Patent",
      "PAT",
      "ipfs://metadata"
    );
    console.log("Minted:", tx);
  };

  return <button onClick={handleMint}>Mint NFT</button>;
}
```

### Q36: What wallet adapters should I use?

**A:** Solana Wallet Adapter:

```bash
npm install @solana/wallet-adapter-react \
            @solana/wallet-adapter-react-ui \
            @solana/wallet-adapter-wallets
```

Supports: Phantom, Solflare, Backpack, Ledger, and more.

### Q37: How do I fetch NFT metadata?

**A:** Query Metaplex metadata account:

```typescript
import { Metaplex } from "@metaplex-foundation/js";

const metaplex = new Metaplex(connection);
const nft = await metaplex.nfts().findByMint({ mintAddress });

console.log(nft.name);        // "Patent US1234567A"
console.log(nft.uri);         // "ipfs://..."
console.log(nft.json.image);  // "ipfs://image.png"
```

### Q38: How do I listen for program events?

**A:** Subscribe to logs:

```typescript
const subscriptionId = connection.onLogs(
  programId,
  (logs) => {
    if (logs.logs.some(log => log.includes("PatentMinted"))) {
      console.log("New patent minted!");
      // Refresh UI
    }
  }
);

// Cleanup
connection.removeOnLogsListener(subscriptionId);
```

### Q39: How do I handle transaction errors?

**A:** Try-catch with detailed error parsing:

```typescript
try {
  const tx = await sdk.mintPatentNFT(...);
} catch (error) {
  if (error.message.includes("PatentAlreadyExists")) {
    alert("This patent has already been minted!");
  } else if (error.message.includes("InsufficientPayment")) {
    alert("Please send at least 0.05 SOL");
  } else {
    console.error("Transaction failed:", error);
  }
}
```

### Q40: How do I estimate transaction costs?

**A:** Get recent blockhash and fee:

```typescript
const { blockhash, lastValidBlockHeight } =
  await connection.getLatestBlockhash();

const message = transaction.compileMessage();
const fee = await connection.getFeeForMessage(message);

console.log(`Estimated fee: ${fee.value / 1e9} SOL`);
```

Typical fees: 0.0005 SOL (~$0.05)

### Q41: Can I batch multiple operations?

**A:** Yes! Solana supports transaction batching:

```typescript
const tx = new Transaction()
  .add(instruction1)
  .add(instruction2)
  .add(instruction3);

const signature = await wallet.sendTransaction(tx, connection);
```

**Limit**: ~1232 bytes per transaction (fits ~10-15 instructions)

### Q42: How do I handle rate limiting?

**A:** Implement retry logic:

```typescript
async function sendWithRetry(tx, maxRetries = 3) {
  for (let i = 0; i < maxRetries; i++) {
    try {
      return await connection.sendTransaction(tx);
    } catch (error) {
      if (i === maxRetries - 1) throw error;
      await sleep(1000 * (i + 1)); // Exponential backoff
    }
  }
}
```

### Q43: How do I verify transactions?

**A:** Confirm transaction status:

```typescript
const signature = await wallet.sendTransaction(tx, connection);

// Wait for confirmation
const confirmation = await connection.confirmTransaction({
  signature,
  blockhash,
  lastValidBlockHeight
});

if (confirmation.value.err) {
  throw new Error("Transaction failed");
}

console.log("Transaction confirmed!");
```

### Q44: How do I get user's NFTs?

**A:** Query token accounts:

```typescript
const tokenAccounts = await connection.getParsedTokenAccountsByOwner(
  userPublicKey,
  { programId: TOKEN_PROGRAM_ID }
);

const nfts = tokenAccounts.value.filter(
  account => account.account.data.parsed.info.tokenAmount.amount === "1"
);
```

### Q45: How do I display NFT images?

**A:** Fetch metadata from IPFS:

```typescript
const response = await fetch(nft.uri);
const metadata = await response.json();

return (
  <img
    src={metadata.image.replace("ipfs://", "https://ipfs.io/ipfs/")}
    alt={metadata.name}
  />
);
```

---

## 🎓 Advanced Topics

### Q46: What is rent-exempt balance?

**A:** Accounts must maintain minimum balance to avoid deletion:

```rust
// Calculate rent-exempt balance
let rent = Rent::get()?;
let space = 8 + 32 + 8 + 1; // discriminator + pubkey + u64 + bool
let lamports = rent.minimum_balance(space);
```

Typical: 0.001-0.01 SOL depending on account size.

### Q47: How do I optimize transaction size?

**A:** Techniques:
1. **Use PDAs** instead of passing addresses
2. **Compress data** (use smaller types)
3. **Split operations** across multiple transactions
4. **Use lookup tables** (address lookup tables)

### Q48: What are versioned transactions?

**A:** New transaction format supporting:
- **Address lookup tables** (reduce transaction size)
- **Multiple versions** (v0, legacy)
- **Better composability**

```typescript
const tx = new VersionedTransaction(message);
```

### Q49: How do I implement royalties?

**A:** Metaplex supports creator royalties:

```rust
let creators = vec![
    Creator {
        address: authority.key(),
        verified: true,
        share: 100, // 100% of royalties
    }
];

// Set in metadata
let seller_fee_basis_points = 500; // 5% royalty
```

Marketplaces that support Metaplex will automatically pay royalties.

### Q50: How do I build a custom indexer?

**A:** Listen to program events and store in database:

```typescript
// Subscribe to program logs
connection.onLogs(programId, async (logs) => {
  const events = parseEvents(logs);

  for (const event of events) {
    if (event.name === "PatentMinted") {
      await db.patents.insert({
        patentNumber: event.data.patentNumber,
        mint: event.data.mint,
        owner: event.data.owner,
        timestamp: Date.now()
      });
    }
  }
});
```

Use PostgreSQL, MongoDB, or Redis for storage.

---

## 🌟 Best Practices

### Q51: What are the top 5 Solana development tips?

**A:**
1. **Always derive PDAs correctly** - Wrong seeds = wrong accounts = failed transactions
2. **Check account ownership** - Verify accounts belong to expected programs
3. **Handle rent properly** - Ensure accounts are rent-exempt
4. **Test on devnet first** - Never deploy untested code to mainnet
5. **Use Anchor constraints** - Let the framework handle validation

### Q52: How do I debug failed transactions?

**A:** Steps:
1. **Check Solana Explorer** - View transaction logs
2. **Enable verbose logging** - `RUST_LOG=solana_runtime::system_instruction_processor=trace`
3. **Use anchor test** - Run tests locally
4. **Add custom logs** - `msg!("Debug: value = {}", value);`
5. **Verify account addresses** - Ensure PDAs match expected values

### Q53: What security audits should I do?

**A:** Before mainnet:
1. **Code review** - Multiple developers review code
2. **Automated testing** - 100% test coverage
3. **Fuzzing** - Test with random inputs
4. **Professional audit** - Hire security firm (OtterSec, Neodyme, etc.)
5. **Bug bounty** - Offer rewards for finding vulnerabilities

### Q54: How do I handle program upgrades?

**A:** Best practices:
1. **Version your state** - Add version field to accounts
2. **Migration instructions** - Create upgrade path for old accounts
3. **Gradual rollout** - Test on devnet, then mainnet
4. **Backup state** - Export all account data before upgrade
5. **Rollback plan** - Keep old program version ready

### Q55: What monitoring should I implement?

**A:** Monitor:
- **Transaction success rate** - Alert if < 95%
- **Account balances** - Ensure fee recipient has funds
- **Program errors** - Track error frequency
- **RPC health** - Monitor RPC provider uptime
- **User activity** - Track mints, listings, sales

---

## 📊 Performance & Optimization

### Q56: How many transactions per second can Solana handle?

**A:** Theoretical: **65,000 TPS**
Practical: **2,000-4,000 TPS** (current network)

For comparison:
- Ethereum: ~15 TPS
- Bitcoin: ~7 TPS
- Visa: ~24,000 TPS

### Q57: How do I optimize program size?

**A:** Techniques:
1. **Remove unused dependencies**
2. **Use `opt-level = "z"` in Cargo.toml**
3. **Avoid large string literals**
4. **Share common code** across programs
5. **Use `cargo-build-sbf --release`**

### Q58: What's the maximum account size?

**A:** **10 MB** per account

For larger data:
- **Split across multiple accounts**
- **Store off-chain** (IPFS, Arweave)
- **Use compression**

### Q59: How do I reduce compute units?

**A:** Optimize:
1. **Minimize CPIs** - Each CPI costs compute units
2. **Avoid loops** - Especially unbounded loops
3. **Use efficient data structures** - Vec vs HashMap
4. **Cache calculations** - Don't recompute values
5. **Request more compute** - `ComputeBudgetInstruction::set_compute_unit_limit()`

### Q60: What's the compute unit limit?

**A:**
- **Default**: 200,000 compute units per instruction
- **Maximum**: 1,400,000 compute units per transaction

Request more:
```typescript
const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
  units: 400000
});
transaction.add(modifyComputeUnits);
```

---

**🎉 You've completed the FAQ! For more details, see:**
- [TEACHME.md](./TEACHME.md) - Detailed architecture guide
- [DEPLOYMENT.md](./DEPLOYMENT.md) - Deployment instructions
- [MIGRATION_GUIDE.md](./MIGRATION_GUIDE.md) - Ethereum to Solana migration

**Built with ❤️ for the Solana community**

