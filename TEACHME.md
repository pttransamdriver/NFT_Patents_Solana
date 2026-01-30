# 📚 TEACH ME: Patent NFT Marketplace - Solana Edition

This document explains how the Solana Patent NFT Marketplace is structured, why it's built this way, and **exactly which files and accounts are involved in each user interaction**. Perfect for becoming a Solana development expert.

---

## 📖 How to Use This Guide

**For Learning Solana Development:**
1. Start with "Project Goals & Vision" to understand WHY
2. Study "Solana Architecture Fundamentals" to understand WHAT
3. **Focus on "Complete Program Execution Flows"** to understand HOW (most important)
4. Review "Account Management Patterns" for Solana-specific concepts
5. Check "Quiz Preparation" section at the end

**For Development:**
- Reference the "Project Structure" section to find files
- Use "Program Execution Flows" to trace bugs and understand data flow
- Check "Deployment" section for environment setup
- Review "Common Patterns" for reusable code examples

---

## 🎯 Project Goals & Vision

**Main Goal**: Create a high-performance, low-cost decentralized marketplace where real patents can be converted into NFTs and traded on Solana.

**Key Requirements**:
- Convert real patents from patent offices into tradeable NFTs
- **Metaplex Standard**: Use Metaplex Token Metadata for NFT compatibility
- Ensure each patent can only be minted once (global uniqueness via PDAs)
- Collect fees (0.05 SOL minting + 2.5% marketplace)
- Support multiple payment methods (SOL, USDC, PSP tokens)
- **Ultra-low costs**: ~$0.001 per transaction vs $50-200 on Ethereum
- **Fast finality**: ~400ms vs 15 seconds on Ethereum

---

## 🏗️ Solana Architecture Fundamentals

### Key Differences from Ethereum

| Concept | Ethereum | Solana |
|---------|----------|--------|
| **State Storage** | Contract storage variables | Separate account data |
| **Address Generation** | CREATE2 | Program Derived Addresses (PDAs) |
| **Function Calls** | Implicit state access | Explicit account passing |
| **Concurrency** | Sequential execution | Parallel transaction processing |
| **Fees** | Variable gas (auction) | Fixed, predictable fees |
| **Speed** | ~15 seconds | ~400ms |

### Program Derived Addresses (PDAs)

PDAs are **deterministic addresses** derived from seeds. They enable:
1. **Predictable account locations** without private keys
2. **Cross-program invocation** (programs can sign for PDAs)
3. **Efficient lookups** (no need to store addresses)

**Example PDA Derivation:**
```rust
let (state_pda, bump) = Pubkey::find_program_address(
    &[b"state"],
    program_id
);
```

### Account Model

Every piece of data in Solana lives in an **account**:
- **Program Accounts**: Store executable code (immutable)
- **Data Accounts**: Store program state (mutable)
- **Token Accounts**: Store SPL token balances
- **System Accounts**: Native SOL balances

**Rent**: Accounts must maintain minimum balance to stay alive (rent-exempt).

---

## 📂 Project Structure

```
NFT_Patents_Solana/
├── programs/                    # Rust programs (on-chain)
│   ├── patent-nft/             # Patent NFT minting (415 lines)
│   │   ├── src/lib.rs          # Program logic
│   │   ├── Cargo.toml          # Dependencies
│   │   └── Xargo.toml          # Build config
│   ├── psp-token/              # PSP SPL Token (483 lines)
│   ├── nft-marketplace/        # NFT marketplace (394 lines)
│   └── search-payment/         # Multi-token payments (516 lines)
├── app/                        # TypeScript SDK (off-chain)
│   └── sdk.ts                  # Frontend integration (503 lines)
├── tests/                      # Anchor tests
│   └── patent-nft.ts           # Test suite (170 lines)
├── README.md                   # Main documentation
├── TEACHME.md                  # This file
├── DEPLOYMENT.md               # Deployment guide
├── MIGRATION_GUIDE.md          # Ethereum → Solana migration
├── QUICKSTART.md               # Quick start guide
├── Anchor.toml                 # Anchor configuration
├── Cargo.toml                  # Workspace config
└── package.json                # Node dependencies
```

