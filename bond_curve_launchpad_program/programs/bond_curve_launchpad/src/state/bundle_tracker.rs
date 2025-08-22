
use anchor_lang::prelude::*;

#[account]
pub struct BundleTracker {
	pub mint: Pubkey,
	pub wallet: Pubkey,
	pub related_wallets: Vec<Pubkey>,
	pub total_bundle_balance: u64,
	pub is_bundling: bool,
}
