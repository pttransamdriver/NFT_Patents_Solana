use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use mpl_token_metadata::instructions::{
    CreateV1CpiBuilder, CreateV1InstructionArgs,
};
use mpl_token_metadata::types::{PrintSupply, TokenStandard};

declare_id!("PatNFT111111111111111111111111111111111111");

#[program]
pub mod patent_nft {
    use super::*;

    /// Initialize the patent NFT program state
    pub fn initialize(ctx: Context<Initialize>, minting_price: u64, platform_fee_percentage: u16) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.minting_price = minting_price;
        state.platform_fee_percentage = platform_fee_percentage;
        state.next_token_id = 1;
        state.bump = ctx.bumps.state;
        Ok(())
    }

    /// Mint a new Patent NFT (public, requires payment)
    pub fn mint_patent_nft(
        ctx: Context<MintPatentNFT>,
        patent_number: String,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        // Input validation
        require!(
            patent_number.len() > 0 && patent_number.len() <= 50,
            PatentNFTError::InvalidPatentNumber
        );
        require!(
            name.len() > 0 && name.len() <= 32,
            PatentNFTError::InvalidName
        );
        require!(
            symbol.len() > 0 && symbol.len() <= 10,
            PatentNFTError::InvalidSymbol
        );
        require!(
            uri.len() > 0 && uri.len() <= 200,
            PatentNFTError::InvalidUri
        );

        let state = &mut ctx.accounts.state;

        // Normalize and hash patent number
        let patent_hash = normalize_patent_id(&patent_number);

        // Check if patent already minted (this is enforced by init constraint, but double-check)
        require!(
            ctx.accounts.patent_registry.token_id == 0,
            PatentNFTError::PatentAlreadyMinted
        );

        // Assign token ID with overflow protection
        let token_id = state.next_token_id;
        state.next_token_id = state
            .next_token_id
            .checked_add(1)
            .ok_or(PatentNFTError::TokenIdOverflow)?;

        // Transfer payment to authority FIRST (fail fast before state changes)
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.payer.key(),
            &ctx.accounts.authority.key(),
            state.minting_price,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.authority.to_account_info(),
            ],
        )?;

        // Store patent registry
        let registry = &mut ctx.accounts.patent_registry;
        registry.patent_hash = patent_hash;
        registry.token_id = token_id;
        registry.owner = ctx.accounts.payer.key();
        registry.patent_number = patent_number.clone();

        // Create NFT using Metaplex
        CreateV1CpiBuilder::new(&ctx.accounts.token_metadata_program.to_account_info())
            .metadata(&ctx.accounts.metadata.to_account_info())
            .master_edition(Some(&ctx.accounts.master_edition.to_account_info()))
            .mint(&ctx.accounts.mint.to_account_info(), true)
            .authority(&ctx.accounts.payer.to_account_info())
            .payer(&ctx.accounts.payer.to_account_info())
            .update_authority(&ctx.accounts.payer.to_account_info(), true)
            .system_program(&ctx.accounts.system_program.to_account_info())
            .sysvar_instructions(&ctx.accounts.sysvar_instructions.to_account_info())
            .spl_token_program(&ctx.accounts.token_program.to_account_info())
            .name(name)
            .symbol(symbol)
            .uri(uri)
            .seller_fee_basis_points(state.platform_fee_percentage)
            .token_standard(TokenStandard::NonFungible)
            .print_supply(PrintSupply::Zero)
            .invoke()?;

        emit!(PatentMinted {
            owner: ctx.accounts.payer.key(),
            token_id,
            patent_number,
            mint: ctx.accounts.mint.key(),
        });

        Ok(())
    }

    /// Admin-only mint function
    pub fn mint_patent_admin(
        ctx: Context<MintPatentAdmin>,
        patent_number: String,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        // Input validation
        require!(
            patent_number.len() > 0 && patent_number.len() <= 50,
            PatentNFTError::InvalidPatentNumber
        );
        require!(
            name.len() > 0 && name.len() <= 32,
            PatentNFTError::InvalidName
        );
        require!(
            symbol.len() > 0 && symbol.len() <= 10,
            PatentNFTError::InvalidSymbol
        );
        require!(
            uri.len() > 0 && uri.len() <= 200,
            PatentNFTError::InvalidUri
        );

        let state = &mut ctx.accounts.state;

        // Normalize and hash patent number
        let patent_hash = normalize_patent_id(&patent_number);

        // Check if patent already minted
        require!(
            ctx.accounts.patent_registry.token_id == 0,
            PatentNFTError::PatentAlreadyMinted
        );

        // Assign token ID with overflow protection
        let token_id = state.next_token_id;
        state.next_token_id = state
            .next_token_id
            .checked_add(1)
            .ok_or(PatentNFTError::TokenIdOverflow)?;

        // Store patent registry
        let registry = &mut ctx.accounts.patent_registry;
        registry.patent_hash = patent_hash;
        registry.token_id = token_id;
        registry.owner = ctx.accounts.recipient.key();
        registry.patent_number = patent_number.clone();

        // Create NFT using Metaplex
        CreateV1CpiBuilder::new(&ctx.accounts.token_metadata_program.to_account_info())
            .metadata(&ctx.accounts.metadata.to_account_info())
            .master_edition(Some(&ctx.accounts.master_edition.to_account_info()))
            .mint(&ctx.accounts.mint.to_account_info(), true)
            .authority(&ctx.accounts.authority.to_account_info())
            .payer(&ctx.accounts.authority.to_account_info())
            .update_authority(&ctx.accounts.authority.to_account_info(), true)
            .system_program(&ctx.accounts.system_program.to_account_info())
            .sysvar_instructions(&ctx.accounts.sysvar_instructions.to_account_info())
            .spl_token_program(&ctx.accounts.token_program.to_account_info())
            .name(name)
            .symbol(symbol)
            .uri(uri)
            .seller_fee_basis_points(state.platform_fee_percentage)
            .token_standard(TokenStandard::NonFungible)
            .print_supply(PrintSupply::Zero)
            .invoke()?;

        emit!(PatentMinted {
            owner: ctx.accounts.recipient.key(),
            token_id,
            patent_number,
            mint: ctx.accounts.mint.key(),
        });

        Ok(())
    }

    /// Update minting price (admin only)
    pub fn update_minting_price(ctx: Context<UpdateState>, new_price: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let old_price = state.minting_price;
        state.minting_price = new_price;

        emit!(MintingPriceUpdated {
            old_price,
            new_price,
        });

        Ok(())
    }

    /// Withdraw accumulated fees (admin only)
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        require!(amount > 0, PatentNFTError::InvalidAmount);

        let state = &ctx.accounts.state;
        let state_account = ctx.accounts.state.to_account_info();

        // Calculate minimum rent-exempt balance
        let rent = Rent::get()?;
        let min_balance = rent.minimum_balance(state_account.data_len());

        // Ensure we don't withdraw below rent-exempt minimum
        let current_balance = state_account.lamports();
        require!(
            current_balance >= amount.checked_add(min_balance).ok_or(PatentNFTError::MathOverflow)?,
            PatentNFTError::InsufficientBalance
        );

        // Perform withdrawal
        **state_account.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.authority.to_account_info().try_borrow_mut_lamports()? += amount;

        emit!(FeeWithdrawn {
            recipient: ctx.accounts.authority.key(),
            amount,
        });

        Ok(())
    }
}

