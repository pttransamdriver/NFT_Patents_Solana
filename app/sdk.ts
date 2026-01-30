/**
 * TypeScript SDK for NFT Patents Solana Programs
 * 
 * This SDK provides helper functions to interact with the four Solana programs:
 * 1. patent_nft - Patent NFT minting
 * 2. psp_token - PSP SPL Token
 * 3. nft_marketplace - NFT marketplace
 * 4. search_payment - Multi-token payment system
 */

import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider, BN } from "@coral-xyz/anchor";
import { 
  PublicKey, 
  SystemProgram, 
  SYSVAR_RENT_PUBKEY,
  Transaction,
  Connection,
  Keypair,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";

// Program IDs (update these after deployment)
export const PATENT_NFT_PROGRAM_ID = new PublicKey("PatNFT111111111111111111111111111111111111");
export const PSP_TOKEN_PROGRAM_ID = new PublicKey("PSPTok111111111111111111111111111111111111");
export const NFT_MARKETPLACE_PROGRAM_ID = new PublicKey("MktPla111111111111111111111111111111111111");
export const SEARCH_PAYMENT_PROGRAM_ID = new PublicKey("SrchPy111111111111111111111111111111111111");

/**
 * Helper function to derive PDAs
 */
export class PDAHelper {
  /**
   * Get Patent NFT program state PDA
   */
  static async getPatentNFTState(): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [Buffer.from("state")],
      PATENT_NFT_PROGRAM_ID
    );
  }

  /**
   * Get Patent Registry PDA for a specific patent number
   */
  static async getPatentRegistry(patentHash: Buffer): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [Buffer.from("patent"), patentHash],
      PATENT_NFT_PROGRAM_ID
    );
  }

  /**
   * Get PSP Token program state PDA
   */
  static async getPSPTokenState(): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [Buffer.from("state")],
      PSP_TOKEN_PROGRAM_ID
    );
  }

  /**
   * Get Spender authorization PDA
   */
  static async getSpenderState(spender: PublicKey): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [Buffer.from("spender"), spender.toBuffer()],
      PSP_TOKEN_PROGRAM_ID
    );
  }

  /**
   * Get Marketplace state PDA
   */
  static async getMarketplaceState(): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [Buffer.from("marketplace")],
      NFT_MARKETPLACE_PROGRAM_ID
    );
  }

  /**
   * Get Listing PDA for a specific NFT mint
   */
  static async getListing(nftMint: PublicKey): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [Buffer.from("listing"), nftMint.toBuffer()],
      NFT_MARKETPLACE_PROGRAM_ID
    );
  }

  /**
   * Get Search Payment program state PDA
   */
  static async getSearchPaymentState(): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [Buffer.from("state")],
      SEARCH_PAYMENT_PROGRAM_ID
    );
  }

  /**
   * Get User Stats PDA
   */
  static async getUserStats(user: PublicKey): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [Buffer.from("user_stats"), user.toBuffer()],
      SEARCH_PAYMENT_PROGRAM_ID
    );
  }
}

/**
 * Patent NFT SDK
 */
export class PatentNFTSDK {
  constructor(
    private program: Program,
    private provider: AnchorProvider
  ) {}

