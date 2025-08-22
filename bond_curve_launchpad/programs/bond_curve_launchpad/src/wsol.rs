use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke, invoke_signed},
    system_instruction,
};
use anchor_spl::token::{Token, TokenAccount};

/// Native SOL mint address (WSOL)
pub const WSOL_MINT: &str = "So11111111111111111111111111111111111111112";

/// Wrap SOL to WSOL
pub fn wrap_sol(
    amount: u64,
    payer: &AccountInfo,
    wsol_account: &AccountInfo,
    system_program: &AccountInfo,
    token_program: &AccountInfo,
    rent: &Sysvar<Rent>,
) -> Result<()> {
    // Create account
    invoke(
        &system_instruction::create_account(
            payer.key,
            wsol_account.key,
            rent.minimum_balance(TokenAccount::LEN),
            TokenAccount::LEN as u64,
            &anchor_spl::token::ID,
        ),
        &[payer.clone(), wsol_account.clone(), system_program.clone()],
    )?;

    // Initialize token account
    invoke(
        &spl_token::instruction::initialize_account(
            &spl_token::ID,
            wsol_account.key,
            &spl_token::native_mint::id(),
            payer.key,
        )?,
        &[
            wsol_account.clone(),
            payer.clone(),
            token_program.clone(),
            rent.to_account_info().clone(),
        ],
    )?;

    // Transfer SOL to the account
    invoke(
        &system_instruction::transfer(payer.key, wsol_account.key, amount),
        &[payer.clone(), wsol_account.clone(), system_program.clone()],
    )?;

    // Sync native account
    invoke(
        &spl_token::instruction::sync_native(
            &spl_token::ID,
            wsol_account.key,
        )?,
        &[wsol_account.clone(), token_program.clone()],
    )?;

    Ok(())
}

/// Unwrap WSOL back to SOL
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

/// Get the WSOL mint pubkey
pub fn get_wsol_mint() -> Pubkey {
    spl_token::native_mint::id()
}