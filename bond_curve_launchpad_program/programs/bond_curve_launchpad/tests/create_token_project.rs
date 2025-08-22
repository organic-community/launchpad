pub mod common;

use std::str::FromStr;
use {
    common::{
		get_program_test,
		bond_curve_launchpad_ix_interface,
		csl_spl_token_ix_interface,
	},
    solana_program_test::tokio,
    solana_sdk::{
        account::Account, pubkey::Pubkey, rent::Rent, signature::Keypair, signer::Signer, system_program,
    },
};


#[tokio::test]
async fn create_token_project_ix_success() {
	let mut program_test = get_program_test();

	// PROGRAMS
	program_test.prefer_bpf(true);

	program_test.add_program(
		"account_compression",
		Pubkey::from_str("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK").unwrap(),
		None,
	);

	program_test.add_program(
		"noop",
		Pubkey::from_str("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV").unwrap(),
		None,
	);

	program_test.add_program(
		"csl_spl_token",
		Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
		None,
	);

	// DATA
	let name: String = Default::default();
	let symbol: String = Default::default();
	let initial_price: u64 = Default::default();
	let curve_type: u8 = Default::default();
	let curve_params: Vec<u64> = vec![Default::default()];

	// KEYPAIR
	let creator_keypair = Keypair::new();
	let mint_keypair = Keypair::new();

	// PUBKEY
	let creator_pubkey = creator_keypair.pubkey();
	let mint_pubkey = mint_keypair.pubkey();

	// EXECUTABLE PUBKEY
	let system_program_pubkey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
	let csl_spl_token_v0_0_0_pubkey = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();

	// PDA
	let (config_pda, _config_pda_bump) = Pubkey::find_program_address(
		&[
			b"config",
		],
		&bond_curve_launchpad::ID,
	);

	let (project_pda, _project_pda_bump) = Pubkey::find_program_address(
		&[
			b"project",
			mint_pubkey.as_ref(),
		],
		&bond_curve_launchpad::ID,
	);

	// ACCOUNT PROGRAM TEST SETUP
	program_test.add_account(
		mint_pubkey,
		Account {
			lamports: 0,
			data: vec![],
			owner: system_program::ID,
			executable: false,
			rent_epoch: 0,
		},
	);

	program_test.add_account(
		creator_pubkey,
		Account {
			lamports: 0,
			data: vec![],
			owner: system_program::ID,
			executable: false,
			rent_epoch: 0,
		},
	);

	program_test.add_account(
		mint_pubkey,
		Account {
			lamports: 0,
			data: vec![],
			owner: system_program::ID,
			executable: false,
			rent_epoch: 0,
		},
	);

	// INSTRUCTIONS
	let (mut banks_client, _, recent_blockhash) = program_test.start().await;

	let ix = bond_curve_launchpad_ix_interface::create_token_project_ix_setup(
		&creator_keypair,
		config_pda,
		project_pda,
		&mint_keypair,
		system_program_pubkey,
		csl_spl_token_v0_0_0_pubkey,
		&name,
		&symbol,
		initial_price,
		curve_type,
		curve_params,
		recent_blockhash,
	);

	let result = banks_client.process_transaction(ix).await;

	// ASSERTIONS
	assert!(result.is_ok());

}
