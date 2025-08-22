use {
	bond_curve_launchpad::{
			entry,
			ID as PROGRAM_ID,
	},
	solana_sdk::{
		entrypoint::{ProcessInstruction, ProgramResult},
		pubkey::Pubkey,
	},
	anchor_lang::prelude::AccountInfo,
	solana_program_test::*,
};

// Type alias for the entry function pointer used to convert the entry function into a ProcessInstruction function pointer.
pub type ProgramEntry = for<'info> fn(
	program_id: &Pubkey,
	accounts: &'info [AccountInfo<'info>],
	instruction_data: &[u8],
) -> ProgramResult;

// Macro to convert the entry function into a ProcessInstruction function pointer.
#[macro_export]
macro_rules! convert_entry {
	($entry:expr) => {
		// Use unsafe block to perform memory transmutation.
		unsafe { core::mem::transmute::<ProgramEntry, ProcessInstruction>($entry) }
	};
}

pub fn get_program_test() -> ProgramTest {
	let program_test = ProgramTest::new(
		"bond_curve_launchpad",
		PROGRAM_ID,
		processor!(convert_entry!(entry)),
	);
	program_test
}
	
pub mod bond_curve_launchpad_ix_interface {

	use {
		solana_sdk::{
			hash::Hash,
			signature::{Keypair, Signer},
			instruction::Instruction,
			pubkey::Pubkey,
			transaction::Transaction,
		},
		bond_curve_launchpad::{
			ID as PROGRAM_ID,
			accounts as bond_curve_launchpad_accounts,
			instruction as bond_curve_launchpad_instruction,
		},
		anchor_lang::{
			prelude::*,
			InstructionData,
		}
	};

	pub fn initialize_launchpad_ix_setup(
		authority: &Keypair,
		config: Pubkey,
		system_program: Pubkey,
		fee_recipient: Pubkey,
		fee_percentage: u16,
		bundle_threshold_percentage: u16,
		graduation_market_cap: u64,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = bond_curve_launchpad_accounts::InitializeLaunchpad {
			authority: authority.pubkey(),
			config: config,
			system_program: system_program,
		};

		let data = 	bond_curve_launchpad_instruction::InitializeLaunchpad {
				fee_recipient,
				fee_percentage,
				bundle_threshold_percentage,
				graduation_market_cap,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&authority.pubkey()),
		);

		transaction.sign(&[
			&authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn create_token_project_ix_setup(
		creator: &Keypair,
		config: Pubkey,
		project: Pubkey,
		mint: &Keypair,
		system_program: Pubkey,
		csl_spl_token_v0_0_0: Pubkey,
		name: &String,
		symbol: &String,
		initial_price: u64,
		curve_type: u8,
		curve_params: Vec<u64>,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = bond_curve_launchpad_accounts::CreateTokenProject {
			creator: creator.pubkey(),
			config: config,
			project: project,
			mint: mint.pubkey(),
			system_program: system_program,
			csl_spl_token_v0_0_0: csl_spl_token_v0_0_0,
		};

		let data = 	bond_curve_launchpad_instruction::CreateTokenProject {
				name: name.clone(),
				symbol: symbol.clone(),
				initial_price,
				curve_type,
				curve_params,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&creator.pubkey()),
		);

		transaction.sign(&[
			&creator,
			&mint,
		], recent_blockhash);

		return transaction;
	}

	pub fn buy_tokens_ix_setup(
		buyer: &Keypair,
		config: Pubkey,
		project: Pubkey,
		mint: Pubkey,
		amount: u64,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = bond_curve_launchpad_accounts::BuyTokens {
			buyer: buyer.pubkey(),
			config: config,
			project: project,
			mint: mint,
		};

		let data = 	bond_curve_launchpad_instruction::BuyTokens {
				amount,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&buyer.pubkey()),
		);

		transaction.sign(&[
			&buyer,
		], recent_blockhash);

		return transaction;
	}

	pub fn sell_tokens_ix_setup(
		seller: &Keypair,
		config: Pubkey,
		project: Pubkey,
		mint: Pubkey,
		amount: u64,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = bond_curve_launchpad_accounts::SellTokens {
			seller: seller.pubkey(),
			config: config,
			project: project,
			mint: mint,
		};

		let data = 	bond_curve_launchpad_instruction::SellTokens {
				amount,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&seller.pubkey()),
		);

		transaction.sign(&[
			&seller,
		], recent_blockhash);

		return transaction;
	}

	pub fn register_wallet_relationship_ix_setup(
		authority: &Keypair,
		config: Pubkey,
		relationship: Pubkey,
		system_program: Pubkey,
		mint: Pubkey,
		wallet_a: Pubkey,
		wallet_b: Pubkey,
		relationship_strength: u16,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = bond_curve_launchpad_accounts::RegisterWalletRelationship {
			authority: authority.pubkey(),
			config: config,
			relationship: relationship,
			system_program: system_program,
		};

		let data = 	bond_curve_launchpad_instruction::RegisterWalletRelationship {
				mint,
				wallet_a,
				wallet_b,
				relationship_strength,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&authority.pubkey()),
		);

		transaction.sign(&[
			&authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn update_bundle_status_ix_setup(
		authority: &Keypair,
		config: Pubkey,
		bundle_tracker: Pubkey,
		system_program: Pubkey,
		mint: Pubkey,
		wallet: Pubkey,
		related_wallets: Vec<Pubkey>,
		total_bundle_balance: u64,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = bond_curve_launchpad_accounts::UpdateBundleStatus {
			authority: authority.pubkey(),
			config: config,
			bundle_tracker: bundle_tracker,
			system_program: system_program,
		};

		let data = 	bond_curve_launchpad_instruction::UpdateBundleStatus {
				mint,
				wallet,
				related_wallets,
				total_bundle_balance,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&authority.pubkey()),
		);

		transaction.sign(&[
			&authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn graduate_token_ix_setup(
		authority: &Keypair,
		config: Pubkey,
		project: Pubkey,
		mint: Pubkey,
		liquidity_pool: Pubkey,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = bond_curve_launchpad_accounts::GraduateToken {
			authority: authority.pubkey(),
			config: config,
			project: project,
		};

		let data = 	bond_curve_launchpad_instruction::GraduateToken {
				mint,
				liquidity_pool,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&authority.pubkey()),
		);

		transaction.sign(&[
			&authority,
		], recent_blockhash);

		return transaction;
	}

}

pub mod csl_spl_token_ix_interface {

	use {
		solana_sdk::{
			hash::Hash,
			signature::{Keypair, Signer},
			instruction::Instruction,
			pubkey::Pubkey,
			transaction::Transaction,
		},
		csl_spl_token::{
			ID as PROGRAM_ID,
			accounts as csl_spl_token_accounts,
			instruction as csl_spl_token_instruction,
		},
		anchor_lang::{
			prelude::*,
			InstructionData,
		}
	};

	declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

}
