
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use std::str::FromStr;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GHhmbFfKT8Ht54Ym4nnwSx1vHz6kGD7Pocmrt7k7Nrnd");

#[program]
pub mod bond_curve_launchpad {
    use super::*;

/// Initialize the launchpad configuration
///
/// Accounts:
/// 0. `[signer]` authority: [AccountInfo] 
/// 1. `[writable]` config: [LaunchpadConfig] 
/// 2. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - fee_recipient: [Pubkey] 
/// - fee_percentage: [u16] 
/// - bundle_threshold_percentage: [u16] 
/// - graduation_market_cap: [u64] 
	pub fn initialize_launchpad(ctx: Context<InitializeLaunchpad>, fee_recipient: Pubkey, fee_percentage: u16, bundle_threshold_percentage: u16, graduation_market_cap: u64) -> Result<()> {
		initialize_launchpad::handler(ctx, fee_recipient, fee_percentage, bundle_threshold_percentage, graduation_market_cap)
	}

/// Create a new token project on the launchpad
///
/// Accounts:
/// 0. `[signer]` creator: [AccountInfo] 
/// 1. `[]` config: [LaunchpadConfig] 
/// 2. `[writable]` project: [TokenProject] 
/// 3. `[writable, signer]` mint: [Mint] 
/// 4. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
/// 5. `[]` csl_spl_token_v0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
///
/// Data:
/// - name: [String] 
/// - symbol: [String] 
/// - initial_price: [u64] 
/// - curve_type: [u8] 
/// - curve_params: [Vec<u64>] 
	pub fn create_token_project(ctx: Context<CreateTokenProject>, name: String, symbol: String, initial_price: u64, curve_type: u8, curve_params: Vec<u64>) -> Result<()> {
		create_token_project::handler(ctx, name, symbol, initial_price, curve_type, curve_params)
	}

/// Buy tokens from the launchpad
///
/// Accounts:
/// 0. `[writable, signer]` buyer: [AccountInfo] 
/// 1. `[]` config: [LaunchpadConfig] 
/// 2. `[writable]` project: [TokenProject] 
/// 3. `[writable]` mint: [Mint] 
///
/// Data:
/// - amount: [u64] Amount of tokens to buy
	pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
		buy_tokens::handler(ctx, amount)
	}

/// Sell tokens back to the launchpad
///
/// Accounts:
/// 0. `[writable, signer]` seller: [AccountInfo] 
/// 1. `[]` config: [LaunchpadConfig] 
/// 2. `[writable]` project: [TokenProject] 
/// 3. `[writable]` mint: [Mint] 
///
/// Data:
/// - amount: [u64] Amount of tokens to sell
	pub fn sell_tokens(ctx: Context<SellTokens>, amount: u64) -> Result<()> {
		sell_tokens::handler(ctx, amount)
	}

/// Register a relationship between two wallets
///
/// Accounts:
/// 0. `[signer]` authority: [AccountInfo] 
/// 1. `[]` config: [LaunchpadConfig] 
/// 2. `[writable]` relationship: [WalletRelationship] 
/// 3. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - mint: [Pubkey] 
/// - wallet_a: [Pubkey] 
/// - wallet_b: [Pubkey] 
/// - relationship_strength: [u16] 
	pub fn register_wallet_relationship(ctx: Context<RegisterWalletRelationship>, mint: Pubkey, wallet_a: Pubkey, wallet_b: Pubkey, relationship_strength: u16) -> Result<()> {
		register_wallet_relationship::handler(ctx, mint, wallet_a, wallet_b, relationship_strength)
	}

/// Update the bundling status of a wallet
///
/// Accounts:
/// 0. `[signer]` authority: [AccountInfo] 
/// 1. `[]` config: [LaunchpadConfig] 
/// 2. `[writable]` bundle_tracker: [BundleTracker] 
/// 3. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - mint: [Pubkey] 
/// - wallet: [Pubkey] 
/// - related_wallets: [Vec<Pubkey>] 
/// - total_bundle_balance: [u64] 
	pub fn update_bundle_status(ctx: Context<UpdateBundleStatus>, mint: Pubkey, wallet: Pubkey, related_wallets: Vec<Pubkey>, total_bundle_balance: u64) -> Result<()> {
		update_bundle_status::handler(ctx, mint, wallet, related_wallets, total_bundle_balance)
	}

/// Graduate a token from the launchpad to a liquidity pool
///
/// Accounts:
/// 0. `[signer]` authority: [AccountInfo] 
/// 1. `[]` config: [LaunchpadConfig] 
/// 2. `[writable]` project: [TokenProject] 
///
/// Data:
/// - mint: [Pubkey] 
/// - liquidity_pool: [Pubkey] 
	pub fn graduate_token(ctx: Context<GraduateToken>, mint: Pubkey, liquidity_pool: Pubkey) -> Result<()> {
		graduate_token::handler(ctx, mint, liquidity_pool)
	}



}
