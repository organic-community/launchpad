use anchor_lang::prelude::*;

/// Calculate the percentage of total supply held by a bundle (in basis points)
pub fn calculate_bundle_percentage(bundle_balance: u64, total_supply: u64) -> Result<u16> {
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

/// Check if a wallet is bundling based on the bundle tracker data
pub fn is_bundling(
    bundle_tracker: &Account<BundleTracker>,
    threshold_percentage: u16,
    total_supply: u64
) -> Result<bool> {
    // If the bundle tracker already indicates bundling, return that
    if bundle_tracker.is_bundling {
        return Ok(true);
    }
    
    // Calculate the percentage of total supply held by this bundle
    if total_supply == 0 {
        return Ok(false); // Can't be bundling if there's no supply
    }
    
    let bundle_percentage = calculate_bundle_percentage(
        bundle_tracker.total_bundle_balance,
        total_supply
    )?;
    
    // Check if the bundle percentage exceeds the threshold
    Ok(bundle_percentage > threshold_percentage)
}

/// Update a bundle tracker with new data
pub fn update_bundle_tracker(
    bundle_tracker: &mut Account<BundleTracker>,
    wallet: &Pubkey,
    mint: &Pubkey,
    related_wallets: Vec<Pubkey>,
    total_bundle_balance: u64,
    threshold_percentage: u16,
    total_supply: u64
) -> Result<()> {
    // Update the basic information
    bundle_tracker.wallet = *wallet;
    bundle_tracker.mint = *mint;
    bundle_tracker.related_wallets = related_wallets;
    bundle_tracker.total_bundle_balance = total_bundle_balance;
    bundle_tracker.last_updated = Clock::get()?.unix_timestamp;
    
    // Calculate if this bundle exceeds the threshold
    let bundle_percentage = calculate_bundle_percentage(
        total_bundle_balance,
        total_supply
    )?;
    
    // Update the bundling status
    bundle_tracker.is_bundling = bundle_percentage > threshold_percentage;
    
    Ok(())
}

/// Check if two wallets are related based on their relationship strength
pub fn are_wallets_related(
    relationship: &Account<WalletRelationship>,
    relationship_threshold: u16
) -> Result<bool> {
    // Check if the relationship strength exceeds the threshold
    Ok(relationship.relationship_strength >= relationship_threshold)
}