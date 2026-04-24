# Project Title

Carbon Credits – A Soroban Smart Contract for Carbon Credit Trading on Stellar

## Project Vision

This project enables **carbon credit trading on the Stellar blockchain** using Soroban smart contracts. It demonstrates:
- How to write a Soroban smart contract in Rust for carbon credit management
- How to manage persistent storage (balances and total supply tracking)
- How to handle user authentication and admin privileges in smart contracts
- How to deploy and interact with contracts on Stellar Testnet

The goal is to provide a foundation for carbon credit platforms to tokenize, trade, and retire carbon offsets on Stellar.

---

## Description

A Soroban smart contract that manages carbon credits on Stellar Testnet. The contract allows:
- **Admin** to mint carbon credits to addresses
- **Users** to transfer credits between accounts
- **Users** to retire (burn) credits to remove them from circulation
- **Anyone** to query balances and total supply

---

## Features

### 1. Carbon Credit Minting
- Admin can mint new carbon credits to any address
- Tracks total supply on-chain
- Transparent minting process

### 2. Credit Transfers
- Users can transfer credits to other addresses
- Requires authentication from sender
- Balance validation before transfer

### 3. Credit Retirement
- Users can retire (burn) their credits
- Retired credits are removed from circulation
- Enables carbon offset verification

### 4. Balance Queries
- Anyone can check credit balance of any address
- Total supply is publicly readable
- On-chain transparency

### 5. Soroban SDK Patterns
- Clear, readable Rust code
- Standard Soroban contract patterns
- Persistent storage with Map data structure

---

## Contract

- **Network**: Stellar Testnet
- **Contract ID**: [CAUML2JGQG6JT2AHHCIYPSGUISAIFUP6DNPKOIED5MDTNQNPHYLFM5DS](https://stellar.expert/explorer/testnet/tx/3141a9b4bd64136a1916326c8f6e7eef51e44ea220d03fc1b3b29fde5b0e7358)
- **Admin**: Initializes contract, can mint credits
- **Functions**:
  - `init(admin: Address)` - Initialize contract with admin
  - `mint_credits(owner: Address, amount: u64)` - Admin mints credits
  - `transfer_credits(from: Address, to: Address, amount: u64)` - Transfer credits
  - `retire_credits(user: Address, amount: u64)` - Retire (burn) credits
  - `get_balance(user: Address) -> u64` - Get credit balance
  - `get_total_supply() -> u64` - Get total credits in circulation

![screenshot](https://i.ibb.co/sd13S30K/image.png)

---

## Future Scopes

### 1. Verified Carbon Credits
- Integrate with carbon credit certification bodies
- Add metadata for credit origins

### 2. Automated Retirement
- Time-locked retirement schedules
- Batch retirement processing

### 3. Trading Marketplace
- Build a frontend dApp for trading
- Order book implementation

### 4. Credit Categories
- Support different types of carbon offsets
- Categorization by project type

### 5. Bridge Integration
- Bridge carbon credits from other blockchains
- Cross-chain carbon tracking

### 6. Governance
- DAO for protocol upgrades
- Community voting on credit standards

---

## Profile

- **Name:** <!-- Fill github name -->
