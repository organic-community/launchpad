// bond_curve.rs - Improved exponential curve implementation

use anchor_lang::prelude::*;

/// Calculate the price to buy a specific amount of tokens based on the current supply
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

    // For exponential curve, we need to integrate the price function
    // price = initial_price * (base/10000)^supply
    // We'll use a numerical approximation by summing small increments
    
    let mut total_cost: u64 = 0;
    let increment_size: u64 = 1; // Can be adjusted for precision vs performance
    
    let mut remaining = amount;
    let mut current = current_supply;
    
    while remaining > 0 {
        let increment = if remaining > increment_size { increment_size } else { remaining };
        
        // Calculate price at current supply
        let price_at_current = calculate_price_at_supply(initial_price, base, current)?;
        
        // Add the cost for this increment
        let increment_cost = price_at_current.checked_mul(increment)
            .ok_or(error!(ErrorCode::MathOverflow))?;
            
        total_cost = total_cost.checked_add(increment_cost)
            .ok_or(error!(ErrorCode::MathOverflow))?;
        
        // Update remaining and current
        remaining = remaining.checked_sub(increment)
            .ok_or(error!(ErrorCode::MathOverflow))?;
        current = current.checked_add(increment)
            .ok_or(error!(ErrorCode::MathOverflow))?;
    }
    
    Ok(total_cost)
}

/// Calculate the price to sell a specific amount of tokens based on the current supply
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

    // For selling, we integrate from (current_supply - amount) to current_supply
    // We'll use a numerical approximation by summing small decrements
    
    let mut total_return: u64 = 0;
    let decrement_size: u64 = 1; // Can be adjusted for precision vs performance
    
    let mut remaining = amount;
    let mut current = current_supply;
    
    while remaining > 0 {
        let decrement = if remaining > decrement_size { decrement_size } else { remaining };
        
        // Calculate price at current supply - decrement
        current = current.checked_sub(decrement)
            .ok_or(error!(ErrorCode::MathOverflow))?;
            
        // Calculate price at updated current supply
        let price_at_current = calculate_price_at_supply(initial_price, base, current)?;
        
        // Add the return for this decrement
        let decrement_return = price_at_current.checked_mul(decrement)
            .ok_or(error!(ErrorCode::MathOverflow))?;
            
        total_return = total_return.checked_add(decrement_return)
            .ok_or(error!(ErrorCode::MathOverflow))?;
        
        // Update remaining
        remaining = remaining.checked_sub(decrement)
            .ok_or(error!(ErrorCode::MathOverflow))?;
    }
    
    Ok(total_return)
}

/// Calculate the current price based on the current supply
pub fn calculate_current_price(
    curve_params: &[u64],
    supply: u64,
) -> Result<u64> {
    if curve_params.len() < 2 {
        return Err(error!(ErrorCode::InvalidCurveParams));
    }
    
    let base = curve_params[0];
    let initial_price = curve_params[1];
    
    calculate_price_at_supply(initial_price, base, supply)
}

/// Helper function to calculate price at a specific supply point
pub fn calculate_price_at_supply(initial_price: u64, base: u64, supply: u64) -> Result<u64> {
    // Price = initial_price * (base/10000)^supply
    // We'll use repeated multiplication for simplicity and to avoid floating point
    
    // For base = 10000 (1.0), price is constant
    if base == 10000 {
        return Ok(initial_price);
    }
    
    // For supply = 0, price is initial_price
    if supply == 0 {
        return Ok(initial_price);
    }
    
    // For large supplies, we need to be careful about overflow
    // We'll use a more efficient algorithm for large supplies
    if supply > 100 {
        // Use binary exponentiation for large powers
        let mut result = initial_price;
        let mut base_power = base;
        let mut exp = supply;
        
        while exp > 0 {
            if exp & 1 == 1 {
                result = result.checked_mul(base_power)
                    .ok_or(error!(ErrorCode::MathOverflow))?
                    .checked_div(10000)
                    .ok_or(error!(ErrorCode::MathOverflow))?;
            }
            
            base_power = base_power.checked_mul(base_power)
                .ok_or(error!(ErrorCode::MathOverflow))?
                .checked_div(10000)
                .ok_or(error!(ErrorCode::MathOverflow))?;
                
            exp >>= 1;
        }
        
        return Ok(result);
    }
    
    // For small supplies, direct calculation is fine
    let mut result = initial_price;
    for _ in 0..supply {
        result = result.checked_mul(base)
            .ok_or(error!(ErrorCode::MathOverflow))?
            .checked_div(10000)
            .ok_or(error!(ErrorCode::MathOverflow))?;
    }
    
    Ok(result)
}

/// Calculate the market cap based on current supply and price
pub fn calculate_market_cap(supply: u64, price: u64) -> Result<u64> {
    supply.checked_mul(price)
        .ok_or(error!(ErrorCode::MathOverflow))
}

// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Invalid curve parameters")]
    InvalidCurveParams,
    #[msg("Insufficient supply")]
    InsufficientSupply,
}