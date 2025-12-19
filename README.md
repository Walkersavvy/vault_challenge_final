![Solana Graduate](https://img.shields.io/badge/Solana-Anchor%20Vault%20Graduate-blueviolet?style=for-the-badge&logo=solana)

# Solana Anchor Vault 

A secure, non-custodial SOL vault built using the **Anchor Framework**. This program allows users to deposit lamports into a unique Program Derived Address (PDA) and withdraw them later.

##  Key Features
- **Deterministic PDAs:** Each user has exactly one vault, derived using their public key as a seed `[b"vault", signer.pubkey()]`.
- **Non-Custodial:** Only the original depositor (signer) can authorize a withdrawal from their specific PDA.
- **Security Guardrails:**
  - Prevents double-initialization of vaults.
  - Ensures deposits meet the minimum rent-exempt balance.
  - Uses Cross-Program Invocation (CPI) to the System Program for secure transfers.

## Tech Stack
- **Language:** Rust
- **Framework:** Anchor (Solana)
- **Deployment:** Solana Devnet / Localnet

## Logic Overview
1. **Deposit:** Transfers lamports from the user to the PDA vault.
2. **Withdraw:** Uses the PDA's seeds and a bump to programmatically sign the transfer of lamports back to the user.