  /**
   * Initialize the Patent NFT program
   */
  async initialize(mintingPrice: BN, platformFeePercentage: number): Promise<string> {
    const [statePDA] = await PDAHelper.getPatentNFTState();

    const tx = await this.program.methods
      .initialize(mintingPrice, platformFeePercentage)
      .accounts({
        state: statePDA,
        authority: this.provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return tx;
  }

  /**
   * Mint a Patent NFT
   */
  async mintPatentNFT(
    patentNumber: string,
    name: string,
    symbol: string,
    uri: string
  ): Promise<string> {
    const [statePDA] = await PDAHelper.getPatentNFTState();
    
    // Normalize patent number and create hash
    const patentHash = this.normalizePatentId(patentNumber);
    const [patentRegistryPDA] = await PDAHelper.getPatentRegistry(patentHash);

    // Create new mint keypair
    const mintKeypair = Keypair.generate();

    // Derive metadata and master edition PDAs (Metaplex)
    const [metadataPDA] = await this.getMetadataPDA(mintKeypair.publicKey);
    const [masterEditionPDA] = await this.getMasterEditionPDA(mintKeypair.publicKey);

    const tx = await this.program.methods
      .mintPatentNft(patentNumber, name, symbol, uri)
      .accounts({
        state: statePDA,
        patentRegistry: patentRegistryPDA,
        payer: this.provider.wallet.publicKey,
        authority: statePDA, // Authority from state
        mint: mintKeypair.publicKey,
        metadata: metadataPDA,
        masterEdition: masterEditionPDA,
        tokenMetadataProgram: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
        sysvarInstructions: new PublicKey("Sysvar1nstructions1111111111111111111111111"),
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([mintKeypair])
      .rpc();

    return tx;
  }

  /**
   * Normalize patent ID (same logic as Rust program)
   */
  private normalizePatentId(patentNumber: string): Buffer {
    const normalized = patentNumber
      .replace(/[\s-]/g, '')
      .toUpperCase();

    const crypto = require('crypto');
    return crypto.createHash('sha256').update(normalized).digest();
  }

  /**
   * Get Metaplex metadata PDA
   */
  private async getMetadataPDA(mint: PublicKey): Promise<[PublicKey, number]> {
    const METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    return PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      METADATA_PROGRAM_ID
    );
  }

  /**
   * Get Metaplex master edition PDA
   */
  private async getMasterEditionPDA(mint: PublicKey): Promise<[PublicKey, number]> {
    const METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    return PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
        Buffer.from("edition"),
      ],
      METADATA_PROGRAM_ID
    );
  }
}

/**
 * PSP Token SDK
 */
export class PSPTokenSDK {
  constructor(
    private program: Program,
    private provider: AnchorProvider
  ) {}

