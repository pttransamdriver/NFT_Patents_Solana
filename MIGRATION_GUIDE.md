# Migration Guide: Ethereum to Solana

This guide helps you understand the key differences between the Ethereum (Solidity) and Solana (Rust/Anchor) implementations of the NFT Patents project.

## Overview of Changes

| Aspect | Ethereum | Solana |
|--------|----------|--------|
| Language | Solidity | Rust (Anchor Framework) |
| NFT Standard | ERC-721 | Metaplex Token Metadata |
| Token Standard | ERC-20 | SPL Token |
| Storage Model | Contract Storage | Account-based |
| Address Generation | Deterministic (CREATE2) | PDAs (Program Derived Addresses) |
| Gas/Fees | Variable, high | Fixed, very low |
| Transaction Speed | ~15 seconds | ~400ms |

## Contract to Program Mapping

### 1. PatentNFT.sol → patent_nft (Rust)

**Ethereum (Solidity)**:
```solidity
contract PatentNFT is ERC721URIStorage, ERC721Enumerable {
    mapping(bytes32 => uint256) private _patentHashToTokenId;
    
    function mintPatentNFT(address to, string memory patentNumber) 
        external payable returns (uint256) {
        // Minting logic
    }
}
```

**Solana (Rust/Anchor)**:
```rust
#[program]
pub mod patent_nft {
    pub fn mint_patent_nft(
        ctx: Context<MintPatentNFT>,
        patent_number: String,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        // Minting logic using Metaplex
    }
}

#[account]
pub struct PatentRegistry {
    pub patent_hash: [u8; 32],
    pub token_id: u64,
    pub owner: Pubkey,
    pub patent_number: String,
}
```

**Key Differences**:
- Solana uses **Metaplex** for NFT standard instead of ERC-721
- Storage is in **separate accounts** (PatentRegistry) instead of contract storage
- **PDAs** replace mapping lookups
- Explicit **account passing** in Context structs

### 2. PSPToken.sol → psp_token (Rust)

**Ethereum (Solidity)**:
```solidity
contract PSPToken is ERC20, ERC20Burnable {
    function purchaseTokens() external payable {
        uint256 tokenAmount = (msg.value * 10**DECIMALS) / tokenPriceInWei;
        _mint(msg.sender, tokenAmount);
    }
}
```

**Solana (Rust/Anchor)**:
```rust
pub fn purchase_tokens(ctx: Context<PurchaseTokens>, sol_amount: u64) -> Result<()> {
    // Transfer SOL
    let ix = system_instruction::transfer(...);
    invoke(&ix, ...)?;
    
    // Mint SPL tokens
    token::mint_to(cpi_ctx, token_amount)?;
    Ok(())
}
```

**Key Differences**:
- Uses **SPL Token** standard instead of ERC-20
- **CPI (Cross-Program Invocation)** for token operations
- Explicit **SOL transfer** using system program
- **Signer seeds** for PDA authority

### 3. NFTMarketplace.sol → nft_marketplace (Rust)

**Ethereum (Solidity)**:
```solidity
contract NFTMarketplace {
    mapping(uint256 => Listing) public listings;
    
    function buyNFT(uint256 listingId) external payable {
        // Transfer NFT and distribute funds
    }
}
```

**Solana (Rust/Anchor)**:
```rust
pub fn buy_nft(ctx: Context<BuyNFT>) -> Result<()> {
    // Transfer SOL
    **ctx.accounts.buyer.try_borrow_mut_lamports()? -= listing.price;
    **ctx.accounts.seller.try_borrow_mut_lamports()? += seller_amount;
    
    // Transfer NFT from escrow
    token::transfer(cpi_ctx, 1)?;
    Ok(())
}
```

**Key Differences**:
- **Escrow accounts** hold NFTs instead of contract custody
- **Direct lamport manipulation** for SOL transfers
- **PDA-based listings** instead of mapping
- Separate accounts for each listing

### 4. SearchPayment.sol → search_payment (Rust)

**Ethereum (Solidity)**:
```solidity
contract SearchPayment {
    enum PaymentToken { ETH, USDC, PSP }
    
    function payWithETH() external payable {
        // Process payment
    }
}
```

**Solana (Rust/Anchor)**:
```rust
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum PaymentToken {
    SOL,
    USDC,
    PSP,
}

pub fn pay_with_sol(ctx: Context<PayWithSOL>) -> Result<()> {
    // Transfer SOL and update stats
    Ok(())
}
```

**Key Differences**:
- **Separate functions** for each payment method
- **User stats** stored in separate PDA accounts
- **Token accounts** explicitly passed for SPL tokens

## Account Model Differences

### Ethereum Storage
```solidity
// All data stored in contract
mapping(address => uint256) public balances;
uint256 public totalSupply;
```

### Solana Accounts
```rust
// Data stored in separate accounts
#[account]
pub struct ProgramState {
    pub authority: Pubkey,
    pub total_supply: u64,
}

#[account]
pub struct UserAccount {
    pub owner: Pubkey,
    pub balance: u64,
}
```