---

## 🔄 COMPLETE PROGRAM EXECUTION FLOWS

This is the **most important section** for understanding how the Solana programs work. Each flow shows exactly which files and accounts are involved.

### 📖 Legend
- **📄** Frontend pages/components
- **⚙️** TypeScript SDK functions
- **🦀** Rust program instructions
- **💾** Account types (PDAs, token accounts, etc.)
- **🔌** External services (IPFS, Google Patents)

---

### 🔍 FLOW 1: Minting a Patent NFT

**User Action**: User searches for a patent and clicks "Mint NFT"

#### Step-by-Step Execution:

1. **📄 Frontend: MintNFTPage.tsx**
   - User enters patent number "US1234567A"
   - Clicks "Mint NFT" button
   - Calls SDK function

2. **⚙️ SDK: app/sdk.ts → PatentNFTSDK.mintPatentNFT()**
   ```typescript
   // Line 147-167
   async mintPatentNFT(patentNumber, name, symbol, uri)
   ```
   - Normalizes patent number (removes spaces, uppercase)
   - Creates SHA-256 hash of normalized patent number
   - Derives PDAs:
     - `state_pda` = PDA(["state"])
     - `patent_registry_pda` = PDA(["patent", patent_hash])
   - Creates new mint keypair for the NFT
   - Derives Metaplex PDAs:
     - `metadata_pda` = Metaplex PDA for token metadata
     - `master_edition_pda` = Metaplex PDA for master edition

3. **💾 Accounts Prepared**:
   ```
   ✓ state (PDA) - Program state account
   ✓ patent_registry (PDA) - Patent uniqueness tracker
   ✓ payer (Signer) - User's wallet
   ✓ mint (Keypair) - New NFT mint account
   ✓ metadata (PDA) - Metaplex metadata account
   ✓ master_edition (PDA) - Metaplex master edition
   ✓ token_metadata_program - Metaplex program
   ✓ token_program - SPL Token program
   ✓ system_program - Solana system program
   ```

4. **🦀 Program: programs/patent-nft/src/lib.rs → mint_patent_nft()**
   ```rust
   // Line 52-95
   pub fn mint_patent_nft(
       ctx: Context<MintPatentNFT>,
       patent_number: String,
       name: String,
       symbol: String,
       uri: String,
   ) -> Result<()>
   ```
   
   **Execution Steps**:
   - **Line 53-56**: Verify payment (0.05 SOL minimum)
   - **Line 58-62**: Check patent doesn't already exist
   - **Line 64-70**: Create patent registry account (PDA)
   - **Line 72-85**: Call Metaplex to create NFT metadata
   - **Line 87-90**: Increment token counter in state
   - **Line 92**: Emit `PatentMinted` event

5. **💾 State Changes**:
   ```
   ✓ Patent Registry created with patent_hash
   ✓ NFT minted with Metaplex metadata
   ✓ Token counter incremented
   ✓ 0.05 SOL transferred to program
   ```

6. **📄 Frontend: Success**
   - Transaction signature returned
   - UI shows "NFT Minted Successfully!"
   - NFT appears in user's wallet

**Total Files Involved**: 3 (MintNFTPage.tsx, sdk.ts, lib.rs)
**Total Accounts**: 10+ (state, registry, mint, metadata, etc.)
**Cost**: ~0.05 SOL + ~0.001 SOL gas = ~0.051 SOL

---

### 🏪 FLOW 2: Listing an NFT on the Marketplace

**User Action**: User clicks "List NFT" on their owned NFT

#### Step-by-Step Execution:

1. **📄 Frontend: MyNFTsModal.tsx**
   - User clicks "List for Sale" button
   - Opens ListNFTModal with NFT details
   - User enters price (e.g., 1 SOL)

2. **⚙️ SDK: app/sdk.ts → NFTMarketplaceSDK.listNFT()**
   ```typescript
   // Line 330-360
   async listNFT(nftMint: PublicKey, price: BN)
   ```
   - Derives PDAs:
     - `marketplace_state_pda` = PDA(["marketplace"])
     - `listing_pda` = PDA(["listing", nft_mint])
   - Gets Associated Token Accounts:
     - `seller_nft_account` = ATA(nft_mint, seller)
     - `escrow_nft_account` = ATA(nft_mint, listing_pda)

