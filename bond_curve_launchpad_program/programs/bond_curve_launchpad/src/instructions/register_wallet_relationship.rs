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
		wallet_a: Pubkey,
		wallet_b: Pubkey,
		relationship_strength: u16,
	)]
	pub struct RegisterWalletRelationship<'info> {
		pub authority: Signer<'info>,

		#[account(
			seeds = [
				b"config",
			],
			bump,
		)]
		pub config: Account<'info, LaunchpadConfig>,

		#[account(
			init,
			space=114,
			payer=authority,
			seeds = [
				b"relationship",
				mint.as_ref(),
				wallet_a.as_ref(),
				wallet_b.as_ref(),
			],
			bump,
		)]
		pub relationship: Account<'info, WalletRelationship>,

		pub system_program: Program<'info, System>,
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
pub fn handler(
	ctx: Context<RegisterWalletRelationship>,
	mint: Pubkey,
	wallet_a: Pubkey,
	wallet_b: Pubkey,
	relationship_strength: u16,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