// Helper function to normalize patent ID
fn normalize_patent_id(patent_number: &str) -> [u8; 32] {
    let normalized: String = patent_number
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .map(|c| c.to_ascii_uppercase())
        .collect();

    use anchor_lang::solana_program::hash::hash;
    hash(normalized.as_bytes()).to_bytes()
}

// Account Contexts
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ProgramState::INIT_SPACE,
        seeds = [b"state"],
        bump
    )]
    pub state: Account<'info, ProgramState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(patent_number: String)]
pub struct MintPatentNFT<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump
    )]
    pub state: Account<'info, ProgramState>,

    #[account(
        init,
        payer = payer,
        space = 8 + PatentRegistry::INIT_SPACE,
        seeds = [b"patent", normalize_patent_id(&patent_number).as_ref()],
        bump
    )]
    pub patent_registry: Account<'info, PatentRegistry>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Authority receives payment
    #[account(mut, address = state.authority)]
    pub authority: AccountInfo<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer,
        mint::freeze_authority = payer,
    )]
    pub mint: Account<'info, Mint>,

    /// CHECK: Metadata account
    #[account(mut)]
    pub metadata: AccountInfo<'info>,

    /// CHECK: Master edition account
    #[account(mut)]
    pub master_edition: AccountInfo<'info>,

    /// CHECK: Token Metadata Program
    pub token_metadata_program: AccountInfo<'info>,

    /// CHECK: Sysvar instructions
    pub sysvar_instructions: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(patent_number: String)]
pub struct MintPatentAdmin<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump,
        has_one = authority
    )]
    pub state: Account<'info, ProgramState>,

    #[account(
        init,
        payer = authority,
        space = 8 + PatentRegistry::INIT_SPACE,
        seeds = [b"patent", normalize_patent_id(&patent_number).as_ref()],
        bump
    )]
    pub patent_registry: Account<'info, PatentRegistry>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Recipient of the NFT
    pub recipient: AccountInfo<'info>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
    )]
    pub mint: Account<'info, Mint>,

    /// CHECK: Metadata account
    #[account(mut)]
    pub metadata: AccountInfo<'info>,

    /// CHECK: Master edition account
    #[account(mut)]
    pub master_edition: AccountInfo<'info>,

    /// CHECK: Token Metadata Program
    pub token_metadata_program: AccountInfo<'info>,

    /// CHECK: Sysvar instructions
    pub sysvar_instructions: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UpdateState<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump,
        has_one = authority
    )]
    pub state: Account<'info, ProgramState>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump,
        has_one = authority
    )]
    pub state: Account<'info, ProgramState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// State Accounts
#[account]
#[derive(InitSpace)]
pub struct ProgramState {
    pub authority: Pubkey,
    pub minting_price: u64,
    pub platform_fee_percentage: u16,
    pub next_token_id: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PatentRegistry {
    pub patent_hash: [u8; 32],
    pub token_id: u64,
    pub owner: Pubkey,
    #[max_len(50)]
    pub patent_number: String,
}

// Events
#[event]
pub struct PatentMinted {
    pub owner: Pubkey,
    pub token_id: u64,
    pub patent_number: String,
    pub mint: Pubkey,
}

#[event]
pub struct MintingPriceUpdated {
    pub old_price: u64,
    pub new_price: u64,
}

#[event]
pub struct FeeWithdrawn {
    pub recipient: Pubkey,
    pub amount: u64,
}

// Errors
#[error_code]
pub enum PatentNFTError {
    #[msg("Insufficient payment for minting")]
    InsufficientPayment,
    #[msg("Patent already minted")]
    PatentAlreadyMinted,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid patent number")]
    InvalidPatentNumber,
    #[msg("Invalid name")]
    InvalidName,
    #[msg("Invalid symbol")]
    InvalidSymbol,
    #[msg("Invalid URI")]
    InvalidUri,
    #[msg("Token ID overflow")]
    TokenIdOverflow,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Math overflow")]
    MathOverflow,
}


