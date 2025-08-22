use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};




	#[derive(Accounts)]
	#[instruction(
		amount: u64,
	)]
	pub struct SellTokens<'info> {
		#[account(
			mut,
		)]
		pub seller: Signer<'info>,

		#[account(
			seeds = [
				b"config",
			],
			bump,
		)]
		pub config: Account<'info, LaunchpadConfig>,

		#[account(
			mut,
			seeds = [
				b"project",
				mint.key().as_ref(),
			],
			bump,
		)]
		pub project: Account<'info, TokenProject>,

		#[account(mut)]
		pub mint: Account<'info, Mint>,
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
pub fn handler(
	ctx: Context<SellTokens>,
	amount: u64,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