3. **💾 Accounts Prepared**:
   ```
   ✓ state (PDA) - Marketplace state
   ✓ listing (PDA) - Listing account for this NFT
   ✓ nft_mint - The NFT being listed
   ✓ seller (Signer) - User's wallet
   ✓ seller_nft_account (ATA) - Seller's NFT token account
   ✓ escrow_nft_account (ATA) - Escrow token account
   ✓ token_program - SPL Token program
   ✓ system_program - System program
   ```

4. **🦀 Program: programs/nft-marketplace/src/lib.rs → list_nft()**
   ```rust
   // Line 52-95
   pub fn list_nft(ctx: Context<ListNFT>, price: u64) -> Result<()>
   ```

   **Execution Steps**:
   - **Line 53-55**: Validate price > 0
   - **Line 57-65**: Create listing account (PDA)
   - **Line 67-75**: Transfer NFT from seller to escrow
   - **Line 77-82**: Store listing data (price, seller, active status)
   - **Line 84**: Emit `NFTListed` event

5. **💾 State Changes**:
   ```
   ✓ Listing account created with price and seller info
   ✓ NFT transferred to escrow (program-controlled account)
   ✓ Seller no longer has NFT in wallet (held in escrow)
   ```

6. **📄 Frontend: Success**
   - NFT appears in marketplace listings
   - Shows price and "Buy Now" button
   - Seller can cancel listing anytime

**Total Files Involved**: 3 (MyNFTsModal.tsx, sdk.ts, lib.rs)
**Total Accounts**: 8 (state, listing, mint, ATAs, etc.)
**Cost**: ~0.0005 SOL

---

### 💰 FLOW 3: Buying an NFT from the Marketplace

**User Action**: User clicks "Buy Now" on a listed NFT

#### Step-by-Step Execution:

1. **📄 Frontend: MarketplacePage.tsx**
   - User clicks "Buy Now" button
   - Confirms purchase in modal
   - Calls SDK function

2. **⚙️ SDK: app/sdk.ts → NFTMarketplaceSDK.buyNFT()**
   ```typescript
   // Line 362-395
   async buyNFT(nftMint: PublicKey, seller: PublicKey)
   ```
   - Derives PDAs and fetches state
   - Gets fee recipient from marketplace state
   - Prepares all accounts for transaction

3. **💾 Accounts Prepared**:
   ```
   ✓ state (PDA) - Marketplace state (for fee %)
   ✓ listing (PDA) - Listing being purchased
   ✓ buyer (Signer) - Buyer's wallet
   ✓ seller - Original seller's wallet
   ✓ fee_recipient - Platform fee recipient
   ✓ buyer_nft_account (ATA) - Buyer's NFT token account
   ✓ escrow_nft_account (ATA) - Escrow holding the NFT
   ✓ token_program - SPL Token program
   ✓ system_program - System program
   ```

4. **🦀 Program: programs/nft-marketplace/src/lib.rs → buy_nft()**
   ```rust
   // Line 97-145
   pub fn buy_nft(ctx: Context<BuyNFT>) -> Result<()>
   ```

   **Execution Steps**:
   - **Line 98-100**: Verify listing is active
   - **Line 102-105**: Calculate fees (2.5% platform fee)
   - **Line 107-115**: Transfer SOL from buyer to seller (97.5%)
   - **Line 117-120**: Transfer SOL from buyer to fee recipient (2.5%)
   - **Line 122-130**: Transfer NFT from escrow to buyer
   - **Line 132-135**: Mark listing as inactive
   - **Line 137**: Emit `NFTSold` event

5. **💾 State Changes**:
   ```
   ✓ Buyer pays listing price in SOL
   ✓ Seller receives 97.5% of price
   ✓ Platform receives 2.5% fee
   ✓ NFT transferred from escrow to buyer
   ✓ Listing marked inactive
   ```

