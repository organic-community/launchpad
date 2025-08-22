// bond_curve.rs - Simplified to only use exponential curve

use anchor_lang::prelude::*;

pub fn calculate_buy_price(
    curve_params: &[u64],
    current_supply: u64,
    amount: u64,
) -> Result<u64> {
    if curve_params.len() < 2 {
        return Err(error!(ErrorCode::InvalidCurveParams));
    }

    let base = curve_params[0];
    let initial_price = curve_params[1];

    // For exponential curve: price = initial_price * base^(current_supply)
    // This is a simplification as true integration would be complex
    // We'll use average price over the range as an approximation
    
    let start_price = calculate_exponential_price_at_supply(initial_price, base, current_supply)?;
    let end_price = calculate_exponential_price_at_supply(initial_price, base, current_supply.checked_add(amount).ok_or(error!(ErrorCode::MathOverflow))?)?;
    
    let avg_price = start_price.checked_add(end_price).ok_or(error!(ErrorCode::MathOverflow))?.checked_div(2).ok_or(error!(ErrorCode::MathOverflow))?;
    let total_cost = avg_price.checked_mul(amount).ok_or(error!(ErrorCode::MathOverflow))?;
    
    Ok(total_cost)
}

pub fn calculate_sell_price(
    curve_params: &[u64],
    current_supply: u64,
    amount: u64,
) -> Result<u64> {
    if curve_params.len() < 2 {
        return Err(error!(ErrorCode::InvalidCurveParams));
    }

    if current_supply < amount {
        return Err(error!(ErrorCode::InsufficientSupply));
    }

    let base = curve_params[0];
    let initial_price = curve_params[1];

    // For selling with exponential curve, we use the same average price approximation
    let new_supply = current_supply.checked_sub(amount).ok_or(error!(ErrorCode::MathOverflow))?;
    let start_price = calculate_exponential_price_at_supply(initial_price, base, new_supply)?;
    let end_price = calculate_exponential_price_at_supply(initial_price, base, current_supply)?;
    
    let avg_price = start_price.checked_add(end_price).ok_or(error!(ErrorCode::MathOverflow))?.checked_div(2).ok_or(error!(ErrorCode::MathOverflow))?;
    let total_return = avg_price.checked_mul(amount).ok_or(error!(ErrorCode::MathOverflow))?;
    
    Ok(total_return)
}

pub fn calculate_current_price(
    curve_params: &[u64],
    supply: u64,
) -> Result<u64> {
    if curve_params.len() < 2 {
        return Err(error!(ErrorCode::InvalidCurveParams));
    }
    
    let base = curve_params[0];
    let initial_price = curve_params[1];
    
    calculate_exponential_price_at_supply(initial_price, base, supply)
}

fn calculate_exponential_price_at_supply(initial_price: u64, base: u64, supply: u64) -> Result<u64> {
    // This is a simplified calculation for base^supply that works for small exponents
    // For a production system, you'd want a more sophisticated approach
    let mut result = initial_price;
    for _ in 0..supply {
        result = result.checked_mul(base).ok_or(error!(ErrorCode::MathOverflow))?.checked_div(10000).ok_or(error!(ErrorCode::MathOverflow))?;
    }
    Ok(result)
}

// Add these error codes to your ErrorCode enum if not already present
#[error_code]
pub enum ErrorCode {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Invalid curve parameters")]
    InvalidCurveParams,
    #[msg("Insufficient supply")]
    InsufficientSupply,
}