## PDA (Program Derived Address) Patterns

PDAs replace Ethereum's deterministic address generation:

**Ethereum (CREATE2)**:
```solidity
address predicted = address(uint160(uint256(keccak256(abi.encodePacked(
    bytes1(0xff),
    address(this),
    salt,
    keccak256(bytecode)
)))));
```

**Solana (PDA)**:
```rust
let (pda, bump) = Pubkey::find_program_address(
    &[b"state"],
    program_id
);
```

## Event Handling

**Ethereum**:
```solidity
event PatentMinted(address indexed to, uint256 indexed tokenId, string patentNumber);
emit PatentMinted(to, tokenId, patentNumber);
```

**Solana**:
```rust
#[event]
pub struct PatentMinted {
    pub owner: Pubkey,
    pub token_id: u64,
    pub patent_number: String,
}

emit!(PatentMinted {
    owner: ctx.accounts.payer.key(),
    token_id,
    patent_number,
});
```

## Error Handling

**Ethereum**:
```solidity
require(msg.value >= mintingPrice, "Insufficient payment");
revert("Custom error message");
```

**Solana**:
```rust
#[error_code]
pub enum PatentNFTError {
    #[msg("Insufficient payment for minting")]
    InsufficientPayment,
}

require!(
    ctx.accounts.payer.lamports() >= state.minting_price,
    PatentNFTError::InsufficientPayment
);
```

## Access Control

**Ethereum**:
```solidity
modifier onlyOwner() {
    require(msg.sender == owner, "Not owner");
    _;
}
```

**Solana**:
```rust
#[derive(Accounts)]
pub struct UpdateState<'info> {
    #[account(
        mut,
        has_one = authority  // Automatic check
    )]
    pub state: Account<'info, ProgramState>,
    
    pub authority: Signer<'info>,
}
```

## Frontend Integration Changes

### Web3 Provider

**Ethereum**:
```typescript
import { ethers } from 'ethers';
const provider = new ethers.providers.Web3Provider(window.ethereum);
```

**Solana**:
```typescript
import { Connection, PublicKey } from '@solana/web3.js';
import { useWallet } from '@solana/wallet-adapter-react';

const connection = new Connection('https://api.devnet.solana.com');
const { publicKey, signTransaction } = useWallet();
```

### Contract/Program Interaction

**Ethereum**:
```typescript
const contract = new ethers.Contract(address, abi, signer);
await contract.mintPatentNFT(to, patentNumber, { value: ethers.utils.parseEther("0.05") });
```

**Solana**:
```typescript
const program = new Program(idl, programId, provider);
await program.methods
  .mintPatentNft(patentNumber, name, symbol, uri)
  .accounts({ /* accounts */ })
  .rpc();
```

## Testing Differences

**Ethereum (Hardhat)**:
```javascript
const { expect } = require("chai");

describe("PatentNFT", function () {
  it("Should mint a patent NFT", async function () {
    const tx = await patentNFT.mintPatentNFT(addr1.address, "US1234567A");
    expect(await patentNFT.ownerOf(1)).to.equal(addr1.address);
  });
});
```

**Solana (Anchor)**:
```typescript
import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";

describe("patent-nft", () => {
  it("Mints a patent NFT", async () => {
    await program.methods
      .mintPatentNft("US1234567A", "Patent", "PAT", "uri")
      .accounts({ /* accounts */ })
      .rpc();
    
    const registry = await program.account.patentRegistry.fetch(registryPDA);
    assert.equal(registry.patentNumber, "US1234567A");
  });
});
```

## Cost Comparison

| Operation | Ethereum (Gas) | Solana (Lamports) |
|-----------|---------------|-------------------|
| Deploy Contract | ~2,000,000 gas (~$100-500) | ~5 SOL (~$100-500) |
| Mint NFT | ~200,000 gas (~$10-50) | ~5,000 lamports (~$0.001) |
| Transfer NFT | ~50,000 gas (~$2-10) | ~5,000 lamports (~$0.001) |
| List on Marketplace | ~100,000 gas (~$5-25) | ~5,000 lamports (~$0.001) |

## Migration Checklist

- [ ] Understand Solana account model
- [ ] Learn Rust basics and Anchor framework
- [ ] Set up Solana development environment
- [ ] Deploy programs to devnet
- [ ] Test all functionality
- [ ] Update frontend to use Solana wallet adapters
- [ ] Migrate backend API endpoints
- [ ] Update documentation
- [ ] Perform security audit
- [ ] Deploy to mainnet

## Resources

- [Solana Cookbook](https://solanacookbook.com/)
- [Anchor Book](https://book.anchor-lang.com/)
- [Metaplex Docs](https://docs.metaplex.com/)
- [SPL Token Docs](https://spl.solana.com/token)

---

For questions, refer to the main README.md or Solana documentation.

