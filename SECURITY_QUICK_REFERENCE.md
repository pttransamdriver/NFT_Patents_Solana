# Security Quick Reference Card

Quick reference for security patterns implemented in this project.

---

## 🔴 Critical Patterns

### 1. Reentrancy Protection
```rust
// ✅ CORRECT: State change FIRST
listing.active = false;
token::transfer(cpi_ctx, 1)?;

// ❌ WRONG: Transfer first
token::transfer(cpi_ctx, 1)?;
listing.active = false;  // VULNERABLE!
```

### 2. Overflow Protection
```rust
// ✅ CORRECT: Use checked_add
state.count = state.count
    .checked_add(1)
    .ok_or(ErrorCode::MathOverflow)?;

// ❌ WRONG: Direct addition
state.count += 1;  // Can overflow!
```

### 3. Balance Verification
```rust
// ✅ CORRECT: Check before transfer
let balance = account.lamports();
require!(balance >= amount, ErrorCode::InsufficientFunds);
**account.try_borrow_mut_lamports()? -= amount;

// ❌ WRONG: Transfer without check
**account.try_borrow_mut_lamports()? -= amount;  // Can underflow!
```

---

## 🟠 High Priority Patterns

### 4. Rent-Exempt Protection
```rust
// ✅ CORRECT: Preserve rent-exempt minimum
let rent = Rent::get()?;
let min_balance = rent.minimum_balance(account.data_len());
require!(
    current_balance >= amount.checked_add(min_balance)?,
    ErrorCode::InsufficientBalance
);
```

### 5. Token Account Validation
```rust
// ✅ CORRECT: Validate mint and owner
require!(
    token_account.mint == expected_mint,
    ErrorCode::InvalidTokenAccount
);
require!(
    token_account.owner == expected_owner,
    ErrorCode::InvalidTokenAccount
);
```

### 6. Input Validation
```rust
// ✅ CORRECT: Validate length
require!(
    name.len() > 0 && name.len() <= 32,
    ErrorCode::InvalidName
);
require!(amount > 0, ErrorCode::InvalidAmount);
```

---

## 🟡 Medium Priority Patterns

### 7. Fail-Fast Pattern
```rust
// ✅ CORRECT: Validate early, execute late
pub fn process(ctx: Context<Process>, amount: u64) -> Result<()> {
    // 1. Validate inputs
    require!(amount > 0, ErrorCode::InvalidAmount);
    require!(!state.paused, ErrorCode::Paused);
    
    // 2. Verify balances
    require!(balance >= amount, ErrorCode::InsufficientFunds);
    
    // 3. Update state
    state.total += amount;
    
    // 4. Execute transfers (expensive, done last)
    token::transfer(cpi_ctx, amount)?;
    
    Ok(())
}
```

### 8. Access Control
```rust
// ✅ CORRECT: Use has_one constraint
#[account(
    mut,
    seeds = [b"state"],
    bump = state.bump,
    has_one = authority  // Verifies state.authority == authority.key()
)]
pub state: Account<'info, ProgramState>,

pub authority: Signer<'info>,  // Must be signer
```

---

## 📋 Security Checklist

Before deploying any function:

- [ ] State changes before external calls?
- [ ] All arithmetic uses `checked_*()` methods?
- [ ] Balance verified before transfers?
- [ ] Rent-exempt minimum preserved?
- [ ] Token accounts validated (mint + owner)?
- [ ] Input lengths validated?
- [ ] Access control on privileged operations?
- [ ] Proper error codes defined?
- [ ] Fail-fast pattern implemented?
- [ ] No hardcoded addresses?

---

## 🚨 Common Vulnerabilities to Avoid

### ❌ DON'T: Check-Then-Use Pattern
```rust
// VULNERABLE to race conditions
if balance >= amount {
    transfer(amount);  // Balance might change between check and use!
}
```

### ❌ DON'T: Unchecked Arithmetic
```rust
// Can overflow/underflow
count += 1;
balance -= amount;
total = price * quantity;
```

### ❌ DON'T: Skip Validation
```rust
// No validation = potential exploit
pub fn transfer(amount: u64) {
    // Missing: amount > 0 check
    // Missing: balance check
    // Missing: overflow check
}
```

### ❌ DON'T: State After Calls
```rust
// Vulnerable to reentrancy
token::transfer(cpi_ctx, amount)?;
state.completed = true;  // Too late!
```

---

## ✅ Security Patterns Summary

| Pattern | Purpose | Priority |
|---------|---------|----------|
| Reentrancy Protection | Prevent double-execution | 🔴 Critical |
| Overflow Protection | Prevent arithmetic errors | 🔴 Critical |
| Balance Verification | Prevent underflow | 🔴 Critical |
| Rent-Exempt Protection | Prevent account deletion | 🟠 High |
| Token Validation | Prevent wrong tokens | 🟠 High |
| Input Validation | Prevent invalid data | 🟠 High |
| Fail-Fast Pattern | Save compute units | 🟡 Medium |
| Access Control | Prevent unauthorized access | 🟠 High |

---

## 📚 Error Code Template

```rust
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds for transaction")]
    InsufficientFunds,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("Cannot withdraw below rent-exempt minimum")]
    InsufficientBalance,
    #[msg("Invalid amount (must be > 0)")]
    InvalidAmount,
}
```

---

## 🎯 Quick Tips

1. **Always** use `checked_*()` for arithmetic
2. **Always** update state before external calls
3. **Always** verify balances before transfers
4. **Always** validate token account mint and owner
5. **Always** preserve rent-exempt minimum
6. **Always** validate input lengths
7. **Always** use descriptive error messages
8. **Never** trust user input without validation
9. **Never** skip balance checks
10. **Never** use unchecked arithmetic

---

## 📞 Need Help?

- See `SECURITY_BEST_PRACTICES.md` for detailed examples
- See `SECURITY_AUDIT_REPORT.md` for vulnerability details
- See `SECURITY_FIXES_SUMMARY.md` for specific fixes

---

**Remember:** Security is not optional. Every line of code is a potential vulnerability if not properly secured.

