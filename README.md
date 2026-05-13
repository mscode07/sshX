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



##📦 Installation

Install from crates.io
cargo install mscode-sshx
Build from source
git clone https://github.com/mscode07/sshx.git
cd sshx
cargo build --release

Binary will be available at:

target/release/sshx

## 🚀 Usage
Generate a new SSH identity
sshx generate --name github

You will be prompted for a master password. The private key will be encrypted before being stored.

List stored identities
sshx list

Example output:

Stored keys:

- github
- server
Get public key
sshx get github --public
Get private key
sshx get github

You will be prompted for your master password before the private key is decrypted.

Delete an identity
sshx delete github
🔐 Security Model

Private keys are never stored in plaintext.

Encryption Process
User enters a master password
Argon2 derives a 256-bit encryption key
A random salt is generated
A random nonce is generated
Private key is encrypted using AES-256-GCM
Encrypted data is stored locally
Storage Format
salt:nonce:ciphertext
Vault Location
~/.sshx/vault/

Example:

~/.sshx/vault/
├── github
├── github.pub
├── server
└── server.pub
📁 Project Structure
sshx/
├── src/
│   ├── cli/
│   │   ├── mod.rs
│   │   └── commands.rs
│   ├── crypto/
│   │   ├── mod.rs
│   │   ├── ssh.rs
│   │   └── encrypt.rs
│   ├── storage/
│   │   ├── mod.rs
│   │   └── vault.rs
│   └── main.rs
├── Cargo.toml
└── README.md
🧪 Example Workflow
# Generate encrypted SSH identity
sshx generate --name github

# List stored identities
sshx list

# Show public key
sshx get github --public

# Show decrypted private key
sshx get github

# Delete identity
sshx delete github
🛣️ Roadmap
 Generate SSH keys
 Encrypted private key storage
 List identities
 Retrieve and decrypt keys
 Delete identities
 Config file support
 Metadata (creation date, fingerprint)
 Improved error handling with anyhow
 Automated tests
 GitHub Actions CI/CD
 Session-based password caching
🛠️ Technologies Used
Rust
clap
ssh-key
Argon2
AES-GCM
rpassword
dirs
base64
💼 Why This Project Matters

sshx demonstrates:

Systems programming in Rust
Cryptography and secure key management
Filesystem persistence
Modular software architecture
CLI application design

This project was built as a production-style tool rather than a toy project.

🤝 Contributing

Contributions, issues, and feature requests are welcome.

Fork the repository
Create a feature branch
Commit your changes
Open a pull request
📄 License

Licensed under either of:

MIT License
Apache License 2.0

at your option.

👨‍💻 Author

Built by Abhishek Thakur (mscode07)

GitHub: https://github.com/mscode07
X/Twitter: https://x.com/mscode07
