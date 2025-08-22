# Bond Curve Launchpad

A Solana-based token launchpad with a bond curve pricing mechanism and anti-bundling features.

## Features

### Core Functionality
- **Bond Curve Pricing**: Tokens are priced according to an exponential bond curve, where price increases as supply increases.
- **SOL-Based Trading**: Buy and sell tokens using SOL, with automatic WSOL wrapping/unwrapping.
- **Token-2022 Integration**: All tokens are created using the Token-2022 program with transfer hooks.
- **Graduation Process**: Tokens graduate to Raydium liquidity pools when they reach $100k market cap.

### Anti-Bundling Mechanism
- **Bundle Detection**: Identifies wallets that are connected to one another and collectively hold a high percentage of tokens.
- **Relationship Tracking**: Tracks wallet relationships based on transaction history.
- **100% Tax**: Applies a 100% tax on transfers from bundling wallets (those exceeding 5% threshold).
- **Automatic Monitoring**: Continuously monitors wallet balances and relationships.

### Fee Structure
- **1% Trading Fee**: Applied on all trades within the launchpad (0.5% to creator, 0.5% to platform).
- **2% External Transfer Fee**: Applied on transfers outside the launchpad.

## Technical Architecture

### Smart Contract Components
- **Bond Curve**: Implements exponential pricing for token buying and selling.
- **Bundle Detection**: Tracks wallet relationships and detects bundling.
- **Transfer Hook**: Implements Token-2022 transfer hook for fee collection and anti-bundling enforcement.
- **WSOL Handling**: Manages wrapping and unwrapping of SOL to WSOL.
- **Graduation**: Handles token graduation to Raydium liquidity pools.

### Frontend Components
- **React UI**: User interface for interacting with the launchpad.
- **Client Library**: TypeScript library for communicating with the smart contract.

## Getting Started

### Prerequisites
- Node.js v14+
- Rust and Solana CLI
- Anchor Framework

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/bond-curve-launchpad.git
cd bond-curve-launchpad
```

2. Install dependencies:
```bash
# Install Rust dependencies
cd programs/bond_curve_launchpad
cargo build

# Install JavaScript dependencies
cd ../../app
npm install
```

3. Build the program:
```bash
anchor build
```

4. Deploy the program:
```bash
anchor deploy
```

5. Start the frontend:
```bash
cd app
npm start
```

## Usage

### Creating a Token
1. Connect your wallet
2. Fill in the token details (name, symbol, initial price, curve parameters)
3. Click "Create Token"

### Buying Tokens
1. Select a token from the dropdown
2. Enter the amount to buy
3. Click "Buy Tokens"

### Selling Tokens
1. Select a token from the dropdown
2. Enter the amount to sell
3. Click "Sell Tokens"

### Graduating a Token (Admin Only)
1. Wait until a token reaches the graduation threshold ($100k market cap)
2. Click "Graduate to Raydium" in the admin panel

## Anti-Bundling Mechanism

The anti-bundling mechanism works as follows:

1. **Relationship Detection**: The system tracks transactions between wallets to establish relationships.
2. **Bundle Calculation**: For each wallet, the system calculates the total token balance held by all related wallets.
3. **Threshold Enforcement**: If a bundle exceeds 5% of the total token supply, all wallets in the bundle are marked as "bundling".
4. **Transfer Tax**: Transfers from bundling wallets are subject to a 100% tax, effectively preventing them from transferring tokens.
5. **Unbundling**: To remove the bundling status, wallets must reduce their collective holdings below the 5% threshold.

## Bond Curve Implementation

The bond curve is implemented as an exponential function:

```
price = initial_price * (base/10000)^supply
```

Where:
- `initial_price` is the starting price of the token
- `base` is the rate of price increase (in basis points, 10000 = 1.0)
- `supply` is the current token supply

For example, with a base of 10100 (1.01x), each token minted increases the price by 1%.

## Token-2022 Transfer Hook

The transfer hook is used to:

1. Detect and prevent transfers from bundling wallets
2. Apply a 2% fee on external transfers (outside the launchpad)

## License

This project is licensed under the MIT License - see the LICENSE file for details.