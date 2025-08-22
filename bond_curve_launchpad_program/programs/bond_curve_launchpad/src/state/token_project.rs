
use anchor_lang::prelude::*;

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
	pub curve_type: u8,
	pub curve_params: Vec<u64>,
	pub is_graduated: bool,
	pub liquidity_pool: Option<Pubkey>,
}
