# moniter

A secure APT repository for Debian and Ubuntu.

`moniter` is distributed through a signed APT repository, allowing packages to be installed and updated using the standard `apt` package manager.

## Features

* Secure GPG-signed repository
* Easy one-line installation
* Compatible with Debian and Ubuntu
* Standard APT package management
* Automatic updates through `apt`

---

## Quick Start

### 1. Configure the repository

```bash
curl -fsSL https://trapoom.github.io/debian-apt-push/setup.sh | sudo bash
```

### 2. Install moniter

```bash
sudo apt install moniter
```

---

## Updating

Update package information:

```bash
sudo apt update
```

Upgrade installed packages:

```bash
sudo apt upgrade
```

---

## Repository Information

Repository URL

```
https://trapoom.github.io/debian-apt-push
```

Distribution

```
stable
```

Component

```
main
```

---

## Requirements

* Debian 11 or later
* Ubuntu 22.04 or later
* Internet connection
* `sudo` privileges

---

## Security

Packages are distributed through a GPG-signed repository.

The installation script automatically:

* Downloads the repository signing key
* Installs the keyring
* Configures the APT source
* Updates the package index

---

## Uninstall Repository

Remove the repository configuration:

```bash
sudo rm -f /etc/apt/sources.list.d/traphumi.list
sudo rm -f /usr/share/keyrings/traphumi-archive-keyring.gpg
sudo apt update
```

---

## License

See the LICENSE file for licensing information.

---

## Author

**trapoom**

GitHub: https://github.com/trapoom
