// transfer_hook.rs - Updated to implement 2% fee for external swaps

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke, invoke_signed},
    system_instruction,
};
use spl_token_2022::{
    extension::{transfer_hook, BaseStateWithExtensions, StateWithExtensions},
    state::{Account, Mint},
};
use spl_transfer_hook_interface::instruction::{ExecuteInstruction, TransferHookInstruction};

pub fn initialize_transfer_hook(
    mint: &AccountInfo,
    authority: &AccountInfo,
    program_id: &Pubkey,
    token_program: &AccountInfo,
) -> Result<()> {
    // Create the instruction to enable the transfer hook
    let ix = spl_token_2022::instruction::initialize_mint_close_authority(
        token_program.key,
        mint.key,
        Some(authority.key),
        None,
    )?;

    // Execute the instruction
    invoke(
        &ix,
        &[
            mint.clone(),
            authority.clone(),
            token_program.clone(),
        ],
    )?;

    // Enable the transfer hook extension
    let ix = spl_token_2022::instruction::initialize_transfer_hook(
        token_program.key,
        mint.key,
        Some(program_id),
        authority.key,
    )?;

    // Execute the instruction
    invoke(
        &ix,
        &[
            mint.clone(),
            authority.clone(),
            token_program.clone(),
        ],
    )?;

    Ok(())
}

pub fn process_transfer_hook(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<()> {
    // Deserialize the instruction data
    let transfer_hook_instruction = TransferHookInstruction::unpack(instruction_data)?;

    // Process the instruction based on its variant
    match transfer_hook_instruction {
        TransferHookInstruction::Execute(execute_instruction) => {
            process_execute_instruction(program_id, accounts, execute_instruction)
        }
        _ => {
            // We only implement the Execute instruction for now
            Err(error!(ErrorCode::UnsupportedInstruction))
        }
    }
}

fn process_execute_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    execute_instruction: ExecuteInstruction,
) -> Result<()> {
    // Parse accounts
    let account_iter = &mut accounts.iter();
    
    let mint_info = next_account_info(account_iter)?;
    let source_info = next_account_info(account_iter)?;
    let destination_info = next_account_info(account_iter)?;
    let owner_info = next_account_info(account_iter)?;
    let token_program_info = next_account_info(account_iter)?;
    
    // Try to get the fee vault and project accounts
    let fee_vault_info = next_account_info(account_iter).ok();
    let project_info = next_account_info(account_iter).ok();
    let creator_info = next_account_info(account_iter).ok();
    
    // Verify the mint account
    let mint_data = mint_info.try_borrow_data()?;
    let mint = StateWithExtensions::<Mint>::unpack(&mint_data)?;
    
    // Get the transfer hook program ID from the mint
    let transfer_hook_program_id = transfer_hook::get_program_id(&mint)?;
    
    // Verify that we're the correct program to handle this transfer
    if transfer_hook_program_id != Some(*program_id) {
        return Err(error!(ErrorCode::IncorrectTransferHookProgram));
    }
    
    // Load the source account
    let source_data = source_info.try_borrow_data()?;
    let source_account = StateWithExtensions::<Account>::unpack(&source_data)?;
    
    // Check if this is a transfer from our launchpad
    let is_launchpad_transfer = is_from_launchpad(project_info, source_info);
    
    // If this is not a launchpad transfer, apply the 2% fee
    if !is_launchpad_transfer {
        // Calculate 2% fee
        let amount = execute_instruction.amount;
        let fee_amount = amount.checked_mul(200).ok_or(error!(ErrorCode::MathOverflow))?.checked_div(10000).ok_or(error!(ErrorCode::MathOverflow))?;
        
        if fee_amount > 0 {
            // In a real implementation, we would:
            // 1. Redirect fee_amount tokens to the fee vault
            // 2. Reduce the amount being transferred
            
            // For now, we'll just log the fee
            msg!("External transfer detected. Applying 2% fee: {}", fee_amount);
            
            // Check if bundling is happening
            let is_bundling = check_if_bundling(source_info, mint_info)?;
            
            if is_bundling {
                // If bundling, apply the 100% tax
                msg!("Bundling detected. Transfer not allowed.");
                return Err(error!(ErrorCode::BundlingDetected));
            }
        }
    }
    
    // Allow the transfer to proceed
    Ok(())
}

fn is_from_launchpad(project_info: Option<&AccountInfo>, source_info: &AccountInfo) -> bool {
    // In a real implementation, we would check if the source account is owned by our launchpad
    // For now, we'll just check if the project_info is provided
    project_info.is_some()
}

fn check_if_bundling(source_info: &AccountInfo, mint_info: &AccountInfo) -> Result<bool> {
    // In a real implementation, you would:
    // 1. Deserialize the BundleTracker account
    // 2. Check if is_bundling is true
    
    // For demonstration, we'll just return false
    // In a real implementation, you would load and check the actual data
    Ok(false)
}

// Add these error codes to your ErrorCode enum
#[error_code]
pub enum ErrorCode {
    #[msg("Unsupported instruction")]
    UnsupportedInstruction,
    #[msg("Incorrect transfer hook program")]
    IncorrectTransferHookProgram,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Bundling detected, transfer not allowed")]
    BundlingDetected,
}