6. **📄 Frontend: Success**
   - NFT appears in buyer's wallet
   - NFT removed from marketplace listings
   - Transaction history updated

**Total Files Involved**: 3 (MarketplacePage.tsx, sdk.ts, lib.rs)
**Total Accounts**: 9 (state, listing, buyer, seller, ATAs, etc.)
**Cost**: Listing price + ~0.0005 SOL gas

---

### 🪙 FLOW 4: Purchasing PSP Tokens

**User Action**: User wants to buy PSP tokens with SOL

#### Step-by-Step Execution:

1. **📄 Frontend: PSPTokenPage.tsx**
   - User enters SOL amount (e.g., 0.1 SOL)
   - Clicks "Purchase PSP" button
   - Calls SDK function

2. **⚙️ SDK: app/sdk.ts → PSPTokenSDK.purchaseTokens()**
   ```typescript
   // Line 230-255
   async purchaseTokens(solAmount: BN)
   ```
   - Derives state PDA
   - Gets or creates buyer's PSP token account
   - Calculates PSP amount based on exchange rate

3. **💾 Accounts Prepared**:
   ```
   ✓ state (PDA) - PSP program state
   ✓ mint (PDA) - PSP token mint
   ✓ buyer (Signer) - Buyer's wallet
   ✓ buyer_token_account (ATA) - Buyer's PSP token account
   ✓ token_program - SPL Token program
   ✓ system_program - System program
   ```

4. **🦀 Program: programs/psp-token/src/lib.rs → purchase_tokens()**
   ```rust
   // Line 95-135
   pub fn purchase_tokens(ctx: Context<PurchaseTokens>, sol_amount: u64) -> Result<()>
   ```

   **Execution Steps**:
   - **Line 96-98**: Verify contract not paused
   - **Line 100-105**: Calculate PSP amount from SOL
   - **Line 107-115**: Transfer SOL from buyer to program
   - **Line 117-125**: Mint PSP tokens to buyer
   - **Line 127-130**: Update total supply tracking
   - **Line 132**: Emit `TokensPurchased` event

5. **💾 State Changes**:
   ```
   ✓ SOL transferred from buyer to program
   ✓ PSP tokens minted to buyer's account
   ✓ Total supply increased
   ```

6. **📄 Frontend: Success**
   - PSP balance updated in UI
   - Can now use PSP for AI searches

**Total Files Involved**: 3 (PSPTokenPage.tsx, sdk.ts, lib.rs)
**Total Accounts**: 6 (state, mint, buyer, token account, etc.)
**Cost**: SOL amount + ~0.0005 SOL gas

---

### 🔍 FLOW 5: Paying for AI Search with Multiple Tokens

**User Action**: User wants to pay for AI patent search

#### Step-by-Step Execution:

1. **📄 Frontend: PatentSearchPage.tsx**
   - User selects payment method (SOL, USDC, or PSP)
   - Clicks "Pay for Search" button
   - Calls appropriate SDK function

2. **⚙️ SDK: app/sdk.ts → SearchPaymentSDK.payWithSOL()**
   ```typescript
   // Line 450-470
   async payWithSOL()
   ```
   - Derives PDAs for state and user stats
   - Prepares transaction

3. **💾 Accounts Prepared**:
   ```
   ✓ state (PDA) - Search payment program state
   ✓ user_stats (PDA) - User's payment statistics
   ✓ user (Signer) - User's wallet
   ✓ system_program - System program
   ```

4. **🦀 Program: programs/search-payment/src/lib.rs → pay_with_sol()**
   ```rust
   // Line 52-90
   pub fn pay_with_sol(ctx: Context<PayWithSOL>) -> Result<()>
   ```

   **Execution Steps**:
   - **Line 53-55**: Verify contract not paused
   - **Line 57-60**: Get search price from state
   - **Line 62-70**: Transfer SOL from user to program
   - **Line 72-80**: Update user stats (increment searches, track payment)
   - **Line 82-85**: Calculate search credits allocated
   - **Line 87**: Emit `PaymentReceived` event

5. **💾 State Changes**:
   ```
   ✓ SOL transferred to program
   ✓ User stats updated (searches purchased, SOL paid)
   ✓ Search credits allocated
   ```

