// wsol.rs - Comprehensive WSOL handling implementation

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke, invoke_signed},
    system_instruction,
};
use anchor_spl::token::{Token, TokenAccount};

/// Native SOL mint address (WSOL)
pub const WSOL_MINT: &str = "So11111111111111111111111111111111111111112";

/// Wrap SOL to WSOL
/// Creates a temporary WSOL account, transfers SOL to it, and returns the account
pub fn wrap_sol(
    amount: u64,
    payer: &AccountInfo,
    system_program: &AccountInfo,
    token_program: &AccountInfo,
    rent: &Sysvar<Rent>,
) -> Result<Keypair> {
    // Create a new keypair for the WSOL account
    let wsol_account = Keypair::new();
    
    // Create account
    invoke(
        &system_instruction::create_account(
            payer.key,
            &wsol_account.pubkey(),
            rent.minimum_balance(TokenAccount::LEN),
            TokenAccount::LEN as u64,
            &anchor_spl::token::ID,
        ),
        &[payer.clone(), wsol_account.to_account_info(), system_program.clone()],
    )?;

    // Initialize token account
    invoke(
        &spl_token::instruction::initialize_account(
            &spl_token::ID,
            &wsol_account.pubkey(),
            &spl_token::native_mint::id(),
            payer.key,
        )?,
        &[
            wsol_account.to_account_info(),
            payer.clone(),
            token_program.clone(),
            rent.to_account_info().clone(),
        ],
    )?;

    // Transfer SOL to the account
    invoke(
        &system_instruction::transfer(payer.key, &wsol_account.pubkey(), amount),
        &[payer.clone(), wsol_account.to_account_info(), system_program.clone()],
    )?;

    // Sync native account
    invoke(
        &spl_token::instruction::sync_native(
            &spl_token::ID,
            &wsol_account.pubkey(),
        )?,
        &[wsol_account.to_account_info(), token_program.clone()],
    )?;

    Ok(wsol_account)
}

/// Unwrap WSOL back to SOL
/// Closes the WSOL account and returns the SOL to the owner
pub fn unwrap_sol(
    wsol_account: &AccountInfo,
    owner: &AccountInfo,
    token_program: &AccountInfo,
) -> Result<()> {
    // Close the account, which will transfer SOL back to the owner
    invoke(
        &spl_token::instruction::close_account(
            &spl_token::ID,
            wsol_account.key,
            owner.key,
            owner.key,
            &[],
        )?,
        &[
            wsol_account.clone(),
            owner.clone(),
            owner.clone(),
            token_program.clone(),
        ],
    )?;

    Ok(())
}

/// Create a temporary WSOL account for a transaction and return the keypair
pub fn create_temp_wsol_account(
    amount: u64,
    payer: &AccountInfo,
    system_program: &AccountInfo,
    token_program: &AccountInfo,
    rent: &Sysvar<Rent>,
) -> Result<Keypair> {
    wrap_sol(amount, payer, system_program, token_program, rent)
}

/// Close a temporary WSOL account after a transaction
pub fn close_temp_wsol_account(
    wsol_account: &AccountInfo,
    owner: &AccountInfo,
    token_program: &AccountInfo,
) -> Result<()> {
    unwrap_sol(wsol_account, owner, token_program)
}

/// Handle a SOL payment by wrapping to WSOL, performing the transaction, and unwrapping
pub fn handle_sol_payment<F>(
    amount: u64,
    payer: &AccountInfo,
    system_program: &AccountInfo,
    token_program: &AccountInfo,
    rent: &Sysvar<Rent>,
    transaction_handler: F,
) -> Result<()>
where
    F: FnOnce(&AccountInfo) -> Result<()>,
{
    // Create temporary WSOL account
    let wsol_keypair = create_temp_wsol_account(
        amount,
        payer,
        system_program,
        token_program,
        rent,
    )?;
    
    let wsol_account_info = wsol_keypair.to_account_info();
    
    // Perform the transaction
    transaction_handler(&wsol_account_info)?;
    
    // Close the temporary WSOL account
    close_temp_wsol_account(
        &wsol_account_info,
        payer,
        token_program,
    )?;
    
    Ok(())
}

/// Get the WSOL mint pubkey
pub fn get_wsol_mint() -> Pubkey {
    spl_token::native_mint::id()
}