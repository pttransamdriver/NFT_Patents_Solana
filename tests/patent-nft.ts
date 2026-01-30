import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { assert } from "chai";

describe("patent-nft", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PatentNft as Program;
  
  let statePDA: PublicKey;
  let stateBump: number;
  
  const mintingPrice = new BN(50_000_000); // 0.05 SOL
  const platformFee = 250; // 2.5%

  before(async () => {
    // Derive state PDA
    [statePDA, stateBump] = await PublicKey.findProgramAddress(
      [Buffer.from("state")],
      program.programId
    );
  });

  it("Initializes the patent NFT program", async () => {
    const tx = await program.methods
      .initialize(mintingPrice, platformFee)
      .accounts({
        state: statePDA,
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Initialize transaction signature:", tx);

    // Fetch and verify state
    const state = await program.account.programState.fetch(statePDA);
    assert.equal(state.authority.toString(), provider.wallet.publicKey.toString());
    assert.equal(state.mintingPrice.toNumber(), mintingPrice.toNumber());
    assert.equal(state.platformFeePercentage, platformFee);
    assert.equal(state.nextTokenId.toNumber(), 1);
  });

  it("Mints a patent NFT", async () => {
    const patentNumber = "US1234567A";
    const name = "Test Patent";
    const symbol = "PAT";
    const uri = "https://api.example.com/metadata/US1234567A";

    // Normalize patent number to get hash
    const normalized = patentNumber.replace(/[\s-]/g, '').toUpperCase();
    const crypto = require('crypto');
    const patentHash = crypto.createHash('sha256').update(normalized).digest();

    // Derive patent registry PDA
    const [patentRegistryPDA] = await PublicKey.findProgramAddress(
      [Buffer.from("patent"), patentHash],
      program.programId
    );

    // Create mint keypair
    const mintKeypair = Keypair.generate();

    // Derive Metaplex PDAs
    const METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    
    const [metadataPDA] = await PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
      ],
      METADATA_PROGRAM_ID
    );

    const [masterEditionPDA] = await PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
        Buffer.from("edition"),
      ],
      METADATA_PROGRAM_ID
    );

    // Mint the NFT
    const tx = await program.methods
      .mintPatentNft(patentNumber, name, symbol, uri)
      .accounts({
        state: statePDA,
        patentRegistry: patentRegistryPDA,
        payer: provider.wallet.publicKey,
        authority: statePDA,
        mint: mintKeypair.publicKey,
        metadata: metadataPDA,
        masterEdition: masterEditionPDA,
        tokenMetadataProgram: METADATA_PROGRAM_ID,
        sysvarInstructions: new PublicKey("Sysvar1nstructions1111111111111111111111111"),
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([mintKeypair])
      .rpc();

    console.log("Mint transaction signature:", tx);

    // Verify patent registry
    const registry = await program.account.patentRegistry.fetch(patentRegistryPDA);
    assert.equal(registry.patentNumber, patentNumber);
    assert.equal(registry.tokenId.toNumber(), 1);
    assert.equal(registry.owner.toString(), provider.wallet.publicKey.toString());
  });

  it("Prevents duplicate patent minting", async () => {
    const patentNumber = "US1234567A"; // Same as previous test
    const name = "Duplicate Patent";
    const symbol = "PAT";
    const uri = "https://api.example.com/metadata/US1234567A";

    const mintKeypair = Keypair.generate();

    try {
      await program.methods
        .mintPatentNft(patentNumber, name, symbol, uri)
        .accounts({
          // ... same accounts as before
        })
        .signers([mintKeypair])
        .rpc();
      
      assert.fail("Should have thrown error for duplicate patent");
    } catch (error) {
      assert.include(error.toString(), "Patent already minted");
    }
  });

  it("Updates minting price", async () => {
    const newPrice = new BN(100_000_000); // 0.1 SOL

    const tx = await program.methods
      .updateMintingPrice(newPrice)
      .accounts({
        state: statePDA,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log("Update price transaction signature:", tx);

    // Verify updated price
    const state = await program.account.programState.fetch(statePDA);
    assert.equal(state.mintingPrice.toNumber(), newPrice.toNumber());
  });

  it("Withdraws accumulated fees", async () => {
    const balanceBefore = await provider.connection.getBalance(provider.wallet.publicKey);
    
    const amount = new BN(10_000_000); // 0.01 SOL

    const tx = await program.methods
      .withdraw(amount)
      .accounts({
        state: statePDA,
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Withdraw transaction signature:", tx);

    const balanceAfter = await provider.connection.getBalance(provider.wallet.publicKey);
    assert.isTrue(balanceAfter > balanceBefore);
  });
});

