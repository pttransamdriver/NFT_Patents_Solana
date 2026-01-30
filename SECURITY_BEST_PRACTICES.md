# Solana Security Best Practices

This document outlines the security best practices implemented in this project and serves as a reference for maintaining secure Solana programs.

---

## 1. Reentrancy Protection

### The Problem
Reentrancy attacks occur when external calls are made before state changes, allowing malicious contracts to re-enter and exploit inconsistent state.

### The Solution
**Always update state BEFORE making external calls (Checks-Effects-Interactions pattern)**

```rust
// ❌ VULNERABLE
pub fn buy_nft(ctx: Context<BuyNFT>) -> Result<()> {
    // Transfer happens first
    token::transfer(cpi_ctx, 1)?;
    
    // State change happens after (VULNERABLE!)
    listing.active = false;
    Ok(())
}

// ✅ SECURE
pub fn buy_nft(ctx: Context<BuyNFT>) -> Result<()> {
    // State change FIRST
    listing.active = false;
    
    // Then transfer
    token::transfer(cpi_ctx, 1)?;
    Ok(())
}
```

---

## 2. Integer Overflow Protection

### The Problem
Arithmetic operations can overflow, causing unexpected behavior or security vulnerabilities.

### The Solution
**Always use checked arithmetic operations**

```rust
// ❌ VULNERABLE
state.next_token_id += 1;
user_stats.total_paid += amount;

// ✅ SECURE
state.next_token_id = state
    .next_token_id
    .checked_add(1)
    .ok_or(ErrorCode::MathOverflow)?;

user_stats.total_paid = user_stats
    .total_paid
    .checked_add(amount)
    .ok_or(ErrorCode::MathOverflow)?;
```

---

## 3. Balance Verification

### The Problem
Attempting to transfer more than available balance causes runtime errors or exploits.

### The Solution
**Verify balances BEFORE attempting transfers**

```rust
// ❌ VULNERABLE
**buyer.try_borrow_mut_lamports()? -= price;

// ✅ SECURE
let buyer_balance = buyer.to_account_info().lamports();
require!(
    buyer_balance >= price,
    ErrorCode::InsufficientFunds
);
**buyer.try_borrow_mut_lamports()? -= price;
```

---

## 4. Rent-Exempt Protection

### The Problem
Withdrawing funds below rent-exempt minimum causes account to be deleted.

### The Solution
**Calculate and preserve rent-exempt minimum**

```rust
// ✅ SECURE
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let state_account = ctx.accounts.state.to_account_info();
    let rent = Rent::get()?;
    let min_balance = rent.minimum_balance(state_account.data_len());
    
    let current_balance = state_account.lamports();
    require!(
        current_balance >= amount.checked_add(min_balance)
            .ok_or(ErrorCode::MathOverflow)?,
        ErrorCode::InsufficientBalance
    );
    
    **state_account.try_borrow_mut_lamports()? -= amount;
    Ok(())
}
```

---

## 5. Input Validation

### The Problem
Unvalidated inputs can cause excessive compute usage, storage issues, or exploits.

### The Solution
**Validate all user inputs**

```rust
// ✅ SECURE
pub fn mint_nft(
    ctx: Context<MintNFT>,
    patent_number: String,
    name: String,
    uri: String,
) -> Result<()> {
    require!(
        patent_number.len() > 0 && patent_number.len() <= 50,
        ErrorCode::InvalidPatentNumber
    );
    require!(
        name.len() > 0 && name.len() <= 32,
        ErrorCode::InvalidName
    );
    require!(
        uri.len() > 0 && uri.len() <= 200,
        ErrorCode::InvalidUri
    );
    // ... rest of function
}
```

---

## 6. Token Account Validation

### The Problem
Malicious users can pass incorrect token accounts to steal funds or tokens.

### The Solution
**Verify token account mint and ownership**

```rust
// ✅ SECURE
pub fn transfer_tokens(ctx: Context<TransferTokens>) -> Result<()> {
    // Verify mint matches expected
    require!(
        ctx.accounts.user_token_account.mint == ctx.accounts.expected_mint.key(),
        ErrorCode::InvalidTokenAccount
    );
    
    // Verify ownership
    require!(
        ctx.accounts.user_token_account.owner == ctx.accounts.user.key(),
        ErrorCode::InvalidTokenAccount
    );
    
    // Now safe to transfer
    token::transfer(cpi_ctx, amount)?;
    Ok(())
}
```

---

## 7. Access Control

### The Problem
Unauthorized users executing privileged operations.

### The Solution
**Use Anchor's `has_one` constraint and verify signers**

```rust
// ✅ SECURE
#[derive(Accounts)]
pub struct AdminOperation<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump,
        has_one = authority  // Verifies state.authority == authority.key()
    )]
    pub state: Account<'info, ProgramState>,
    
    pub authority: Signer<'info>,  // Must be signer
}
```

---

## 8. Fail-Fast Pattern

### The Problem
Performing expensive operations before validation wastes compute units.

### The Solution
**Validate early, execute late**

```rust
// ✅ SECURE
pub fn purchase(ctx: Context<Purchase>, amount: u64) -> Result<()> {
    // 1. Validate inputs
    require!(amount > 0, ErrorCode::InvalidAmount);
    require!(!state.paused, ErrorCode::ContractPaused);
    
    // 2. Verify balances
    require!(
        buyer_balance >= price,
        ErrorCode::InsufficientFunds
    );
    
    // 3. Update state
    state.total_sales += amount;
    
    // 4. Execute transfers (most expensive, done last)
    token::transfer(cpi_ctx, amount)?;
    
    Ok(())
}
```

---

## 9. Race Condition Prevention

### The Problem
Multiple transactions executing simultaneously can cause inconsistent state.

### The Solution
**Use atomic operations and proper account constraints**

```rust
// ✅ SECURE - Use init constraint to prevent double-initialization
#[account(
    init,
    payer = user,
    space = 8 + Registry::INIT_SPACE,
    seeds = [b"registry", patent_hash.as_ref()],
    bump
)]
pub registry: Account<'info, Registry>,

// State changes are atomic within a transaction
listing.active = false;  // This prevents concurrent purchases
```

---

## 10. Error Handling

### The Problem
Poor error messages make debugging difficult and hide security issues.

### The Solution
**Use descriptive error codes**

```rust
// ✅ SECURE
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds for transaction")]
    InsufficientFunds,
    #[msg("Token account mint does not match expected mint")]
    InvalidTokenAccount,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("Cannot withdraw below rent-exempt minimum")]
    InsufficientBalance,
}
```

---

## Security Checklist

Before deploying to production, verify:

- [ ] All arithmetic uses `checked_*()` methods
- [ ] State changes happen before external calls
- [ ] All user inputs are validated
- [ ] Balance checks before all transfers
- [ ] Rent-exempt minimums are preserved
- [ ] Token accounts are validated (mint + ownership)
- [ ] Access control on all privileged operations
- [ ] Proper error handling with descriptive messages
- [ ] No hardcoded addresses (use PDAs)
- [ ] Comprehensive test coverage

---

## Additional Resources

- [Solana Security Best Practices](https://docs.solana.com/developing/programming-model/security)
- [Anchor Security](https://www.anchor-lang.com/docs/security)
- [Neodyme Security Guide](https://github.com/neodyme-labs/solana-security-txt)
- [Sealevel Attacks](https://github.com/coral-xyz/sealevel-attacks)

---

**Remember:** Security is not a feature, it's a requirement. Always prioritize security over convenience.

