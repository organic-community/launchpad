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

use crate::bundle_detection;
use crate::BundleTracker;

/// Initialize the transfer hook extension on a token mint
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

/// Process the transfer hook instruction
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

/// Process the Execute instruction for the transfer hook
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
    
    // Try to get the optional accounts
    let project_info = next_account_info(account_iter).ok();
    let fee_vault_info = next_account_info(account_iter).ok();
    let bundle_tracker_info = next_account_info(account_iter).ok();
    let config_info = next_account_info(account_iter).ok();
    
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
    
    // Get the transfer amount
    let amount = execute_instruction.amount;
    
    // Check for bundling if we have the necessary accounts
    if let (Some(bundle_tracker), Some(config)) = (bundle_tracker_info, config_info) {
        // Try to deserialize the bundle tracker account
        if bundle_tracker.owner == program_id {
            let bundle_tracker_data = bundle_tracker.try_borrow_data()?;
            if !bundle_tracker_data.is_empty() {
                // Deserialize the bundle tracker account
                let bundle_tracker_account = Account::<BundleTracker>::try_from(bundle_tracker)?;
                
                // Check if the source wallet is part of a bundle
                let is_bundling = bundle_tracker_account.is_bundling;
                
                if is_bundling {
                    // If bundling is detected, apply 100% tax by preventing the transfer
                    msg!("Bundling detected. Applying 100% tax.");
                    
                    // In a real implementation, we would:
                    // 1. Cancel the current transfer
                    // 2. Initiate a new transfer to burn the tokens or send to a fee account
                    
                    // For now, we'll just prevent the transfer
                    return Err(error!(ErrorCode::BundlingDetected));
                }
            }
        }
    }
    
    // If this is not a launchpad transfer, apply the 2% fee
    if !is_launchpad_transfer && fee_vault_info.is_some() {
        // Calculate 2% fee
        let fee_amount = amount
            .checked_mul(200)
            .ok_or(error!(ErrorCode::MathOverflow))?
            .checked_div(10000)
            .ok_or(error!(ErrorCode::DivisionByZero))?;
        
        if fee_amount > 0 {
            // Log the fee
            msg!("External transfer detected. Applying 2% fee: {}", fee_amount);
            
            // In a real implementation, we would:
            // 1. Modify the transfer amount to subtract the fee
            // 2. Initiate a separate transfer for the fee to the fee vault
            
            // This would require additional CPI calls to the token program
            // For now, we'll just log the fee
        }
    }
    
    // Allow the transfer to proceed
    Ok(())
}

/// Check if a transfer is originating from our launchpad
fn is_from_launchpad(project_info: Option<&AccountInfo>, source_info: &AccountInfo) -> bool {
    if let Some(project) = project_info {
        // Check if the project account is owned by our program
        if project.owner == &crate::ID {
            // In a real implementation, you would check if the source account
            // is owned by our launchpad or is part of a launchpad transaction
            return true;
        }
    }
    
    false
}