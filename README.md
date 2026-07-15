# moniter

[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Debian%20%7C%20Ubuntu-red.svg)]()
[![Release](https://img.shields.io/github/v/release/trapoom/debian-apt-push)]()

A lightweight system monitoring utility written in Rust.

This repository hosts the signed APT repository, source code, and release artifacts for **moniter**.

`moniter` is a Linux monitoring tool designed to display real-time CPU usage, RAM usage, and battery information on Debian and Ubuntu systems.

---

# Features

- Written in Rust
- Real-time CPU usage monitoring
- RAM usage monitoring
- Battery status monitoring
- Battery percentage display
- Battery charging state detection
- Lightweight resource usage
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
curl -fsSL hhttps://github.com/trapoom/moniter-sys/setup.sh | sudo bash
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
https://github.com/trapoom/moniter-sys/
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

```

Verify the key:

```bash
-----BEGIN PGP PUBLIC KEY BLOCK-----

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
=j7MY
-----END PGP PUBLIC KEY BLOCK-----
```
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
git clone https://github.com/trapoom/moniter-sys/.git
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
