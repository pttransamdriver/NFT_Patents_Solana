use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount, Transfer};

declare_id!("PSPTok111111111111111111111111111111111111");

/// 1 PSP = $0.01 USD
/// 500 PSP = $5.00 for one AI search
#[program]
pub mod psp_token {
    use super::*;

    /// Initialize the PSP token program
    pub fn initialize(
        ctx: Context<Initialize>,
        token_price_in_lamports: u64,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.token_price_in_lamports = token_price_in_lamports;
        state.paused = false;
        state.bump = ctx.bumps.state;
        Ok(())
    }

    /// Purchase PSP tokens with SOL
    pub fn purchase_tokens(ctx: Context<PurchaseTokens>, sol_amount: u64) -> Result<()> {
        let state = &ctx.accounts.state;

        require!(!state.paused, PSPTokenError::ContractPaused);
        require!(sol_amount > 0, PSPTokenError::InvalidAmount);

        // Calculate token amount
        let token_amount = sol_amount
            .checked_mul(10u64.pow(9)) // SPL token decimals
            .ok_or(PSPTokenError::MathOverflow)?
            .checked_div(state.token_price_in_lamports)
            .ok_or(PSPTokenError::MathOverflow)?;

        require!(token_amount > 0, PSPTokenError::InsufficientPayment);

        // Check max supply with overflow protection
        let current_supply = ctx.accounts.mint.supply;
        let max_supply = 10_000_000 * 10u64.pow(9); // 10 million PSP
        let new_supply = current_supply
            .checked_add(token_amount)
            .ok_or(PSPTokenError::MathOverflow)?;
        require!(
            new_supply <= max_supply,
            PSPTokenError::MaxSupplyExceeded
        );

        // Verify buyer has sufficient balance
        let buyer_balance = ctx.accounts.buyer.to_account_info().lamports();
        require!(
            buyer_balance >= sol_amount,
            PSPTokenError::InsufficientFunds
        );

        // Verify token account ownership
        require!(
            ctx.accounts.buyer_token_account.owner == ctx.accounts.buyer.key(),
            PSPTokenError::InvalidTokenAccount
        );

        // Transfer SOL from buyer to program
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.buyer.key(),
            &ctx.accounts.state.key(),
            sol_amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.state.to_account_info(),
            ],
        )?;

        // Mint tokens to buyer
        let seeds = &[b"state".as_ref(), &[state.bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.state.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::mint_to(cpi_ctx, token_amount)?;

        emit!(TokensPurchased {
            buyer: ctx.accounts.buyer.key(),
            amount: token_amount,
            sol_paid: sol_amount,
        });

        Ok(())
    }

    /// Redeem PSP tokens for SOL
    pub fn redeem_tokens(ctx: Context<RedeemTokens>, token_amount: u64) -> Result<()> {
        let state = &ctx.accounts.state;

        require!(!state.paused, PSPTokenError::ContractPaused);
        require!(token_amount > 0, PSPTokenError::InvalidAmount);

        // Calculate SOL amount
        let sol_amount = token_amount
            .checked_mul(state.token_price_in_lamports)
            .ok_or(PSPTokenError::MathOverflow)?
            .checked_div(10u64.pow(9))
            .ok_or(PSPTokenError::MathOverflow)?;

        // Verify token account ownership
        require!(
            ctx.accounts.user_token_account.owner == ctx.accounts.user.key(),
            PSPTokenError::InvalidTokenAccount
        );

        // Verify user has sufficient tokens
        require!(
            ctx.accounts.user_token_account.amount >= token_amount,
            PSPTokenError::InsufficientTokenBalance
        );

        // Calculate minimum rent-exempt balance for state account
        let state_account = ctx.accounts.state.to_account_info();
        let rent = Rent::get()?;
        let min_balance = rent.minimum_balance(state_account.data_len());

        // Ensure contract has enough balance (including rent-exempt minimum)
        let current_balance = state_account.lamports();
        require!(
            current_balance >= sol_amount.checked_add(min_balance).ok_or(PSPTokenError::MathOverflow)?,
            PSPTokenError::InsufficientContractBalance
        );

        // Burn tokens from user FIRST (fail fast)
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::burn(cpi_ctx, token_amount)?;

        // Transfer SOL to user
        **state_account.try_borrow_mut_lamports()? -= sol_amount;
        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += sol_amount;

        emit!(TokensRedeemed {
            user: ctx.accounts.user.key(),
            amount: token_amount,
            sol_received: sol_amount,
        });

        Ok(())
    }

    /// Spend tokens on behalf of user (for authorized contracts)
    pub fn spend_tokens_for(ctx: Context<SpendTokensFor>, amount: u64) -> Result<()> {
        let state = &ctx.accounts.state;

        require!(!state.paused, PSPTokenError::ContractPaused);
        require!(amount > 0, PSPTokenError::InvalidAmount);

        // Check if spender is authorized
        let spender_state = &ctx.accounts.spender_state;
        require!(spender_state.authorized, PSPTokenError::UnauthorizedSpender);

        // Verify token account ownership
        require!(
            ctx.accounts.user_token_account.owner == ctx.accounts.user.key(),
            PSPTokenError::InvalidTokenAccount
        );

        // Verify user has sufficient tokens
        require!(
            ctx.accounts.user_token_account.amount >= amount,
            PSPTokenError::InsufficientTokenBalance
        );

        // Transfer tokens from user to program
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.program_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        // Burn the tokens
        let seeds = &[b"state".as_ref(), &[state.bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.program_token_account.to_account_info(),
            authority: ctx.accounts.state.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::burn(cpi_ctx, amount)?;

        Ok(())
    }

    /// Set authorized spender
    pub fn set_authorized_spender(
        ctx: Context<SetAuthorizedSpender>,
        authorized: bool,
    ) -> Result<()> {
        let spender_state = &mut ctx.accounts.spender_state;
        spender_state.spender = ctx.accounts.spender.key();
        spender_state.authorized = authorized;
        Ok(())
    }

    /// Update token price (admin only)
    pub fn update_token_price(ctx: Context<UpdateState>, new_price: u64) -> Result<()> {
        require!(new_price > 0, PSPTokenError::InvalidAmount);

        let state = &mut ctx.accounts.state;
        let old_price = state.token_price_in_lamports;
        state.token_price_in_lamports = new_price;

        emit!(PriceUpdated {
            old_price,
            new_price,
        });

        Ok(())
    }

    /// Mint additional tokens (admin only)
    pub fn mint(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let state = &ctx.accounts.state;

        // Check max supply
        let current_supply = ctx.accounts.mint.supply;
        let max_supply = 10_000_000 * 10u64.pow(9);
        require!(
            current_supply + amount <= max_supply,
            PSPTokenError::MaxSupplyExceeded
        );

        let seeds = &[b"state".as_ref(), &[state.bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.state.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::mint_to(cpi_ctx, amount)?;

        Ok(())
    }

    /// Withdraw SOL from contract (admin only)
    pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
        require!(amount > 0, PSPTokenError::InvalidAmount);

        let state_account = ctx.accounts.state.to_account_info();

        // Calculate minimum rent-exempt balance
        let rent = Rent::get()?;
        let min_balance = rent.minimum_balance(state_account.data_len());

        // Ensure we don't withdraw below rent-exempt minimum
        let current_balance = state_account.lamports();
        require!(
            current_balance >= amount.checked_add(min_balance).ok_or(PSPTokenError::MathOverflow)?,
            PSPTokenError::InsufficientContractBalance
        );

        **state_account.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.authority.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    /// Pause contract (admin only)
    pub fn pause(ctx: Context<UpdateState>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.paused = true;
        Ok(())
    }

    /// Unpause contract (admin only)
    pub fn unpause(ctx: Context<UpdateState>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.paused = false;
        Ok(())
    }
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

    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = state,
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct PurchaseTokens<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump
    )]
    pub state: Account<'info, ProgramState>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RedeemTokens<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump
    )]
    pub state: Account<'info, ProgramState>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SpendTokensFor<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump
    )]
    pub state: Account<'info, ProgramState>,

    #[account(
        seeds = [b"spender", spender.key().as_ref()],
        bump
    )]
    pub spender_state: Account<'info, SpenderState>,

    /// CHECK: The authorized spender program
    pub spender: AccountInfo<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub program_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SetAuthorizedSpender<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump,
        has_one = authority
    )]
    pub state: Account<'info, ProgramState>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + SpenderState::INIT_SPACE,
        seeds = [b"spender", spender.key().as_ref()],
        bump
    )]
    pub spender_state: Account<'info, SpenderState>,

    /// CHECK: The spender to authorize/deauthorize
    pub spender: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
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
pub struct MintTokens<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump,
        has_one = authority
    )]
    pub state: Account<'info, ProgramState>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub to: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
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
    pub token_price_in_lamports: u64,
    pub paused: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct SpenderState {
    pub spender: Pubkey,
    pub authorized: bool,
}

// Events
#[event]
pub struct TokensPurchased {
    pub buyer: Pubkey,
    pub amount: u64,
    pub sol_paid: u64,
}

#[event]
pub struct TokensRedeemed {
    pub user: Pubkey,
    pub amount: u64,
    pub sol_received: u64,
}

#[event]
pub struct PriceUpdated {
    pub old_price: u64,
    pub new_price: u64,
}

// Errors
#[error_code]
pub enum PSPTokenError {
    #[msg("Contract is paused")]
    ContractPaused,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Insufficient payment")]
    InsufficientPayment,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Max supply exceeded")]
    MaxSupplyExceeded,
    #[msg("Insufficient contract balance")]
    InsufficientContractBalance,
    #[msg("Unauthorized spender")]
    UnauthorizedSpender,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("Insufficient token balance")]
    InsufficientTokenBalance,
}


