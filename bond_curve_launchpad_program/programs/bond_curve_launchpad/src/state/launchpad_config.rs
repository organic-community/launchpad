
use anchor_lang::prelude::*;

#[account]
pub struct LaunchpadConfig {
	pub authority: Pubkey,
	pub fee_recipient: Pubkey,
	pub fee_percentage: u16,
	pub bundle_threshold_percentage: u16,
	pub graduation_market_cap: u64,
}
