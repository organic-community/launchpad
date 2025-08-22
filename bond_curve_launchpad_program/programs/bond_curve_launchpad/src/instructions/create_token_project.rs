use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};




	#[derive(Accounts)]
	#[instruction(
		name: String,
		symbol: String,
		initial_price: u64,
		curve_type: u8,
		curve_params: Vec<u64>,
	)]
	pub struct CreateTokenProject<'info> {
		pub creator: Signer<'info>,

		#[account(
			seeds = [
				b"config",
			],
			bump,
		)]
		pub config: Account<'info, LaunchpadConfig>,

		#[account(
			init,
			space=243,
			payer=creator,
			seeds = [
				b"project",
				mint.key().as_ref(),
			],
			bump,
		)]
		pub project: Account<'info, TokenProject>,

		#[account(
			init,
			payer = creator,
			mint::decimals = 0,
		)]
		pub mint: Account<'info, Mint>,

		pub system_program: Program<'info, System>,

		pub csl_spl_token_v0_0_0: Program<'info, Token>,
	}

	impl<'info> CreateTokenProject<'info> {
		pub fn cpi_csl_spl_token_initialize_mint2(&self, decimals: u8, mint_authority: Pubkey, freeze_authority: Option<Pubkey>) -> Result<()> {
			anchor_spl::token::initialize_mint2(
				CpiContext::new(self.csl_spl_token_v0_0_0.to_account_info(), 
					anchor_spl::token::InitializeMint2 {
						mint: self.mint.to_account_info()
					}
				),
				decimals, 
				mint_authority, 
				freeze_authority, 
			)
		}
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
pub fn handler(
	ctx: Context<CreateTokenProject>,
	name: String,
	symbol: String,
	initial_price: u64,
	curve_type: u8,
	curve_params: Vec<u64>,
) -> Result<()> {
    // Implement your business logic here...
	
	// Cpi calls wrappers
	ctx.accounts.cpi_csl_spl_token_initialize_mint2(
		Default::default(),
		Pubkey::default(),
		None,
	)?;

	Ok(())
}
