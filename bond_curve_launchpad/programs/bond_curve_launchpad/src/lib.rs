use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
    associated_token::AssociatedToken,
};
use solana_program::{
    program::invoke,
    system_instruction,
};

mod bond_curve;
mod wsol;
mod transfer_hook;
mod bundle_detection;

use bond_curve::{calculate_buy_price, calculate_sell_price, calculate_current_price, calculate_market_cap, is_eligible_for_graduation};
use wsol::{wrap_sol, unwrap_sol, get_wsol_mint};
use transfer_hook::initialize_transfer_hook;
use bundle_detection::{calculate_bundle_percentage, is_bundling, update_bundle_tracker, are_wallets_related};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod bond_curve_launchpad {
    use super::*;

    pub fn initialize_launchpad(
        ctx: Context<InitializeLaunchpad>,
        fee_recipient: Pubkey,
        bundle_threshold_percentage: u16,
        graduation_market_cap: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        config.authority = ctx.accounts.authority.key();
        config.fee_recipient = fee_recipient;
        config.bundle_threshold_percentage = bundle_threshold_percentage;
        config.graduation_market_cap = graduation_market_cap;
        config.trading_fee_bps = 100; // 1% trading fee
        config.relationship_threshold = 300; // 3% relationship threshold
        
        Ok(())
    }

    pub fn create_token_project(
        ctx: Context<CreateTokenProject>,
        name: String,
        symbol: String,
        initial_price: u64,
        curve_params: Vec<u64>,
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;
        let mint = &ctx.accounts.mint;
        
        // Initialize project data
        project.mint = mint.key();
        project.creator = ctx.accounts.authority.key();
        project.name = name;
        project.symbol = symbol;
        project.initial_price = initial_price;
        project.current_price = initial_price;
        project.supply = 0;
        project.reserve_balance = 0;
        project.curve_params = curve_params;
        project.is_graduated = false;
        project.liquidity_pool = None;
        project.creator_fee_earned = 0;
        project.platform_fee_earned = 0;
        
        // Initialize the mint with Token-2022 transfer hook
        initialize_transfer_hook(
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.authority.to_account_info(),
            &ctx.program_id,
            &ctx.accounts.token_program.to_account_info(),
        )?;
        
        Ok(())
    }

    pub fn buy_tokens(
        ctx: Context<BuyTokens>,
        amount: u64,
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;
        let config = &ctx.accounts.config;
        
        // Calculate the price based on the bond curve
        let price = calculate_buy_price(
            &project.curve_params,
            project.supply,
            amount,
        )?;
        
        // Check if buyer has enough SOL
        if ctx.accounts.buyer.lamports() < price {
            return Err(error!(ErrorCode::InsufficientFunds));
        }
        
        // Calculate 1% trading fee
        let total_fee = price
            .checked_mul(config.trading_fee_bps as u64)
            .ok_or(error!(ErrorCode::MathOverflow))?
            .checked_div(10000)
            .ok_or(error!(ErrorCode::DivisionByZero))?;
        
        // Split fee 50/50 between creator and platform
        let creator_fee = total_fee.checked_div(2).ok_or(error!(ErrorCode::DivisionByZero))?;
        let platform_fee = total_fee.checked_sub(creator_fee).ok_or(error!(ErrorCode::MathOverflow))?;
        
        let reserve_amount = price.checked_sub(total_fee).ok_or(error!(ErrorCode::MathOverflow))?;
        
        // Transfer SOL from buyer to project reserve
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.buyer.key(),
                &project.key(),
                reserve_amount,
            ),
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.project.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        
        // Transfer creator fee to creator
        if creator_fee > 0 {
            invoke(
                &system_instruction::transfer(
                    &ctx.accounts.buyer.key(),
                    &project.creator,
                    creator_fee,
                ),
                &[
                    ctx.accounts.buyer.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
            
            // Update creator fee earned
            project.creator_fee_earned = project.creator_fee_earned
                .checked_add(creator_fee)
                .ok_or(error!(ErrorCode::MathOverflow))?;
        }
        
        // Transfer platform fee to fee vault
        if platform_fee > 0 {
            invoke(
                &system_instruction::transfer(
                    &ctx.accounts.buyer.key(),
                    &ctx.accounts.fee_vault.key(),
                    platform_fee,
                ),
                &[
                    ctx.accounts.buyer.to_account_info(),
                    ctx.accounts.fee_vault.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
            
            // Update platform fee earned
            project.platform_fee_earned = project.platform_fee_earned
                .checked_add(platform_fee)
                .ok_or(error!(ErrorCode::MathOverflow))?;
        }
        
        // Mint tokens to buyer
        // In a real implementation, you would mint tokens to the buyer's token account
        // For simplicity, we'll just update the project state
        project.supply = project.supply.checked_add(amount).ok_or(error!(ErrorCode::MathOverflow))?;
        project.reserve_balance = project.reserve_balance.checked_add(reserve_amount).ok_or(error!(ErrorCode::MathOverflow))?;
        project.current_price = calculate_current_price(&project.curve_params, project.supply)?;
        
        // Check if the token is eligible for graduation
        if !project.is_graduated && is_eligible_for_graduation(project.supply, project.current_price, config.graduation_market_cap)? {
            // Mark as eligible for graduation
            // In a real implementation, you might want to emit an event or set a flag
            msg!("Token is now eligible for graduation!");
        }
        
        // Update bundle status if bundle tracker is provided
        if let Some(bundle_tracker) = &mut ctx.accounts.bundle_tracker {
            // Get the buyer's token balance after this purchase
            let new_balance = ctx.accounts.buyer_token_account.amount.checked_add(amount)
                .ok_or(error!(ErrorCode::MathOverflow))?;
                
            // For simplicity, we'll just update with the current wallet's balance
            // In a real implementation, you would track all related wallets
            let related_wallets = Vec::new();
            
            // Update the bundle tracker
            update_bundle_tracker(
                bundle_tracker,
                &ctx.accounts.buyer.key(),
                &project.mint,
                related_wallets,
                new_balance,
                config.bundle_threshold_percentage,
                project.supply
            )?;
        }
        
        Ok(())
    }

    pub fn sell_tokens(
        ctx: Context<SellTokens>,
        amount: u64,
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;
        let config = &ctx.accounts.config;
        
        // Check if the project has enough supply
        if project.supply < amount {
            return Err(error!(ErrorCode::InsufficientSupply));
        }
        
        // Calculate the sell price based on the bond curve
        let price = calculate_sell_price(
            &project.curve_params,
            project.supply,
            amount,
        )?;
        
        // Check if the project has enough reserve balance
        if project.reserve_balance < price {
            return Err(error!(ErrorCode::InsufficientFunds));
        }
        
        // Calculate 1% trading fee
        let total_fee = price
            .checked_mul(config.trading_fee_bps as u64)
            .ok_or(error!(ErrorCode::MathOverflow))?
            .checked_div(10000)
            .ok_or(error!(ErrorCode::DivisionByZero))?;
        
        // Split fee 50/50 between creator and platform
        let creator_fee = total_fee.checked_div(2).ok_or(error!(ErrorCode::DivisionByZero))?;
        let platform_fee = total_fee.checked_sub(creator_fee).ok_or(error!(ErrorCode::MathOverflow))?;
        
        let payout_amount = price.checked_sub(total_fee).ok_or(error!(ErrorCode::MathOverflow))?;
        
        // Transfer SOL from project reserve to seller
        **project.to_account_info().try_borrow_mut_lamports()? = project
            .to_account_info()
            .lamports()
            .checked_sub(payout_amount)
            .ok_or(error!(ErrorCode::InsufficientFunds))?;
            
        **ctx.accounts.seller.try_borrow_mut_lamports()? = ctx
            .accounts.seller
            .lamports()
            .checked_add(payout_amount)
            .ok_or(error!(ErrorCode::MathOverflow))?;
        
        // Transfer creator fee
        if creator_fee > 0 {
            **project.to_account_info().try_borrow_mut_lamports()? = project
                .to_account_info()
                .lamports()
                .checked_sub(creator_fee)
                .ok_or(error!(ErrorCode::InsufficientFunds))?;
                
            // In a real implementation, you would transfer to the creator
            // For simplicity, we'll just update the project state
            project.creator_fee_earned = project.creator_fee_earned
                .checked_add(creator_fee)
                .ok_or(error!(ErrorCode::MathOverflow))?;
        }
        
        // Transfer platform fee
        if platform_fee > 0 {
            **project.to_account_info().try_borrow_mut_lamports()? = project
                .to_account_info()
                .lamports()
                .checked_sub(platform_fee)
                .ok_or(error!(ErrorCode::InsufficientFunds))?;
                
            **ctx.accounts.fee_vault.try_borrow_mut_lamports()? = ctx
                .accounts.fee_vault
                .lamports()
                .checked_add(platform_fee)
                .ok_or(error!(ErrorCode::MathOverflow))?;
                
            project.platform_fee_earned = project.platform_fee_earned
                .checked_add(platform_fee)
                .ok_or(error!(ErrorCode::MathOverflow))?;
        }
        
        // Burn tokens from seller
        // In a real implementation, you would burn tokens from the seller's token account
        // For simplicity, we'll just update the project state
        project.supply = project.supply.checked_sub(amount).ok_or(error!(ErrorCode::MathOverflow))?;
        project.reserve_balance = project.reserve_balance.checked_sub(price).ok_or(error!(ErrorCode::MathOverflow))?;
        project.current_price = calculate_current_price(&project.curve_params, project.supply)?;
        
        // Update bundle status if bundle tracker is provided
        if let Some(bundle_tracker) = &mut ctx.accounts.bundle_tracker {
            // Get the seller's token balance after this sale
            let new_balance = ctx.accounts.seller_token_account.amount.checked_sub(amount)
                .ok_or(error!(ErrorCode::MathOverflow))?;
                
            // For simplicity, we'll just update with the current wallet's balance
            // In a real implementation, you would track all related wallets
            let related_wallets = Vec::new();
            
            // Update the bundle tracker
            update_bundle_tracker(
                bundle_tracker,
                &ctx.accounts.seller.key(),
                &project.mint,
                related_wallets,
                new_balance,
                config.bundle_threshold_percentage,
                project.supply
            )?;
        }
        
        Ok(())
    }

    pub fn register_wallet_relationship(
        ctx: Context<RegisterWalletRelationship>,
        mint: Pubkey,
        wallet_a: Pubkey,
        wallet_b: Pubkey,
        relationship_strength: u16,
        transaction_count: u16,
    ) -> Result<()> {
        let relationship = &mut ctx.accounts.relationship;
        
        // Only the launchpad authority can register wallet relationships
        if ctx.accounts.authority.key() != ctx.accounts.config.authority {
            return Err(error!(ErrorCode::UnauthorizedAccess));
        }
        
        relationship.mint = mint;
        relationship.wallet_a = wallet_a;
        relationship.wallet_b = wallet_b;
        relationship.relationship_strength = relationship_strength;
        relationship.last_transaction = Clock::get()?.unix_timestamp;
        relationship.transaction_count = transaction_count;
        
        Ok(())
    }

    pub fn update_bundle_status(
        ctx: Context<UpdateBundleStatus>,
        mint: Pubkey,
        wallet: Pubkey,
        related_wallets: Vec<Pubkey>,
        total_bundle_balance: u64,
    ) -> Result<()> {
        let bundle_tracker = &mut ctx.accounts.bundle_tracker;
        let config = &ctx.accounts.config;
        let project = &ctx.accounts.project;
        
        // Only the launchpad authority can update bundle status
        if ctx.accounts.authority.key() != config.authority {
            return Err(error!(ErrorCode::UnauthorizedAccess));
        }
        
        // Update the bundle tracker
        update_bundle_tracker(
            bundle_tracker,
            &wallet,
            &mint,
            related_wallets,
            total_bundle_balance,
            config.bundle_threshold_percentage,
            project.supply
        )?;
        
        Ok(())
    }

    pub fn graduate_token(
        ctx: Context<GraduateToken>,
        mint: Pubkey,
        liquidity_pool: Pubkey,
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;
        let config = &ctx.accounts.config;
        
        // Only the launchpad authority can graduate tokens
        if ctx.accounts.authority.key() != config.authority {
            return Err(error!(ErrorCode::UnauthorizedAccess));
        }
        
        // Check if the token is eligible for graduation
        if !is_eligible_for_graduation(project.supply, project.current_price, config.graduation_market_cap)? {
            return Err(error!(ErrorCode::TokenNotEligibleForGraduation));
        }
        
        // Update project state
        project.is_graduated = true;
        project.liquidity_pool = Some(liquidity_pool);
        
        // In a real implementation, you would:
        // 1. Create a liquidity pool on Raydium
        // 2. Transfer tokens and SOL to the pool
        // 3. Initialize the pool
        
        Ok(())
    }

    pub fn create_raydium_pool(
        ctx: Context<CreateRaydiumPool>,
        mint: Pubkey,
        initial_liquidity_amount: u64,
        initial_token_amount: u64,
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;
        let config = &ctx.accounts.config;
        
        // Only the launchpad authority can create liquidity pools
        if ctx.accounts.authority.key() != config.authority {
            return Err(error!(ErrorCode::UnauthorizedAccess));
        }
        
        // Check if the token is eligible for graduation
        if !is_eligible_for_graduation(project.supply, project.current_price, config.graduation_market_cap)? {
            return Err(error!(ErrorCode::TokenNotEligibleForGraduation));
        }
        
        // Check if the project has enough reserve balance for initial liquidity
        if project.reserve_balance < initial_liquidity_amount {
            return Err(error!(ErrorCode::InsufficientFunds));
        }
        
        // In a real implementation, you would:
        // 1. Create a Raydium CLMM pool
        // 2. Transfer SOL and tokens to the pool
        // 3. Initialize the pool with the desired parameters
        
        // For now, we'll just update the project state
        project.is_graduated = true;
        project.liquidity_pool = Some(ctx.accounts.liquidity_pool.key());
        
        // Transfer SOL from project reserve to liquidity pool
        **project.to_account_info().try_borrow_mut_lamports()? = project
            .to_account_info()
            .lamports()
            .checked_sub(initial_liquidity_amount)
            .ok_or(error!(ErrorCode::InsufficientFunds))?;
            
        **ctx.accounts.liquidity_pool.try_borrow_mut_lamports()? = ctx
            .accounts.liquidity_pool
            .lamports()
            .checked_add(initial_liquidity_amount)
            .ok_or(error!(ErrorCode::MathOverflow))?;
        
        // Update project state
        project.reserve_balance = project.reserve_balance
            .checked_sub(initial_liquidity_amount)
            .ok_or(error!(ErrorCode::MathOverflow))?;
        
        Ok(())
    }

    pub fn withdraw_platform_fees(
        ctx: Context<WithdrawPlatformFees>,
        amount: u64,
    ) -> Result<()> {
        // Only the launchpad authority can withdraw fees
        if ctx.accounts.authority.key() != ctx.accounts.config.authority {
            return Err(error!(ErrorCode::UnauthorizedAccess));
        }
        
        // Check if the fee vault has enough balance
        if ctx.accounts.fee_vault.lamports() < amount {
            return Err(error!(ErrorCode::InsufficientFunds));
        }
        
        // Transfer SOL from fee vault to recipient
        **ctx.accounts.fee_vault.try_borrow_mut_lamports()? = ctx
            .accounts.fee_vault
            .lamports()
            .checked_sub(amount)
            .ok_or(error!(ErrorCode::InsufficientFunds))?;
            
        **ctx.accounts.recipient.try_borrow_mut_lamports()? = ctx
            .accounts.recipient
            .lamports()
            .checked_add(amount)
            .ok_or(error!(ErrorCode::MathOverflow))?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeLaunchpad<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<LaunchpadConfig>(),
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, LaunchpadConfig>,
    
    #[account(
        init,
        payer = authority,
        space = 8,
        seeds = [b"fee_vault"],
        bump
    )]
    pub fee_vault: SystemAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTokenProject<'info> {
    #[account(mut)]
    pub config: Account<'info, LaunchpadConfig>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<TokenProject>(),
        seeds = [b"project", mint.key().as_ref()],
        bump
    )]
    pub project: Account<'info, TokenProject>,
    
    #[account(mut)]
    pub mint: Signer<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    pub config: Account<'info, LaunchpadConfig>,
    
    #[account(
        mut,
        seeds = [b"project", mint.key().as_ref()],
        bump
    )]
    pub project: Account<'info, TokenProject>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"fee_vault"],
        bump
    )]
    pub fee_vault: SystemAccount<'info>,
    
    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + std::mem::size_of::<BundleTracker>(),
        seeds = [b"bundle", mint.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub bundle_tracker: Option<Account<'info, BundleTracker>>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct SellTokens<'info> {
    pub config: Account<'info, LaunchpadConfig>,
    
    #[account(
        mut,
        seeds = [b"project", mint.key().as_ref()],
        bump
    )]
    pub project: Account<'info, TokenProject>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = seller,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"fee_vault"],
        bump
    )]
    pub fee_vault: SystemAccount<'info>,
    
    #[account(
        init_if_needed,
        payer = seller,
        space = 8 + std::mem::size_of::<BundleTracker>(),
        seeds = [b"bundle", mint.key().as_ref(), seller.key().as_ref()],
        bump
    )]
    pub bundle_tracker: Option<Account<'info, BundleTracker>>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct RegisterWalletRelationship<'info> {
    pub config: Account<'info, LaunchpadConfig>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<WalletRelationship>(),
        seeds = [b"relationship", mint.key().as_ref(), wallet_a.key().as_ref(), wallet_b.key().as_ref()],
        bump
    )]
    pub relationship: Account<'info, WalletRelationship>,
    
    /// CHECK: This is just a pubkey parameter
    pub mint: UncheckedAccount<'info>,
    
    /// CHECK: This is just a pubkey parameter
    pub wallet_a: UncheckedAccount<'info>,
    
    /// CHECK: This is just a pubkey parameter
    pub wallet_b: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateBundleStatus<'info> {
    pub config: Account<'info, LaunchpadConfig>,
    
    #[account(
        seeds = [b"project", mint.key().as_ref()],
        bump
    )]
    pub project: Account<'info, TokenProject>,
    
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + std::mem::size_of::<BundleTracker>(),
        seeds = [b"bundle", mint.key().as_ref(), wallet.key().as_ref()],
        bump
    )]
    pub bundle_tracker: Account<'info, BundleTracker>,
    
    /// CHECK: This is just a pubkey parameter
    pub mint: UncheckedAccount<'info>,
    
    /// CHECK: This is just a pubkey parameter
    pub wallet: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GraduateToken<'info> {
    pub config: Account<'info, LaunchpadConfig>,
    
    #[account(
        mut,
        seeds = [b"project", mint.key().as_ref()],
        bump
    )]
    pub project: Account<'info, TokenProject>,
    
    /// CHECK: This is just a pubkey parameter
    pub mint: UncheckedAccount<'info>,
    
    /// CHECK: This is just a pubkey parameter
    pub liquidity_pool: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateRaydiumPool<'info> {
    pub config: Account<'info, LaunchpadConfig>,
    
    #[account(
        mut,
        seeds = [b"project", mint.key().as_ref()],
        bump
    )]
    pub project: Account<'info, TokenProject>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub liquidity_pool: SystemAccount<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct WithdrawPlatformFees<'info> {
    pub config: Account<'info, LaunchpadConfig>,
    
    #[account(
        mut,
        seeds = [b"fee_vault"],
        bump
    )]
    pub fee_vault: SystemAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LaunchpadConfig {
    pub authority: Pubkey,
    pub fee_recipient: Pubkey,
    pub bundle_threshold_percentage: u16,
    pub graduation_market_cap: u64,
    pub trading_fee_bps: u16, // 100 = 1%
    pub relationship_threshold: u16, // 300 = 3%
}

