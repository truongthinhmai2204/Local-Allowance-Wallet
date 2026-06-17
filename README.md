# Local Allowance Wallet (Soroban)

A user-facing financial application built on Stellar Soroban.

## Overview

Local Allowance Wallet lets a sponsor (parent, school, employer) allocate funds
to a recipient with spending categories and optional expiration.

Examples:
- Food allowance
- Education allowance
- Transportation allowance

## Features

- Create allowance
- Spend allowance
- Category restriction
- Expiration support
- Transaction history events

## Contract Storage

### Allowance

```rust
pub struct Allowance {
    pub sponsor: Address,
    pub recipient: Address,
    pub category: Symbol,
    pub amount: i128,
    pub spent: i128,
    pub expires_at: u64,
}
```

## Build

```bash
cargo build --target wasm32v1-none
```

## Deploy

```bash
stellar contract deploy   --wasm target/wasm32v1-none/release/local_allowance_wallet.wasm   --source alice   --network testnet
```

## Functions

- create_allowance
- spend
- get_allowance

## Events

- allowance_created
- allowance_spent

## Future Work

- QR payment flow
- Merchant whitelist
- Stable asset integration
- Wallet integration