6. **📄 Frontend: Success**
   - Search credits displayed
   - User can now perform AI searches
   - Payment history updated

**Total Files Involved**: 3 (PatentSearchPage.tsx, sdk.ts, lib.rs)
**Total Accounts**: 4 (state, user_stats, user, system)
**Cost**: Search price (~0.005 SOL) + ~0.0005 SOL gas

---

## 🔑 Account Management Patterns

### Pattern 1: PDA Derivation

**Purpose**: Create deterministic addresses without private keys

**Example**:
```rust
// Derive state PDA
let (state_pda, bump) = Pubkey::find_program_address(
    &[b"state"],
    program_id
);

// Derive user-specific PDA
let (user_stats_pda, bump) = Pubkey::find_program_address(
    &[b"user_stats", user.key().as_ref()],
    program_id
);
```

**When to Use**:
- Program state storage
- User-specific data
- Listing/registry accounts
- Any account the program needs to sign for

### Pattern 2: Associated Token Accounts (ATAs)

**Purpose**: Standard location for SPL token balances

**Example**:
```typescript
const userTokenAccount = await getAssociatedTokenAddress(
    mint,           // Token mint address
    owner,          // Owner's wallet
    allowOffCurve   // Allow PDA owners
);
```

**When to Use**:
- NFT ownership
- SPL token balances
- Escrow accounts

### Pattern 3: Cross-Program Invocation (CPI)

**Purpose**: Call other programs from your program

**Example**:
```rust
// Call SPL Token program to transfer tokens
let cpi_ctx = CpiContext::new(
    ctx.accounts.token_program.to_account_info(),
    Transfer {
        from: ctx.accounts.seller_account.to_account_info(),
        to: ctx.accounts.buyer_account.to_account_info(),
        authority: ctx.accounts.seller.to_account_info(),
    },
);
token::transfer(cpi_ctx, amount)?;
```

**When to Use**:
- Token transfers
- NFT minting (Metaplex)
- Calling other programs

---

## 📊 Performance Metrics

| Operation | Ethereum | Solana | Improvement |
|-----------|----------|--------|-------------|
| Mint NFT | $50-200 | $0.001 | **50,000x cheaper** |
| List NFT | $20-80 | $0.0005 | **40,000x cheaper** |
| Buy NFT | $30-100 | $0.0005 | **60,000x cheaper** |
| Token Transfer | $5-20 | $0.00025 | **20,000x cheaper** |
| Transaction Time | ~15 sec | ~400ms | **37x faster** |

---

## 🎓 Quiz Preparation

### Key Concepts to Master:

1. **PDAs**: How they're derived, why they're used, when to use them
2. **Account Model**: Difference between program accounts and data accounts
3. **Rent**: Why accounts need minimum balance, how to calculate rent-exempt amount
4. **CPIs**: How programs call other programs, signer seeds
5. **Anchor Framework**: How it simplifies Solana development
6. **Token Standards**: SPL Token vs Metaplex NFTs
7. **Transaction Flow**: How transactions are built, signed, and sent

### Practice Questions:

1. What PDA seeds are used for the patent registry account?
2. How does the marketplace ensure NFTs are held securely during listing?
3. What happens if you try to mint the same patent twice?
4. How are platform fees calculated and distributed in the marketplace?
5. What's the difference between `has_one` and manual validation in Anchor?

---

## 🚀 Next Steps

1. **Run the tests**: `anchor test`
2. **Deploy to devnet**: Follow DEPLOYMENT.md
3. **Explore the SDK**: Read app/sdk.ts
4. **Build a frontend**: Integrate the SDK with React
5. **Take the quiz**: Test your knowledge with the quiz program

---

## 📚 Additional Resources

- [Solana Cookbook](https://solanacookbook.com/)
- [Anchor Book](https://book.anchor-lang.com/)
- [Metaplex Docs](https://docs.metaplex.com/)
- [SPL Token Docs](https://spl.solana.com/token)
- [Solana Program Library](https://github.com/solana-labs/solana-program-library)

---

**Built with ❤️ for Solana developers**

