use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("MktPla111111111111111111111111111111111111");

#[program]
pub mod nft_marketplace {
    use super::*;

    /// Initialize the marketplace
    pub fn initialize(
        ctx: Context<Initialize>,
        platform_fee_percent: u16,
    ) -> Result<()> {
        require!(
            platform_fee_percent <= 1000,
            MarketplaceError::FeeTooHigh
        );

        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.fee_recipient = ctx.accounts.fee_recipient.key();
        state.platform_fee_percent = platform_fee_percent;
        state.listing_count = 0;
        state.bump = ctx.bumps.state;
        Ok(())
    }

    /// List an NFT for sale
    pub fn list_nft(
        ctx: Context<ListNFT>,
        price: u64,
    ) -> Result<()> {
        require!(price > 0, MarketplaceError::InvalidPrice);

        let state = &mut ctx.accounts.state;
        state.listing_count += 1;

        let listing = &mut ctx.accounts.listing;
        listing.listing_id = state.listing_count;
        listing.nft_mint = ctx.accounts.nft_mint.key();
        listing.seller = ctx.accounts.seller.key();
        listing.price = price;
        listing.active = true;
        listing.bump = ctx.bumps.listing;

        // Transfer NFT to escrow
        let cpi_accounts = Transfer {
            from: ctx.accounts.seller_nft_account.to_account_info(),
            to: ctx.accounts.escrow_nft_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        emit!(NFTListed {
            listing_id: listing.listing_id,
            nft_mint: listing.nft_mint,
            seller: listing.seller,
            price: listing.price,
        });

        Ok(())
    }

    /// Buy an NFT
    pub fn buy_nft(ctx: Context<BuyNFT>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        
        require!(listing.active, MarketplaceError::ListingNotActive);
        require!(
            ctx.accounts.buyer.key() != listing.seller,
            MarketplaceError::CannotBuyOwnNFT
        );

        let state = &ctx.accounts.state;
        
        // Calculate fees
        let platform_fee = (listing.price as u128)
            .checked_mul(state.platform_fee_percent as u128)
            .ok_or(MarketplaceError::MathOverflow)?
            .checked_div(10000)
            .ok_or(MarketplaceError::MathOverflow)? as u64;
        
        let seller_amount = listing.price
            .checked_sub(platform_fee)
            .ok_or(MarketplaceError::MathOverflow)?;

        // Transfer SOL from buyer to seller
        **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? -= listing.price;
        **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += seller_amount;
        **ctx.accounts.fee_recipient.to_account_info().try_borrow_mut_lamports()? += platform_fee;

        // Transfer NFT from escrow to buyer
        let seeds = &[
            b"listing",
            listing.nft_mint.as_ref(),
            &[listing.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_nft_account.to_account_info(),
            to: ctx.accounts.buyer_nft_account.to_account_info(),
            authority: ctx.accounts.listing.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, 1)?;

        listing.active = false;

        emit!(NFTSold {
            listing_id: listing.listing_id,
            nft_mint: listing.nft_mint,
            seller: listing.seller,
            buyer: ctx.accounts.buyer.key(),
            price: listing.price,
        });

        Ok(())
    }

    /// Cancel a listing
    pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        
        require!(listing.active, MarketplaceError::ListingNotActive);

        // Transfer NFT back to seller
        let seeds = &[
            b"listing",
            listing.nft_mint.as_ref(),
            &[listing.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_nft_account.to_account_info(),
            to: ctx.accounts.seller_nft_account.to_account_info(),
            authority: ctx.accounts.listing.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, 1)?;

        listing.active = false;

        emit!(ListingCancelled {
            listing_id: listing.listing_id,
        });

        Ok(())
    }

    /// Update listing price
    pub fn update_price(ctx: Context<UpdatePrice>, new_price: u64) -> Result<()> {
        require!(new_price > 0, MarketplaceError::InvalidPrice);

        let listing = &mut ctx.accounts.listing;
        require!(listing.active, MarketplaceError::ListingNotActive);

        listing.price = new_price;

        Ok(())
    }

    /// Set platform fee (admin only)
    pub fn set_platform_fee(ctx: Context<UpdateState>, new_fee: u16) -> Result<()> {
        require!(new_fee <= 1000, MarketplaceError::FeeTooHigh);

        let state = &mut ctx.accounts.state;
        state.platform_fee_percent = new_fee;

        Ok(())
    }

    /// Set fee recipient (admin only)
    pub fn set_fee_recipient(ctx: Context<UpdateState>, new_recipient: Pubkey) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.fee_recipient = new_recipient;

        Ok(())
    }
}

// Account Contexts
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + MarketplaceState::INIT_SPACE,
        seeds = [b"marketplace"],
        bump
    )]
    pub state: Account<'info, MarketplaceState>,

    /// CHECK: Fee recipient
    pub fee_recipient: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ListNFT<'info> {
    #[account(
        mut,
        seeds = [b"marketplace"],
        bump = state.bump
    )]
    pub state: Account<'info, MarketplaceState>,

    #[account(
        init,
        payer = seller,
        space = 8 + Listing::INIT_SPACE,
        seeds = [b"listing", nft_mint.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,

    /// CHECK: NFT mint
    pub nft_mint: AccountInfo<'info>,

    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(mut)]
    pub seller_nft_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = seller,
        token::mint = nft_mint,
        token::authority = listing,
    )]
    pub escrow_nft_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct BuyNFT<'info> {
    #[account(
        seeds = [b"marketplace"],
        bump = state.bump
    )]
    pub state: Account<'info, MarketplaceState>,

    #[account(
        mut,
        seeds = [b"listing", listing.nft_mint.as_ref()],
        bump = listing.bump
    )]
    pub listing: Account<'info, Listing>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK: Seller receives payment
    #[account(mut, address = listing.seller)]
    pub seller: AccountInfo<'info>,

    /// CHECK: Fee recipient
    #[account(mut, address = state.fee_recipient)]
    pub fee_recipient: AccountInfo<'info>,

    #[account(mut)]
    pub buyer_nft_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub escrow_nft_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelListing<'info> {
    #[account(
        mut,
        seeds = [b"listing", listing.nft_mint.as_ref()],
        bump = listing.bump,
        has_one = seller
    )]
    pub listing: Account<'info, Listing>,

    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(mut)]
    pub seller_nft_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub escrow_nft_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(
        mut,
        seeds = [b"listing", listing.nft_mint.as_ref()],
        bump = listing.bump,
        has_one = seller
    )]
    pub listing: Account<'info, Listing>,

    pub seller: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateState<'info> {
    #[account(
        mut,
        seeds = [b"marketplace"],
        bump = state.bump,
        has_one = authority
    )]
    pub state: Account<'info, MarketplaceState>,

    pub authority: Signer<'info>,
}

// State Accounts
#[account]
#[derive(InitSpace)]
pub struct MarketplaceState {
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,
    pub platform_fee_percent: u16,
    pub listing_count: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub listing_id: u64,
    pub nft_mint: Pubkey,
    pub seller: Pubkey,
    pub price: u64,
    pub active: bool,
    pub bump: u8,
}

// Events
#[event]
pub struct NFTListed {
    pub listing_id: u64,
    pub nft_mint: Pubkey,
    pub seller: Pubkey,
    pub price: u64,
}

#[event]
pub struct NFTSold {
    pub listing_id: u64,
    pub nft_mint: Pubkey,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub price: u64,
}

#[event]
pub struct ListingCancelled {
    pub listing_id: u64,
}

// Errors
#[error_code]
pub enum MarketplaceError {
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Listing not active")]
    ListingNotActive,
    #[msg("Cannot buy your own NFT")]
    CannotBuyOwnNFT,
    #[msg("Fee cannot exceed 10%")]
    FeeTooHigh,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Unauthorized")]
    Unauthorized,
}


