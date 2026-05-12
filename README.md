# sshx 🔐

A secure SSH identity manager written in Rust.

`sshx` helps developers generate, store, and manage SSH keys with encrypted private key storage using Argon2 and AES-256-GCM.

---

## ✨ Features

- 🔑 Generate Ed25519 SSH key pairs
- 🔐 Encrypt private keys with a master password
- 🛡️ Argon2 password-based key derivation
- 🔒 AES-256-GCM authenticated encryption
- 📂 Secure local vault storage
- 📋 List all stored identities
- 📤 Retrieve public and private keys
- 🗑️ Delete stored identities
- 👀 Hidden password input in terminal

---

## 🏗️ Architecture

```text
CLI (clap)
   ↓
Command Handlers
   ↓
Crypto Layer
   ├── SSH Key Generation
   └── Encrypt / Decrypt
   ↓
Storage Layer
   └── ~/.sshx/vault
