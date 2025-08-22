// wsol.rs - Helper module for WSOL wrapping/unwrapping

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke, invoke_signed},
    system_instruction,
};
use anchor_spl::token::{Token, TokenAccount};

pub fn wrap_sol(
    amount: u64,
    wsol_account: &AccountInfo,
    owner: &AccountInfo,
    system_program: &AccountInfo,
    token_program: &AccountInfo,
    rent: &Sysvar<Rent>,
) -> Result<()> {
    // Create account
    invoke(
        &system_instruction::create_account(
            owner.key,
            wsol_account.key,
            rent.minimum_balance(TokenAccount::LEN),
            TokenAccount::LEN as u64,
            &anchor_spl::token::ID,
        ),
        &[owner.clone(), wsol_account.clone(), system_program.clone()],
    )?;

    // Initialize token account
    invoke(
        &spl_token::instruction::initialize_account(
            &spl_token::ID,
            wsol_account.key,
            &spl_token::native_mint::id(),
            owner.key,
        )?,
        &[
            wsol_account.clone(),
            owner.clone(),
            token_program.clone(),
            rent.to_account_info().clone(),
        ],
    )?;

    // Transfer SOL to the account
    invoke(
        &system_instruction::transfer(owner.key, wsol_account.key, amount),
        &[owner.clone(), wsol_account.clone(), system_program.clone()],
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

pub fn wrap_sol_with_pda(
    amount: u64,
    wsol_account: &AccountInfo,
    pda: &AccountInfo,
    owner: &AccountInfo,
    system_program: &AccountInfo,
    token_program: &AccountInfo,
    rent: &Sysvar<Rent>,
    seeds: &[&[u8]],
) -> Result<()> {
    // Create account
    invoke_signed(
        &system_instruction::create_account(
            owner.key,
            wsol_account.key,
            rent.minimum_balance(TokenAccount::LEN),
            TokenAccount::LEN as u64,
            &anchor_spl::token::ID,
        ),
        &[owner.clone(), wsol_account.clone(), system_program.clone()],
        &[seeds],
    )?;

    // Initialize token account
    invoke(
        &spl_token::instruction::initialize_account(
            &spl_token::ID,
            wsol_account.key,
            &spl_token::native_mint::id(),
            pda.key,
        )?,
        &[
            wsol_account.clone(),
            pda.clone(),
            token_program.clone(),
            rent.to_account_info().clone(),
        ],
    )?;

    // Transfer SOL to the account
    invoke(
        &system_instruction::transfer(owner.key, wsol_account.key, amount),
        &[owner.clone(), wsol_account.clone(), system_program.clone()],
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

pub fn unwrap_sol_with_pda(
    wsol_account: &AccountInfo,
    pda: &AccountInfo,
    owner: &AccountInfo,
    token_program: &AccountInfo,
    seeds: &[&[u8]],
) -> Result<()> {
    // Close the account, which will transfer SOL back to the owner
    invoke_signed(
        &spl_token::instruction::close_account(
            &spl_token::ID,
            wsol_account.key,
            owner.key,
            pda.key,
            &[],
        )?,
        &[
            wsol_account.clone(),
            owner.clone(),
            pda.clone(),
            token_program.clone(),
        ],
        &[seeds],
    )?;

    Ok(())
}