#[account]
pub struct TokenProject {
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub name: String,
    pub symbol: String,
    pub initial_price: u64,
    pub current_price: u64,
    pub supply: u64,
    pub reserve_balance: u64,
    pub curve_params: Vec<u64>,
    pub is_graduated: bool,
    pub liquidity_pool: Option<Pubkey>,
    pub creator_fee_earned: u64,
    pub platform_fee_earned: u64,
}

#[account]
pub struct BundleTracker {
    pub mint: Pubkey,
    pub wallet: Pubkey,
    pub related_wallets: Vec<Pubkey>,
    pub total_bundle_balance: u64,
    pub is_bundling: bool,
    pub last_updated: i64,
}

#[account]
pub struct WalletRelationship {
    pub mint: Pubkey,
    pub wallet_a: Pubkey,
    pub wallet_b: Pubkey,
    pub relationship_strength: u16,
    pub last_transaction: i64,
    pub transaction_count: u16,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Insufficient supply")]
    InsufficientSupply,
    #[msg("Unauthorized access")]
    UnauthorizedAccess,
    #[msg("Token not eligible for graduation")]
    TokenNotEligibleForGraduation,
    #[msg("Invalid curve parameters")]
    InvalidCurveParams,
    #[msg("Division by zero")]
    DivisionByZero,
    #[msg("Bundling detected")]
    BundlingDetected,
    #[msg("Unsupported instruction")]
    UnsupportedInstruction,
    #[msg("Incorrect transfer hook program")]
    IncorrectTransferHookProgram,
}