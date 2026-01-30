# Setup and Testing Guide

This guide will help you set up the Solana development environment and test the security fixes.

---

## 🔧 Installation (One-Time Setup)

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

### 3. Install Anchor
```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

### 4. Verify Installation
```bash
rustc --version
solana --version
anchor --version
```

---

## 🏗️ Build the Project

### Build All Programs
```bash
cd NFT_Patents_Solana
anchor build
```

This will compile all four programs:
- `patent-nft`
- `nft-marketplace`
- `psp-token`
- `search-payment`

### Expected Output
```
✅ Compiling patent-nft...
✅ Compiling nft-marketplace...
✅ Compiling psp-token...
✅ Compiling search-payment...
✅ Build successful!
```

---

## 🧪 Run Tests

### Run All Tests
```bash
anchor test
```

### Run Specific Program Tests
```bash
# Test patent-nft only
anchor test --skip-build -- --test patent-nft

# Test with verbose output
anchor test -- --nocapture
```

---

## ✅ Manual Verification (Without Building)

If you can't build right now, you can manually verify the security fixes:

### 1. Check Reentrancy Protection

**File:** `programs/nft-marketplace/src/lib.rs`  
**Line:** 90

Look for:
```rust
// CRITICAL FIX: Mark listing as inactive FIRST to prevent reentrancy
listing.active = false;
```

This should appear **BEFORE** any token transfers.

### 2. Check Overflow Protection

**File:** `programs/patent-nft/src/lib.rs`  
**Lines:** 67-71

Look for:
```rust
state.next_token_id = state
    .next_token_id
    .checked_add(1)
    .ok_or(PatentNFTError::TokenIdOverflow)?;
```

### 3. Check Balance Verification

**File:** `programs/nft-marketplace/src/lib.rs`  
**Lines:** 93-97

Look for:
```rust
let buyer_balance = ctx.accounts.buyer.to_account_info().lamports();
require!(
    buyer_balance >= listing.price,
    MarketplaceError::InsufficientFunds
);
```

### 4. Check Rent-Exempt Protection

**File:** `programs/patent-nft/src/lib.rs`  
**Lines:** 222-228

Look for:
```rust
let rent = Rent::get()?;
let min_balance = rent.minimum_balance(state_account.data_len());
require!(
    current_balance >= amount.checked_add(min_balance).ok_or(PatentNFTError::MathOverflow)?,
    PatentNFTError::InsufficientBalance
);
```

### 5. Check Token Account Validation

**File:** `programs/search-payment/src/lib.rs`  
**Lines:** 100-107

Look for:
```rust
require!(
    ctx.accounts.user_usdc_account.mint == state.usdc_token_mint,
    SearchPaymentError::InvalidTokenAccount
);
require!(
    ctx.accounts.user_usdc_account.owner == ctx.accounts.user.key(),
    SearchPaymentError::InvalidTokenAccount
);
```

---

## 📊 Code Review Checklist

Manually verify these patterns in the code:

### Patent NFT Program (`programs/patent-nft/src/lib.rs`)
- [ ] Line 67-71: `checked_add()` for token ID
- [ ] Line 30-51: Input validation (length checks)
- [ ] Line 222-228: Rent-exempt protection in withdraw
- [ ] Line 456-481: New error codes added

### NFT Marketplace (`programs/nft-marketplace/src/lib.rs`)
- [ ] Line 90: State change before transfers (reentrancy fix)
- [ ] Line 93-97: Balance verification
- [ ] Line 99-104: Token account validation
- [ ] Line 37-42: Overflow protection in list_nft
- [ ] Line 428-449: New error codes added

### PSP Token (`programs/psp-token/src/lib.rs`)
- [ ] Line 42-48: Atomic supply check with overflow protection
- [ ] Line 35-38: Balance verification in purchase
- [ ] Line 116-138: Comprehensive checks in redeem
- [ ] Line 265-286: Rent-exempt protection in withdraw
- [ ] Line 523-546: New error codes added

### Search Payment (`programs/search-payment/src/lib.rs`)
- [ ] Line 48-52: Balance check in pay_with_sol
- [ ] Line 100-123: Token validation in pay_with_usdc
- [ ] Line 164-187: Token validation in pay_with_psp
- [ ] Line 68-77: Overflow protection in stats
- [ ] Line 276-303: Rent-exempt protection in withdraw
- [ ] Line 581-602: New error codes added

---

## 🔍 Static Analysis (Alternative to Building)

### Check for Common Issues
```bash
# Check for unchecked arithmetic (should find none in fixed code)
grep -r "+=\|*=\|-=" programs/*/src/lib.rs | grep -v "checked_"

# Check for state changes after transfers (should find none)
grep -A5 "token::transfer\|invoke" programs/*/src/lib.rs | grep "active = false"

# Verify all error codes are defined
grep -r "ok_or(" programs/*/src/lib.rs
```

---

## 📝 Testing Strategy

### Unit Tests
The existing test file `tests/patent-nft.ts` covers:
- ✅ Program initialization
- ✅ NFT minting
- ✅ Duplicate prevention
- ✅ Price updates
- ✅ Fee withdrawal

### Integration Tests (Recommended to Add)
```typescript
// Test reentrancy protection
it("Prevents reentrancy in marketplace", async () => {
  // Attempt to buy same NFT twice simultaneously
  // Should fail on second attempt
});

// Test overflow protection
it("Prevents token ID overflow", async () => {
  // Set next_token_id to u64::MAX - 1
  // Attempt to mint - should succeed
  // Attempt to mint again - should fail with TokenIdOverflow
});

// Test balance verification
it("Prevents purchase with insufficient funds", async () => {
  // Attempt to buy with insufficient SOL
  // Should fail with InsufficientFunds
});
```

---

## 🚀 Deployment Testing

### 1. Deploy to Devnet
```bash
# Configure for devnet
solana config set --url devnet

# Get devnet SOL
solana airdrop 2

# Deploy
anchor deploy
```

### 2. Test on Devnet
```bash
# Run tests against devnet
anchor test --provider.cluster devnet
```

### 3. Monitor Transactions
```bash
# View recent transactions
solana transaction-history <PROGRAM_ID>
```

---

## 📚 Additional Resources

- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Solana Security Best Practices](https://docs.solana.com/developing/programming-model/security)

---

## ❓ Troubleshooting

### Build Errors
```bash
# Clean and rebuild
anchor clean
anchor build
```

### Test Failures
```bash
# Run with verbose logging
RUST_LOG=debug anchor test -- --nocapture
```

### Dependency Issues
```bash
# Update dependencies
cargo update
```

---

## ✅ Verification Complete

Once you've completed the setup and testing:

1. ✅ All programs compile without errors
2. ✅ All tests pass
3. ✅ Security patterns verified
4. ✅ Ready for deployment

**Your project is production-ready!** 🎉

