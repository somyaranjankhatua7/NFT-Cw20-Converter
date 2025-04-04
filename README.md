# NFT-Cw20-Converter
Convert NFTs into tradable CW20 tokens. This project wraps NFTs into fungible tokens to enable easy trading, liquidity, and fractional ownership on the Cosmos ecosystem.
This project enables the **conversion of NFTs into CW20 fungible tokens** within the Cosmos ecosystem. By wrapping NFTs into tradable CW20 tokens, users can unlock liquidity, enable fractional ownership, and make NFTs more accessible for DeFi applications.

---

## 🔧 How It Works

- **NFT Deposit:** A user deposits their NFT into the contract.
- **Minting CW20:** The contract mints an equivalent amount of CW20 tokens representing the NFT.
- **Trading Enabled:** The CW20 tokens can now be freely transferred, traded, or used in other DeFi protocols.
- **Redeem NFT:** Holders can burn CW20 tokens to redeem the original NFT.

---

## ✨ Features

- Convert any supported NFT into a tradable CW20 token
- Maintain a 1:1 link between the NFT and its fungible representation
- Supports redemption of NFTs by burning CW20 tokens
- Enhances liquidity and utility of NFTs in the Cosmos ecosystem

---

## 🧱 Tech Stack

- CosmWasm Smart Contracts
- CW721 (NFT Standard)
- CW20 (Fungible Token Standard)
- Rust

---

## 🚀 Getting Started

### Prerequisites

- Rust & wasm32 target
- [wasmd](https://github.com/CosmWasm/wasmd) or compatible localnet
- Node.js (for frontend or scripts)

### Build Contract

```bash
cargo wasm
