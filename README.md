# moniter

[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Debian%20%7C%20Ubuntu-red.svg)]()
[![Release](https://img.shields.io/github/v/release/trapoom/debian-apt-push)]()

A modern system monitoring utility written in Rust.

This repository hosts the signed APT repository, source code, and release artifacts for **moniter**.

`moniter` is a lightweight Linux monitoring tool designed for Debian and Ubuntu systems.

---

# Features

- Written in Rust
- CPU monitoring
- Memory monitoring
- Disk usage monitoring
- Network monitoring
- Temperature monitoring
- Native Linux support
- Debian package distribution
- Signed APT repository

---

# Supported Platforms

## Operating Systems

### Debian

- Debian 12 Bookworm
- Debian 13 Trixie

### Ubuntu

- Ubuntu 22.04 LTS
- Ubuntu 24.04 LTS

---

## Architectures

Supported architectures:

```
amd64
arm64
```

---

# Installation

## Install from APT Repository

### Step 1 - Add repository

```bash
curl -fsSL https://trapoom.github.io/debian-apt-push/setup.sh | sudo bash
```

The installer will:

- Download repository signing key
- Install GPG keyring:

```
/usr/share/keyrings/traphumi-archive-keyring.gpg
```

- Create APT source:

```
/etc/apt/sources.list.d/traphumi.list
```

- Run:

```bash
apt update
```

---

### Step 2 - Install moniter

```bash
sudo apt install moniter
```

---

# Repository Information

APT Repository:

```
https://trapoom.github.io/debian-apt-push/
```

Distribution:

```
stable
```

Component:

```
main
```

---

# Repository Signing Key

The repository packages are signed using GPG.

Fingerprint:

```
XXXX XXXX XXXX XXXX XXXX
XXXX XXXX XXXX XXXX XXXX
```

Verify the key:

```bash
gpg --show-keys traphumi-archive-keyring.gpg
```

---

# Build From Source

## Requirements

- Rust stable
- Cargo
- Git

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs | sh
```

---

## Clone

```bash
git clone https://github.com/trapoom/debian-apt-push.git
cd debian-apt-push
```

---

## Build

Development build:

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

Binary:

```
target/release/moniter
```

Run:

```bash
./target/release/moniter
```

---

# Example Output

Example:

```
System Monitor

CPU        7%
Memory     1.3G / 8G
Disk       41%
Network    13 MB/s
Temperature 42 C
```

---

# Project Structure

```
.
├── src/
│   ├── main.rs
│   └── modules/
│
├── Cargo.toml
├── Cargo.lock
├── debian/
│
├── setup.sh
├── LICENSE
├── README.md
└── CHANGELOG.md
```

---

# Configuration

Currently no configuration file is required.

Future versions may support:

```
~/.config/moniter/config.toml
```

---

# Package Management

Update repository information:

```bash
sudo apt update
```

Upgrade:

```bash
sudo apt upgrade
```

---

# Uninstall

Remove the application:

```bash
sudo apt remove moniter
```

Remove repository:

```bash
sudo rm /etc/apt/sources.list.d/traphumi.list
sudo rm /usr/share/keyrings/traphumi-archive-keyring.gpg
sudo apt update
```

---

# Development

Before submitting changes, run:

Format:

```bash
cargo fmt
```

Test:

```bash
cargo test
```

Lint:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

---

# Contributing

Pull requests are welcome.

Before submitting:

```bash
cargo fmt
cargo test
cargo clippy
```

Please describe your changes clearly.

---

# Security

If you discover a security vulnerability, please report it privately.

Do not open a public issue containing sensitive information.

Security reports can be submitted through GitHub Security Advisories.

---

# Roadmap

Future plans:

- ARM builds
- Automatic update checker
- Homebrew support
- Snap package
- Flatpak package
- Configuration file support
- More system metrics

---

# Changelog

See:

```
CHANGELOG.md
```

Current release:

```
1.0.0
```

---

# License

This project is licensed under the MIT License.

See:

```
LICENSE
```

Cargo.toml:

```toml
license = "MIT"
```

---

# Author

trapoom

GitHub:

https://github.com/trapoom
