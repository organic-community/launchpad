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
async fn buy_tokens_ix_success() {
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
	let amount: u64 = Default::default();

	// KEYPAIR
	let buyer_keypair = Keypair::new();
	let mint_keypair = Keypair::new();

	// PUBKEY
	let buyer_pubkey = buyer_keypair.pubkey();
	let mint_pubkey = mint_keypair.pubkey();

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
		buyer_pubkey,
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

	let ix = bond_curve_launchpad_ix_interface::buy_tokens_ix_setup(
		&buyer_keypair,
		config_pda,
		project_pda,
		mint_pubkey,
		amount,
		recent_blockhash,
	);

	let result = banks_client.process_transaction(ix).await;

	// ASSERTIONS
	assert!(result.is_ok());

}
