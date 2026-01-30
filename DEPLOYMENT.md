# Deployment Guide - NFT Patents Solana

This guide walks you through deploying the NFT Patents Solana programs to localnet, devnet, and mainnet.

## Prerequisites

Before deploying, ensure you have:

1. **Solana CLI installed** (v1.17+)
   ```bash
   solana --version
   ```

2. **Anchor CLI installed** (v0.29+)
   ```bash
   anchor --version
   ```

3. **Rust toolchain** (latest stable)
   ```bash
   rustc --version
   ```

4. **A funded wallet**
   - For localnet: No funding needed
   - For devnet: Use `solana airdrop`
   - For mainnet: Real SOL required

## Step 1: Build the Programs

```bash
# Clean previous builds
anchor clean

# Build all programs
anchor build

# Verify build succeeded
ls -la target/deploy/
```

You should see `.so` files for each program:
- `patent_nft.so`
- `psp_token.so`
- `nft_marketplace.so`
- `search_payment.so`

## Step 2: Generate Program Keypairs

```bash
# Generate keypairs for each program (if not already generated)
solana-keygen new -o target/deploy/patent_nft-keypair.json
solana-keygen new -o target/deploy/psp_token-keypair.json
solana-keygen new -o target/deploy/nft_marketplace-keypair.json
solana-keygen new -o target/deploy/search_payment-keypair.json
```

**Important**: Save these keypairs securely! They determine your program IDs.

## Step 3: Update Program IDs

Get the program IDs from the keypairs:

```bash
solana-keygen pubkey target/deploy/patent_nft-keypair.json
solana-keygen pubkey target/deploy/psp_token-keypair.json
solana-keygen pubkey target/deploy/nft_marketplace-keypair.json
solana-keygen pubkey target/deploy/search_payment-keypair.json
```

Update `Anchor.toml` with these program IDs:

```toml
[programs.localnet]
patent_nft = "YourPatentNFTProgramID"
psp_token = "YourPSPTokenProgramID"
nft_marketplace = "YourMarketplaceProgramID"
search_payment = "YourSearchPaymentProgramID"
```

Also update the `declare_id!()` macros in each program's `lib.rs`:

```rust
// programs/patent-nft/src/lib.rs
declare_id!("YourPatentNFTProgramID");

// programs/psp-token/src/lib.rs
declare_id!("YourPSPTokenProgramID");

// programs/nft-marketplace/src/lib.rs
declare_id!("YourMarketplaceProgramID");

// programs/search-payment/src/lib.rs
declare_id!("YourSearchPaymentProgramID");
```

Rebuild after updating:
```bash
anchor build
```

## Step 4: Deploy to Localnet

### Start Local Validator

```bash
# Terminal 1: Start validator
solana-test-validator
```

### Deploy Programs

```bash
# Terminal 2: Configure for localnet
solana config set --url localhost

# Check your wallet balance
solana balance

# Deploy all programs
anchor deploy --provider.cluster localnet

# Verify deployment
solana program show <PROGRAM_ID>
```

## Step 5: Deploy to Devnet

### Configure Devnet

```bash
# Set cluster to devnet
solana config set --url devnet

# Check your wallet
solana address

# Airdrop SOL (if needed)
solana airdrop 2

# Check balance
solana balance
```

### Deploy to Devnet

```bash
# Deploy all programs
anchor deploy --provider.cluster devnet

# Verify each program
solana program show <PATENT_NFT_PROGRAM_ID>
solana program show <PSP_TOKEN_PROGRAM_ID>
solana program show <NFT_MARKETPLACE_PROGRAM_ID>
solana program show <SEARCH_PAYMENT_PROGRAM_ID>
```

## Step 6: Initialize Programs

After deployment, you need to initialize each program:

### Initialize Patent NFT

```bash
anchor run initialize-patent-nft --provider.cluster devnet
```

Or using TypeScript:
```typescript
const mintingPrice = new BN(50_000_000); // 0.05 SOL
const platformFee = 250; // 2.5%

await patentNFTProgram.methods
  .initialize(mintingPrice, platformFee)
  .accounts({
    state: statePDA,
    authority: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Initialize PSP Token

```typescript
const tokenPrice = new BN(10_000); // Price in lamports per PSP

await pspTokenProgram.methods
  .initialize(tokenPrice)
  .accounts({
    state: statePDA,
    mint: pspMint,
    authority: wallet.publicKey,
    tokenProgram: TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
    rent: SYSVAR_RENT_PUBKEY,
  })
  .rpc();
```

### Initialize NFT Marketplace

```typescript
const platformFeePercent = 250; // 2.5%

await marketplaceProgram.methods
  .initialize(platformFeePercent)
  .accounts({
    state: statePDA,
    feeRecipient: feeRecipientPubkey,
    authority: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Initialize Search Payment

```typescript
const searchPriceSOL = new BN(5_000_000); // 0.005 SOL
const searchPriceUSDC = new BN(5_000_000); // 5 USDC (6 decimals)
const searchPricePSP = new BN(500_000_000_000_000_000); // 500 PSP (18 decimals)

await searchPaymentProgram.methods
  .initialize(searchPriceSOL, searchPriceUSDC, searchPricePSP)
  .accounts({
    state: statePDA,
    pspTokenMint: pspMintPubkey,
    usdcTokenMint: usdcMintPubkey,
    authority: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

## Step 7: Deploy to Mainnet

**⚠️ WARNING**: Mainnet deployment costs real SOL. Ensure you've thoroughly tested on devnet first!

### Prepare for Mainnet

1. **Audit your code**: Consider professional security audits
2. **Test extensively**: Run all tests on devnet
3. **Fund your wallet**: Ensure sufficient SOL for deployment (~5-10 SOL recommended)
4. **Backup keypairs**: Store program keypairs securely

### Deploy to Mainnet

```bash
# Configure for mainnet
solana config set --url mainnet-beta

# Verify wallet and balance
solana address
solana balance

# Deploy (this costs real SOL!)
anchor deploy --provider.cluster mainnet

# Verify deployment
solana program show <PROGRAM_ID>
```

## Cost Estimates

| Network | Per Program | Total (4 programs) |
|---------|-------------|-------------------|
| Localnet | Free | Free |
| Devnet | Free (airdrop) | Free |
| Mainnet | ~1-2 SOL | ~4-8 SOL |

## Troubleshooting

### "Insufficient funds"
```bash
# Check balance
solana balance

# For devnet, airdrop more
solana airdrop 2
```

### "Program already deployed"
```bash
# Upgrade existing program
anchor upgrade <PROGRAM_PATH> --program-id <PROGRAM_ID>
```

### "Build failed"
```bash
# Clean and rebuild
anchor clean
cargo clean
anchor build
```

## Post-Deployment Checklist

- [ ] All programs deployed successfully
- [ ] Program IDs updated in `Anchor.toml`
- [ ] Program IDs updated in `lib.rs` files
- [ ] All programs initialized
- [ ] Test basic functionality (mint, list, buy)
- [ ] Update frontend with new program IDs
- [ ] Document program addresses
- [ ] Set up monitoring/alerts

## Security Recommendations

1. **Use a dedicated deployment wallet**
2. **Enable program upgrade authority carefully**
3. **Consider making programs immutable** after thorough testing
4. **Monitor program accounts** for unusual activity
5. **Set up alerts** for large transactions

## Next Steps

After deployment:
1. Update frontend configuration with program IDs
2. Test all functionality end-to-end
3. Set up monitoring and logging
4. Prepare user documentation
5. Plan for upgrades and maintenance

---

For questions or issues, refer to the main README.md or open an issue on GitHub.

