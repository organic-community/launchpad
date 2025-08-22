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
async fn graduate_token_ix_success() {
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

	// DATA
	let mint: Pubkey = Pubkey::default();
	let liquidity_pool: Pubkey = Pubkey::default();

	// KEYPAIR
	let authority_keypair = Keypair::new();

	// PUBKEY
	let authority_pubkey = authority_keypair.pubkey();

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
			mint.as_ref(),
		],
		&bond_curve_launchpad::ID,
	);

	// ACCOUNT PROGRAM TEST SETUP
	program_test.add_account(
		authority_pubkey,
		Account {
			lamports: 1_000_000_000_000,
			data: vec![],
			owner: system_program::ID,
			executable: false,
			rent_epoch: 0,
		},
	);

	// INSTRUCTIONS
	let (mut banks_client, _, recent_blockhash) = program_test.start().await;

	let ix = bond_curve_launchpad_ix_interface::graduate_token_ix_setup(
		&authority_keypair,
		config_pda,
		project_pda,
		mint,
		liquidity_pool,
		recent_blockhash,
	);

	let result = banks_client.process_transaction(ix).await;

	// ASSERTIONS
	assert!(result.is_ok());

}