  /**
   * Purchase PSP tokens with SOL
   */
  async purchaseTokens(solAmount: BN): Promise<string> {
    const [statePDA] = await PDAHelper.getPSPTokenState();

    // Get or create buyer's token account
    const buyerTokenAccount = await getAssociatedTokenAddress(
      statePDA, // PSP mint (stored in state)
      this.provider.wallet.publicKey
    );

    const tx = await this.program.methods
      .purchaseTokens(solAmount)
      .accounts({
        state: statePDA,
        mint: statePDA, // Assuming mint is derived from state
        buyer: this.provider.wallet.publicKey,
        buyerTokenAccount: buyerTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return tx;
  }

  /**
   * Redeem PSP tokens for SOL
   */
  async redeemTokens(tokenAmount: BN): Promise<string> {
    const [statePDA] = await PDAHelper.getPSPTokenState();

    const userTokenAccount = await getAssociatedTokenAddress(
      statePDA,
      this.provider.wallet.publicKey
    );

    const tx = await this.program.methods
      .redeemTokens(tokenAmount)
      .accounts({
        state: statePDA,
        mint: statePDA,
        user: this.provider.wallet.publicKey,
        userTokenAccount: userTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    return tx;
  }
}

/**
 * NFT Marketplace SDK
 */
export class NFTMarketplaceSDK {
  constructor(
    private program: Program,
    private provider: AnchorProvider
  ) {}

  /**
   * List an NFT for sale
   */
  async listNFT(nftMint: PublicKey, price: BN): Promise<string> {
    const [statePDA] = await PDAHelper.getMarketplaceState();
    const [listingPDA] = await PDAHelper.getListing(nftMint);

    const sellerNftAccount = await getAssociatedTokenAddress(
      nftMint,
      this.provider.wallet.publicKey
    );

    const escrowNftAccount = await getAssociatedTokenAddress(
      nftMint,
      listingPDA,
      true
    );

    const tx = await this.program.methods
      .listNft(price)
      .accounts({
        state: statePDA,
        listing: listingPDA,
        nftMint: nftMint,
        seller: this.provider.wallet.publicKey,
        sellerNftAccount: sellerNftAccount,
        escrowNftAccount: escrowNftAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    return tx;
  }

  /**
   * Buy an NFT
   */
  async buyNFT(nftMint: PublicKey, seller: PublicKey): Promise<string> {
    const [statePDA] = await PDAHelper.getMarketplaceState();
    const [listingPDA] = await PDAHelper.getListing(nftMint);

    const buyerNftAccount = await getAssociatedTokenAddress(
      nftMint,
      this.provider.wallet.publicKey
    );

    const escrowNftAccount = await getAssociatedTokenAddress(
      nftMint,
      listingPDA,
      true
    );

    // Fetch state to get fee recipient
    const state = await this.program.account.marketplaceState.fetch(statePDA);

    const tx = await this.program.methods
      .buyNft()
      .accounts({
        state: statePDA,
        listing: listingPDA,
        buyer: this.provider.wallet.publicKey,
        seller: seller,
        feeRecipient: state.feeRecipient,
        buyerNftAccount: buyerNftAccount,
        escrowNftAccount: escrowNftAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return tx;
  }

  /**
   * Cancel a listing
   */
  async cancelListing(nftMint: PublicKey): Promise<string> {
    const [listingPDA] = await PDAHelper.getListing(nftMint);

    const sellerNftAccount = await getAssociatedTokenAddress(
      nftMint,
      this.provider.wallet.publicKey
    );

    const escrowNftAccount = await getAssociatedTokenAddress(
      nftMint,
      listingPDA,
      true
    );

    const tx = await this.program.methods
      .cancelListing()
      .accounts({
        listing: listingPDA,
        seller: this.provider.wallet.publicKey,
        sellerNftAccount: sellerNftAccount,
        escrowNftAccount: escrowNftAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    return tx;
  }
}

/**
 * Search Payment SDK
 */
export class SearchPaymentSDK {
  constructor(
    private program: Program,
    private provider: AnchorProvider
  ) {}

  /**
   * Pay for search with SOL
   */
  async payWithSOL(): Promise<string> {
    const [statePDA] = await PDAHelper.getSearchPaymentState();
    const [userStatsPDA] = await PDAHelper.getUserStats(this.provider.wallet.publicKey);

    const tx = await this.program.methods
      .payWithSol()
      .accounts({
        state: statePDA,
        userStats: userStatsPDA,
        user: this.provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return tx;
  }

  /**
   * Pay for search with PSP tokens
   */
  async payWithPSP(pspMint: PublicKey): Promise<string> {
    const [statePDA] = await PDAHelper.getSearchPaymentState();
    const [userStatsPDA] = await PDAHelper.getUserStats(this.provider.wallet.publicKey);

    const userPspAccount = await getAssociatedTokenAddress(
      pspMint,
      this.provider.wallet.publicKey
    );

    const programPspAccount = await getAssociatedTokenAddress(
      pspMint,
      statePDA,
      true
    );

    const tx = await this.program.methods
      .payWithPsp()
      .accounts({
        state: statePDA,
        userStats: userStatsPDA,
        user: this.provider.wallet.publicKey,
        userPspAccount: userPspAccount,
        programPspAccount: programPspAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return tx;
  }
}

/**
 * Main SDK class that combines all program SDKs
 */
export class NFTPatentsSolanaSDK {
  public patentNFT: PatentNFTSDK;
  public pspToken: PSPTokenSDK;
  public marketplace: NFTMarketplaceSDK;
  public searchPayment: SearchPaymentSDK;

  constructor(
    private connection: Connection,
    private wallet: any, // Wallet adapter
    private programs: {
      patentNFT: Program;
      pspToken: Program;
      marketplace: Program;
      searchPayment: Program;
    }
  ) {
    const provider = new AnchorProvider(connection, wallet, {});

    this.patentNFT = new PatentNFTSDK(programs.patentNFT, provider);
    this.pspToken = new PSPTokenSDK(programs.pspToken, provider);
    this.marketplace = new NFTMarketplaceSDK(programs.marketplace, provider);
    this.searchPayment = new SearchPaymentSDK(programs.searchPayment, provider);
  }
}


