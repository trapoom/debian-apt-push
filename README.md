# moniter

A modern system monitoring utility written in **Rust** for Debian and Ubuntu.

`moniter` is built using the standard Rust toolchain and can be installed either by compiling from source or through the official signed APT repository.

---

## Features

* Written in Rust
* Standard Cargo project
* Secure APT repository
* Debian and Ubuntu support
* GPG-signed package distribution
* Easy installation and updates

---

## Requirements

### Build from Source

* Rust (latest stable)
* Cargo
* Git

Install Rust using rustup:

```bash
curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs | sh
```

Verify the installation:

```bash
rustc --version
cargo --version
```

---

# Clone the Repository

```bash
git clone https://github.com/trapoom/moniter.git
cd moniter
```

---

# Build

Debug build:

```bash
cargo build
```

The binary will be generated at:

```text
target/debug/moniter
```

Release build:

```bash
cargo build --release
```

The optimized binary will be generated at:

```text
target/release/moniter
```

---

# Run

Using Cargo:

```bash
cargo run
```

Using the compiled binary:

```bash
./target/release/moniter
```

---

# Testing

Run all tests:

```bash
cargo test
```

Run formatting check:

```bash
cargo fmt --check
```

Run Clippy:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

---

# Install via APT Repository

Configure the repository:

```bash
curl -fsSL https://trapoom.github.io/debian-apt-push/setup.sh | sudo bash
```

Install:

```bash
sudo apt install moniter
```

Update:

```bash
sudo apt update
sudo apt upgrade
```

---

# Project Structure

```text
.
├── src/
│   ├── main.rs
│   └── ...
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── README.md
```

---

# Development

This project follows the standard Cargo workflow.

Common commands:

```bash
cargo build
cargo build --release
cargo run
cargo test
cargo fmt
cargo clippy
cargo clean
```

---

# Security

The Debian package repository is protected using a dedicated GPG signing key.

The installation script automatically:

* Downloads the repository key
* Installs the keyring
* Configures the APT source
* Updates the package index

---

# License

This project is licensed under the MIT License.

See the LICENSE file for details.

---

# Author

**trapoom**

GitHub: https://github.com/trapoom
