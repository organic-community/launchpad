use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};




	#[derive(Accounts)]
	#[instruction(
		fee_recipient: Pubkey,
		fee_percentage: u16,
		bundle_threshold_percentage: u16,
		graduation_market_cap: u64,
	)]
	pub struct InitializeLaunchpad<'info> {
		pub authority: Signer<'info>,

		#[account(
			init,
			space=84,
			payer=authority,
			seeds = [
				b"config",
			],
			bump,
		)]
		pub config: Account<'info, LaunchpadConfig>,

		pub system_program: Program<'info, System>,
	}

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
pub fn handler(
	ctx: Context<InitializeLaunchpad>,
	fee_recipient: Pubkey,
	fee_percentage: u16,
	bundle_threshold_percentage: u16,
	graduation_market_cap: u64,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
