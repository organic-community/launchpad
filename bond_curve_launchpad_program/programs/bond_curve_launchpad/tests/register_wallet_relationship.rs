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
async fn register_wallet_relationship_ix_success() {
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
	let wallet_a: Pubkey = Pubkey::default();
	let wallet_b: Pubkey = Pubkey::default();
	let relationship_strength: u16 = Default::default();

	// KEYPAIR
	let authority_keypair = Keypair::new();

	// PUBKEY
	let authority_pubkey = authority_keypair.pubkey();

	// EXECUTABLE PUBKEY
	let system_program_pubkey = Pubkey::from_str("11111111111111111111111111111111").unwrap();

	// PDA
	let (config_pda, _config_pda_bump) = Pubkey::find_program_address(
		&[
			b"config",
		],
		&bond_curve_launchpad::ID,
	);

	let (relationship_pda, _relationship_pda_bump) = Pubkey::find_program_address(
		&[
			b"relationship",
			mint.as_ref(),
			wallet_a.as_ref(),
			wallet_b.as_ref(),
		],
		&bond_curve_launchpad::ID,
	);

	// ACCOUNT PROGRAM TEST SETUP
	program_test.add_account(
		authority_pubkey,
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

	let ix = bond_curve_launchpad_ix_interface::register_wallet_relationship_ix_setup(
		&authority_keypair,
		config_pda,
		relationship_pda,
		system_program_pubkey,
		mint,
		wallet_a,
		wallet_b,
		relationship_strength,
		recent_blockhash,
	);

	let result = banks_client.process_transaction(ix).await;

	// ASSERTIONS
	assert!(result.is_ok());

}
