# 🪙 Rust Bitcoin Deterministic Wallet (BIP32/BIP38)

In this project I will build a deterministic Bitcoin wallet in Rust, leveraging:
- **BIP32**: Hierarchical Deterministic (HD) wallets
- **BIP38**: Passphrase-protected private keys
- **BDK** (Bitcoin Dev Kit)
- **Electrum server** for network sync and broadcasting

By the end of this tutorial, you’ll be able to:
- 🔐 Generate a master key from a seed
- 🔁 Derive child keys/wallets
- 💸 Send and receive BTC (on testnet!)
- 🔍 Understand wallet privacy techniques like address reuse prevention

---

## 📦 Features

- Generate master seed or use custom seed
- Derive extended private/public keys
- Manage Bitcoin testnet/mainnet support
- Electrum integration for syncing and broadcasting
- Send BTC transactions (with Taproot support if needed)
- Wallet descriptors for managing key purposes
- Optional persistent memory storage with memory DB

---

## 🧠 Concepts I learnt from this 

| Concept | Description |
|--------|-------------|
|  BIP32 | HD Wallets with tree-like key structures |
|  BIP38 | Password-encrypted private keys |
|  Extended Keys | Public and Private keys used to derive addresses |
|  Electrum | SPV-based lightweight Bitcoin network connection |
|  Descriptors | Define key purposes and formats |
|  Key Derivation | Parent-child wallet creation with security enhancement |
|  Privacy | Address rotation to avoid blockchain traceability |

---

