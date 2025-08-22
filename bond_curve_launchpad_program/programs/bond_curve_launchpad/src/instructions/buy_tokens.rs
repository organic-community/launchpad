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
	pub struct BuyTokens<'info> {
		#[account(
			mut,
		)]
		pub buyer: Signer<'info>,

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
pub fn handler(
	ctx: Context<BuyTokens>,
	amount: u64,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
