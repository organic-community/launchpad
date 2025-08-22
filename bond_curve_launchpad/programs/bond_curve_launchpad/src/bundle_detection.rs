// bundle_detection.rs - Implementation of the anti-bundling detection mechanism

use anchor_lang::prelude::*;

/// Check if a wallet is part of a bundle that exceeds the threshold
pub fn is_wallet_bundling(
    wallet: &Pubkey,
    mint: &Pubkey,
    bundle_tracker: &Option<Account<BundleTracker>>,
    threshold_percentage: u16,
    total_supply: u64,
) -> Result<bool> {
    // If no bundle tracker is provided, we can't determine bundling
    if bundle_tracker.is_none() {
        return Ok(false);
    }

    let tracker = bundle_tracker.as_ref().unwrap();
    
    // If the bundle tracker is for a different wallet or mint, it's not relevant
    if tracker.wallet != *wallet || tracker.mint != *mint {
        return Ok(false);
    }
    
    // If the bundle tracker already indicates bundling, return that
    if tracker.is_bundling {
        return Ok(true);
    }
    
    // Calculate the percentage of total supply held by this bundle
    if total_supply == 0 {
        return Ok(false); // Can't be bundling if there's no supply
    }
    
    let bundle_percentage = calculate_bundle_percentage(tracker.total_bundle_balance, total_supply)?;
    
    // Check if the bundle percentage exceeds the threshold
    Ok(bundle_percentage > threshold_percentage)
}

/// Calculate the percentage of total supply held by a bundle (in basis points)
fn calculate_bundle_percentage(bundle_balance: u64, total_supply: u64) -> Result<u16> {
    if total_supply == 0 {
        return Err(error!(ErrorCode::DivisionByZero));
    }
    
    // Calculate percentage in basis points (1% = 100 basis points)
    let percentage = (bundle_balance as u128)
        .checked_mul(10000)
        .ok_or(error!(ErrorCode::MathOverflow))?
        .checked_div(total_supply as u128)
        .ok_or(error!(ErrorCode::DivisionByZero))?;
    
    // Ensure the percentage fits in a u16
    if percentage > u16::MAX as u128 {
        return Err(error!(ErrorCode::MathOverflow));
    }
    
    Ok(percentage as u16)
}

/// Update the bundle status for a wallet
pub fn update_bundle_status(
    bundle_tracker: &mut Account<BundleTracker>,
    related_wallets: Vec<Pubkey>,
    total_bundle_balance: u64,
    threshold_percentage: u16,
    total_supply: u64,
) -> Result<()> {
    // Update the bundle tracker with the new information
    bundle_tracker.related_wallets = related_wallets;
    bundle_tracker.total_bundle_balance = total_bundle_balance;
    
    // Calculate if this bundle exceeds the threshold
    let bundle_percentage = calculate_bundle_percentage(total_bundle_balance, total_supply)?;
    bundle_tracker.is_bundling = bundle_percentage > threshold_percentage;
    
    Ok(())
}

/// Check if two wallets are related based on their relationship strength
pub fn are_wallets_related(
    wallet_relationship: &Option<Account<WalletRelationship>>,
    relationship_threshold: u16,
) -> Result<bool> {
    // If no relationship data is provided, we assume they're not related
    if wallet_relationship.is_none() {
        return Ok(false);
    }
    
    let relationship = wallet_relationship.as_ref().unwrap();
    
    // Check if the relationship strength exceeds the threshold
    Ok(relationship.relationship_strength >= relationship_threshold)
}

#[account]
pub struct BundleTracker {
    pub mint: Pubkey,
    pub wallet: Pubkey,
    pub related_wallets: Vec<Pubkey>,
    pub total_bundle_balance: u64,
    pub is_bundling: bool,
}

#[account]
pub struct WalletRelationship {
    pub mint: Pubkey,
    pub wallet_a: Pubkey,
    pub wallet_b: Pubkey,
    pub relationship_strength: u16,
    pub last_transaction: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Division by zero")]
    DivisionByZero,
}