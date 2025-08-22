use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};




	#[derive(Accounts)]
	#[instruction(
		mint: Pubkey,
		liquidity_pool: Pubkey,
	)]
	pub struct GraduateToken<'info> {
		pub authority: Signer<'info>,

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
				mint.as_ref(),
			],
			bump,
		)]
		pub project: Account<'info, TokenProject>,
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
pub fn handler(
	ctx: Context<GraduateToken>,
	mint: Pubkey,
	liquidity_pool: Pubkey,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
