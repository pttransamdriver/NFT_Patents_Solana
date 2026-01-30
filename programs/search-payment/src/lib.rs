use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("SrchPy111111111111111111111111111111111111");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PaymentToken {
    SOL,
    USDC,
    PSP,
}

#[program]
pub mod search_payment {
    use super::*;

    /// Initialize the search payment program
    pub fn initialize(
        ctx: Context<Initialize>,
        search_price_in_sol: u64,
        search_price_in_usdc: u64,
        search_price_in_psp: u64,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.psp_token_mint = ctx.accounts.psp_token_mint.key();
        state.usdc_token_mint = ctx.accounts.usdc_token_mint.key();
        state.search_price_in_sol = search_price_in_sol;
        state.search_price_in_usdc = search_price_in_usdc;
        state.search_price_in_psp = search_price_in_psp;
        state.searches_per_payment = 1;
        state.paused = false;
        state.bump = ctx.bumps.state;
        Ok(())
    }

    /// Pay for AI search with SOL
    pub fn pay_with_sol(ctx: Context<PayWithSOL>) -> Result<()> {
        let state = &ctx.accounts.state;

        require!(!state.paused, SearchPaymentError::ContractPaused);
        require!(
            state.search_price_in_sol > 0,
            SearchPaymentError::PriceNotSet
        );

        // Verify user has sufficient balance
        let user_balance = ctx.accounts.user.to_account_info().lamports();
        require!(
            user_balance >= state.search_price_in_sol,
            SearchPaymentError::InsufficientFunds
        );

        // Transfer SOL from user to program
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.state.key(),
            state.search_price_in_sol,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.state.to_account_info(),
            ],
        )?;

        // Update user stats with overflow protection
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.sol_paid = user_stats
            .sol_paid
            .checked_add(state.search_price_in_sol)
            .ok_or(SearchPaymentError::MathOverflow)?;
        user_stats.searches_purchased = user_stats
            .searches_purchased
            .checked_add(state.searches_per_payment)
            .ok_or(SearchPaymentError::MathOverflow)?;

        emit!(PaymentReceived {
            user: ctx.accounts.user.key(),
            payment_method: PaymentToken::SOL,
            amount: state.search_price_in_sol,
            search_credits: state.searches_per_payment,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Pay for AI search with USDC
    pub fn pay_with_usdc(ctx: Context<PayWithUSDC>) -> Result<()> {
        let state = &ctx.accounts.state;

        require!(!state.paused, SearchPaymentError::ContractPaused);
        require!(
            state.search_price_in_usdc > 0,
            SearchPaymentError::PriceNotSet
        );

        // Verify token accounts match expected mint
        require!(
            ctx.accounts.user_usdc_account.mint == state.usdc_token_mint,
            SearchPaymentError::InvalidTokenAccount
        );
        require!(
            ctx.accounts.program_usdc_account.mint == state.usdc_token_mint,
            SearchPaymentError::InvalidTokenAccount
        );

        // Verify token account ownership
        require!(
            ctx.accounts.user_usdc_account.owner == ctx.accounts.user.key(),
            SearchPaymentError::InvalidTokenAccount
        );

        // Verify user has sufficient balance
        require!(
            ctx.accounts.user_usdc_account.amount >= state.search_price_in_usdc,
            SearchPaymentError::InsufficientFunds
        );

        // Transfer USDC from user to program
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_usdc_account.to_account_info(),
            to: ctx.accounts.program_usdc_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, state.search_price_in_usdc)?;

        // Update user stats with overflow protection
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.usdc_paid = user_stats
            .usdc_paid
            .checked_add(state.search_price_in_usdc)
            .ok_or(SearchPaymentError::MathOverflow)?;
        user_stats.searches_purchased = user_stats
            .searches_purchased
            .checked_add(state.searches_per_payment)
            .ok_or(SearchPaymentError::MathOverflow)?;

        emit!(PaymentReceived {
            user: ctx.accounts.user.key(),
            payment_method: PaymentToken::USDC,
            amount: state.search_price_in_usdc,
            search_credits: state.searches_per_payment,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Pay for AI search with PSP tokens
    pub fn pay_with_psp(ctx: Context<PayWithPSP>) -> Result<()> {
        let state = &ctx.accounts.state;

        require!(!state.paused, SearchPaymentError::ContractPaused);
        require!(
            state.search_price_in_psp > 0,
            SearchPaymentError::PriceNotSet
        );

        // Verify token accounts match expected mint
        require!(
            ctx.accounts.user_psp_account.mint == state.psp_token_mint,
            SearchPaymentError::InvalidTokenAccount
        );
        require!(
            ctx.accounts.program_psp_account.mint == state.psp_token_mint,
            SearchPaymentError::InvalidTokenAccount
        );

        // Verify token account ownership
        require!(
            ctx.accounts.user_psp_account.owner == ctx.accounts.user.key(),
            SearchPaymentError::InvalidTokenAccount
        );

        // Verify user has sufficient balance
        require!(
            ctx.accounts.user_psp_account.amount >= state.search_price_in_psp,
            SearchPaymentError::InsufficientFunds
        );

        // Transfer PSP from user to program
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_psp_account.to_account_info(),
            to: ctx.accounts.program_psp_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, state.search_price_in_psp)?;

        // Update user stats with overflow protection
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.psp_paid = user_stats
            .psp_paid
            .checked_add(state.search_price_in_psp)
            .ok_or(SearchPaymentError::MathOverflow)?;
        user_stats.searches_purchased = user_stats
            .searches_purchased
            .checked_add(state.searches_per_payment)
            .ok_or(SearchPaymentError::MathOverflow)?;

        emit!(PaymentReceived {
            user: ctx.accounts.user.key(),
            payment_method: PaymentToken::PSP,
            amount: state.search_price_in_psp,
            search_credits: state.searches_per_payment,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Update search price for a specific payment method
    pub fn update_search_price(
        ctx: Context<UpdateState>,
        payment_token: PaymentToken,
        new_price: u64,
    ) -> Result<()> {
        require!(new_price > 0, SearchPaymentError::InvalidPrice);

        let state = &mut ctx.accounts.state;
        let old_price;

        match payment_token {
            PaymentToken::SOL => {
                old_price = state.search_price_in_sol;
                state.search_price_in_sol = new_price;
            }
            PaymentToken::USDC => {
                old_price = state.search_price_in_usdc;
                state.search_price_in_usdc = new_price;
            }
            PaymentToken::PSP => {
                old_price = state.search_price_in_psp;
                state.search_price_in_psp = new_price;
            }
        }

        emit!(PriceUpdated {
            token: payment_token,
            old_price,
            new_price,
        });

        Ok(())
    }

    /// Update token mint addresses
    pub fn update_token_address(
        ctx: Context<UpdateState>,
        payment_token: PaymentToken,
        new_address: Pubkey,
    ) -> Result<()> {
        let state = &mut ctx.accounts.state;

        match payment_token {
            PaymentToken::SOL => {
                return Err(SearchPaymentError::CannotUpdateSOL.into());
            }
            PaymentToken::USDC => {
                state.usdc_token_mint = new_address;
            }
            PaymentToken::PSP => {
                state.psp_token_mint = new_address;
            }
        }

        Ok(())
    }

    /// Withdraw SOL from contract
    pub fn withdraw_sol(ctx: Context<WithdrawSOL>, amount: u64) -> Result<()> {
        require!(amount > 0, SearchPaymentError::InvalidAmount);

        let state_account = ctx.accounts.state.to_account_info();

        // Calculate minimum rent-exempt balance
        let rent = Rent::get()?;
        let min_balance = rent.minimum_balance(state_account.data_len());

        // Ensure we don't withdraw below rent-exempt minimum
        let current_balance = state_account.lamports();
        require!(
            current_balance >= amount.checked_add(min_balance).ok_or(SearchPaymentError::MathOverflow)?,
            SearchPaymentError::InsufficientBalance
        );

        **state_account.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.authority.to_account_info().try_borrow_mut_lamports()? += amount;

        emit!(TokensWithdrawn {
            owner: ctx.accounts.authority.key(),
            token: PaymentToken::SOL,
            amount,
        });

        Ok(())
    }

    /// Withdraw USDC from contract
    pub fn withdraw_usdc(ctx: Context<WithdrawToken>) -> Result<()> {
        let state = &ctx.accounts.state;
        let amount = ctx.accounts.program_token_account.amount;

        require!(amount > 0, SearchPaymentError::InsufficientBalance);

        let seeds = &[b"state".as_ref(), &[state.bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.program_token_account.to_account_info(),
            to: ctx.accounts.authority_token_account.to_account_info(),
            authority: ctx.accounts.state.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount)?;

        emit!(TokensWithdrawn {
            owner: ctx.accounts.authority.key(),
            token: PaymentToken::USDC,
            amount,
        });

        Ok(())
    }

    /// Withdraw PSP from contract
    pub fn withdraw_psp(ctx: Context<WithdrawToken>) -> Result<()> {
        let state = &ctx.accounts.state;
        let amount = ctx.accounts.program_token_account.amount;

        require!(amount > 0, SearchPaymentError::InsufficientBalance);

        let seeds = &[b"state".as_ref(), &[state.bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.program_token_account.to_account_info(),
            to: ctx.accounts.authority_token_account.to_account_info(),
            authority: ctx.accounts.state.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount)?;

        emit!(TokensWithdrawn {
            owner: ctx.accounts.authority.key(),
            token: PaymentToken::PSP,
            amount,
        });

        Ok(())
    }

    /// Pause contract
    pub fn pause(ctx: Context<UpdateState>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.paused = true;
        Ok(())
    }

    /// Unpause contract
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

    /// CHECK: PSP token mint
    pub psp_token_mint: AccountInfo<'info>,

    /// CHECK: USDC token mint
    pub usdc_token_mint: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PayWithSOL<'info> {
    #[account(
        seeds = [b"state"],
        bump = state.bump
    )]
    pub state: Account<'info, ProgramState>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserStats::INIT_SPACE,
        seeds = [b"user_stats", user.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PayWithUSDC<'info> {
    #[account(
        seeds = [b"state"],
        bump = state.bump
    )]
    pub state: Account<'info, ProgramState>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserStats::INIT_SPACE,
        seeds = [b"user_stats", user.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_usdc_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub program_usdc_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PayWithPSP<'info> {
    #[account(
        seeds = [b"state"],
        bump = state.bump
    )]
    pub state: Account<'info, ProgramState>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserStats::INIT_SPACE,
        seeds = [b"user_stats", user.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_psp_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub program_psp_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
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
pub struct WithdrawSOL<'info> {
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

#[derive(Accounts)]
pub struct WithdrawToken<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump,
        has_one = authority
    )]
    pub state: Account<'info, ProgramState>,

    #[account(mut)]
    pub program_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority_token_account: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

// State Accounts
#[account]
#[derive(InitSpace)]
pub struct ProgramState {
    pub authority: Pubkey,
    pub psp_token_mint: Pubkey,
    pub usdc_token_mint: Pubkey,
    pub search_price_in_sol: u64,
    pub search_price_in_usdc: u64,
    pub search_price_in_psp: u64,
    pub searches_per_payment: u64,
    pub paused: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct UserStats {
    pub sol_paid: u64,
    pub usdc_paid: u64,
    pub psp_paid: u64,
    pub searches_purchased: u64,
}

// Events
#[event]
pub struct PaymentReceived {
    pub user: Pubkey,
    pub payment_method: PaymentToken,
    pub amount: u64,
    pub search_credits: u64,
    pub timestamp: i64,
}

#[event]
pub struct PriceUpdated {
    pub token: PaymentToken,
    pub old_price: u64,
    pub new_price: u64,
}

#[event]
pub struct TokensWithdrawn {
    pub owner: Pubkey,
    pub token: PaymentToken,
    pub amount: u64,
}

// Errors
#[error_code]
pub enum SearchPaymentError {
    #[msg("Contract is paused")]
    ContractPaused,
    #[msg("Price not set")]
    PriceNotSet,
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Cannot update SOL address")]
    CannotUpdateSOL,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Invalid amount")]
    InvalidAmount,
}


