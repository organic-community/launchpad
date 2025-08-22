# Bond Curve Launchpad with Anti-Bundling Mechanism

A Solana-based token launchpad with exponential bond curve pricing and an anti-bundling mechanism using Token-2022 transfer hooks.

## Features

- **SOL-based Launchpad**: Users can buy and sell tokens using native SOL (with automatic WSOL wrapping/unwrapping)
- **Exponential Bond Curve Pricing**: Configurable exponential curve for predictable token pricing
- **Token-2022 Integration**: Uses transfer hooks to implement anti-bundling mechanisms and external swap fees
- **Fee Structure**:
  - **1% Trading Fee** on the launchpad (0.5% to creator, 0.5% to platform)
  - **2% External Swap Fee** for trades outside the launchpad
  - **Anti-Bundling Mechanism**: 100% tax on transfers from wallets collectively holding >5% of a token
- **Graduation Process**: Tokens graduate to Raydium liquidity pools at $100k market cap

## Project Structure

- `/programs/bond_curve_launchpad`: Solana program (smart contract) written in Rust
- `/app`: Frontend application written in React/TypeScript

## Smart Contract Components

1. **LaunchpadConfig**: Global configuration for the launchpad
2. **TokenProject**: Information about each token project
3. **BondCurve**: Parameters for the exponential bond curve
4. **BundleTracker**: Track wallet relationships and bundling status
5. **TransferHook**: Implement the Token-2022 transfer hook for fees and anti-bundling
6. **FeeVault**: PDA to collect platform fees

## How It Works

### Bond Curve Mechanism

The launchpad uses an exponential bond curve to determine token prices. As more tokens are bought, the price increases according to the formula:

`price = initial_price * base^(current_supply)`

Where:
- `initial_price` is the starting price of the token
- `base` is the growth factor (in basis points, 10000 = 1.0)
- `current_supply` is the current token supply

### Fee Structure

The launchpad implements a dual fee structure:

1. **Launchpad Trading Fees (1%)**:
   - 0.5% goes to the token creator
   - 0.5% goes to the platform fee vault

2. **External Swap Fees (2%)**:
   - Applied to all transfers outside the launchpad
   - Implemented via Token-2022 transfer hook
   - Encourages trading on the launchpad

### Anti-Bundling Mechanism

The anti-bundling mechanism works as follows:

1. The launchpad tracks relationships between wallets (e.g., wallets that have sent funds to each other)
2. When wallets are identified as part of a "bundle" (related wallets collectively holding >5% of a token)
3. The Token-2022 transfer hook applies a 100% tax on transfers from these wallets
4. The tax is lifted once the bundle's holdings drop below the 5% threshold

### Graduation Process

Tokens graduate from the launchpad to Raydium liquidity pools when they reach a market cap of $100k. The graduation process:

1. Creates a liquidity pool on Raydium
2. Transfers tokens and SOL to the pool
3. Initializes the pool

## Setup and Installation

### Prerequisites

- Rust and Cargo
- Solana CLI
- Node.js and npm/yarn
- Anchor Framework

### Building the Program

```bash
# Clone the repository
git clone https://github.com/yourusername/bond-curve-launchpad.git
cd bond-curve-launchpad

# Build the program
anchor build

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

### Running the Frontend

```bash
# Navigate to the app directory
cd app

# Install dependencies
npm install

# Start the development server
npm start
```

## Usage

### Creating a Token

1. Connect your wallet
2. Fill in the token details:
   - Name
   - Symbol
   - Initial Price
   - Curve Parameters (base, initial_price)
3. Click "Create Token"

### Buying Tokens

1. Select a token from the dropdown
2. Enter the amount to buy
3. Click "Buy Tokens"

### Selling Tokens

1. Select a token from the dropdown
2. Enter the amount to sell
3. Click "Sell Tokens"

### Admin Functions

1. Withdraw platform fees to a specified recipient

## Technical Details

### Exponential Bond Curve Calculation

For buying:
```
Total cost = average price * amount
where average price = (price at current supply + price at current supply + amount) / 2
```

For selling:
```
Total return = average price * amount
where average price = (price at current supply - amount + price at current supply) / 2
```

### Token-2022 Transfer Hook

The transfer hook intercepts token transfers and:

1. Checks if the transfer is from the launchpad
2. If not, applies a 2% fee
3. Checks if the sender is part of a bundle exceeding the threshold
4. If yes, applies a 100% tax (redirects tokens to buy and burn)
5. If no, allows the transfer to proceed normally

## Security Considerations

- **Reentrancy Protection**: The program uses Anchor's reentrancy protection
- **Overflow/Underflow Protection**: All mathematical operations use checked arithmetic
- **Access Control**: Only authorized accounts can perform sensitive operations
- **Signer Verification**: All transactions require appropriate signers

## License

This project is licensed under the MIT License - see the LICENSE file for details.