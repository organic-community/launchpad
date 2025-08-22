
use anchor_lang::prelude::*;

#[account]
pub struct WalletRelationship {
	pub mint: Pubkey,
	pub wallet_a: Pubkey,
	pub wallet_b: Pubkey,
	pub relationship_strength: u16,
	pub last_transaction: i64,
}
