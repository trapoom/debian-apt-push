# Monitor-Sys

> A high-performance Linux System Monitor Terminal User Interface (TUI) written in Rust.

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![Version](https://img.shields.io/badge/version-1.0.1-blue.svg)
![Platform](https://img.shields.io/badge/platform-Linux-lightgrey.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

`monitor-sys` is a native Linux system monitoring application designed to provide real-time hardware telemetry through a modern terminal interface. Written entirely in Rust, the project emphasizes performance, reliability, predictable resource usage, and direct integration with Linux kernel interfaces.

Instead of relying on heavyweight frameworks or external monitoring services, `monitor-sys` collects information directly from the operating system using the standard virtual filesystems exposed by the Linux kernel. The result is a responsive monitoring application with minimal runtime overhead and a clean architecture that is suitable for both everyday system observation and long-running monitoring sessions.

---

# Table of Contents

* Overview
* Features
* Architecture
* System Data Collection
* Terminal Rendering
* Image Rendering Pipeline
* Installation
* Usage
* Project Structure
* Design Principles
* Performance
* Security
* Roadmap
* Contributing
* License

---

# Overview

The primary objective of `monitor-sys` is to demonstrate how a modern terminal application can provide comprehensive system monitoring while maintaining a lightweight runtime footprint.

The application combines low-level Linux telemetry, efficient asynchronous event handling, and a structured rendering pipeline to deliver a responsive terminal dashboard without sacrificing maintainability or code quality.

Core goals include:

* Native Linux integration
* Efficient resource utilization
* Memory-safe implementation
* Predictable performance
* Clean software architecture
* Extensible module organization

---

# Features

## Real-Time Hardware Telemetry

The application continuously monitors essential system information by reading directly from Linux kernel interfaces.

Collected information includes:

* CPU utilization
* Memory usage
* System uptime
* Temperature sensors
* Battery information
* Power supply status

---

## Direct Kernel Integration

Rather than relying on external monitoring daemons or unnecessary abstraction layers, `monitor-sys` reads information directly from Linux virtual filesystems.

Typical sources include:

```
/proc/stat
/proc/meminfo
/sys/class/thermal
/sys/class/power_supply
```

This approach minimizes dependencies while providing accurate and up-to-date system information.

---

## Battery Monitoring

The power management subsystem automatically discovers battery devices exposed by the Linux kernel.

Supported information includes:

* Battery capacity
* Charging status
* Current voltage
* Power source
* Remaining energy
* Health information (when available)

Automatic device discovery allows the application to adapt to different hardware configurations without additional configuration.

---

## Interactive Command Interface

The dashboard includes an integrated command interface that allows users to execute application-specific commands without leaving the monitoring environment.

The command subsystem supports:

* Interactive command execution
* Command history
* Input editing
* Extensible command registration

This architecture makes future feature expansion significantly easier.

---

## Terminal Rendering Engine

Rendering is performed using a dedicated drawing layer built on top of Ratatui and Crossterm.

The renderer provides:

* Dynamic layouts
* Live dashboard updates
* Automatic screen resizing
* Alternate screen management
* Efficient redraw scheduling

The terminal state is properly restored after application shutdown, ensuring that no artifacts remain in the user's console.

---

## Raw Image Rendering

`monitor-sys` includes an experimental renderer capable of displaying raw image buffers inside the terminal.

Supported input formats:

```
RGB

RGBA
```

Expected dimensions:

```
512 × 512 pixels
```

The rendering pipeline transforms raw pixel buffers into terminal-compatible character representations, allowing image visualization without requiring native graphical capabilities.

---

# Architecture

The project is organized around a modular architecture that separates hardware monitoring, event processing, rendering, and application state management.

```
                Linux Kernel

        +-----------------------+
        |  /proc    /sys        |
        +-----------+-----------+
                    |
                    |
                    v

          Telemetry Collection Layer
                    |
                    |
                    v

             System State Model
                    |
                    |
                    v

          Application Controller
                    |
          +---------+---------+
          |                   |
          |                   |
          v                   v

     Event Handler      Rendering Engine
          |                   |
          +---------+---------+
                    |
                    v

              Terminal Output
```

Each subsystem has a clearly defined responsibility, allowing the project to remain maintainable as additional functionality is introduced.

---

# Threading Model

`monitor-sys` separates user interaction from telemetry updates to maintain a responsive interface during continuous monitoring.

The architecture consists of two primary execution paths.

## Event Processing

Responsible for:

* Keyboard events
* Window resize events
* User commands
* Application control

```
Keyboard
     |
     v
Event Listener
     |
     v
Message Channel
     |
     v
Main Application
```

---

## Monitoring Loop

Responsible for:

* Reading kernel data
* Parsing hardware information
* Updating internal state
* Triggering redraw operations

```
Linux Filesystem

      |

Telemetry Parser

      |

System State

      |

Renderer

      |

Display
```

Separating these responsibilities improves responsiveness while simplifying synchronization between components.

---

# System Data Collection

The monitoring subsystem periodically reads kernel-provided information and converts it into strongly typed Rust structures.

Typical workflow:

```
Linux Virtual Filesystem

        |

Read File

        |

Parse Raw Text

        |

Validate Values

        |

Update System State

        |

Render Dashboard
```

This design keeps parsing logic isolated from rendering logic, making both systems easier to maintain and test.

---

# Terminal Rendering

Rendering is performed independently from telemetry collection.

Each frame is generated from an immutable snapshot of the current application state.

Advantages include:

* Consistent rendering
* Reduced flickering
* Predictable updates
* Simplified state management

Only the current application state is required to produce a frame, allowing rendering to remain deterministic and independent of hardware access.

---

# Image Rendering Pipeline

The raw image renderer accepts uncompressed RGB or RGBA byte buffers.

Pipeline overview:

```
Raw Image

      |

Pixel Decoder

      |

Color Conversion

      |

Intensity Mapping

      |

Character Selection

      |

Terminal Frame

      |

Display
```

Example conversion using Python:

```python
from PIL import Image

image = (
    Image.open("image.png")
    .convert("RGBA")
    .resize((512, 512))
)

with open("image.raw", "wb") as file:
    file.write(image.tobytes())
```

Load the generated file from the command interface:

```text
image load /path/to/image.raw
```

---

# Installation

## Package Installation

For Debian-based distributions:

```bash
sudo apt update
sudo apt install monitor-sys
```

---

## Building From Source

Clone the repository:

```bash
git clone https://github.com/trapoom/moniter-sys.git

cd moniter-sys
```

Build the release binary:

```bash
cargo build --release
```

Run the application:

```bash
./target/release/monitor
```

---

# Usage

Start the application:

```bash
monitor
```

Once running, the dashboard continuously updates system information while accepting interactive commands through the integrated command interface.

---

# Project Structure

```
monitor-sys/

├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── sysinfo.rs
│   ├── renderer.rs
│   ├── commands.rs
│   ├── events.rs
│   └── ui.rs
│
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── README.md
```

Typical module responsibilities:

| Module        | Responsibility                |
| ------------- | ----------------------------- |
| `main.rs`     | Application entry point       |
| `app.rs`      | Global application state      |
| `sysinfo.rs`  | Hardware telemetry collection |
| `renderer.rs` | Terminal rendering            |
| `commands.rs` | Command processing            |
| `events.rs`   | Event handling                |
| `ui.rs`       | Layout construction           |

---

# Design Principles

The project is built around several fundamental principles.

## Simplicity

Each module should have a single, clearly defined responsibility.

## Performance

Avoid unnecessary allocations, excessive copying, and redundant processing whenever practical.

## Reliability

Rust's ownership and type systems are used extensively to reduce runtime errors and eliminate common classes of memory bugs.

## Maintainability

The project favors modular organization and descriptive code over unnecessary abstraction.

## Extensibility

New telemetry sources, rendering widgets, and commands should be implementable without requiring significant architectural changes.

---

# Performance

Several design decisions contribute to the application's efficiency.

* Direct kernel data collection
* Low-overhead parsing
* Efficient terminal rendering
* Controlled memory allocation
* Minimal dependency footprint
* Message-passing concurrency
* Native compiled performance

The application is intended to remain lightweight even during extended monitoring sessions.

---

# Security

`monitor-sys` does not require elevated privileges for normal operation.

The application:

* Reads only local system information
* Does not transmit telemetry externally
* Does not install background services
* Does not modify operating system configuration

All monitored information remains on the local machine.

---

# Roadmap

Future development plans include:

* GPU monitoring
* Network bandwidth visualization
* Disk I/O statistics
* Process management interface
* Configurable dashboard layouts
* Theme customization
* Plugin architecture
* Remote monitoring support
* Exportable metrics
* Historical data visualization

---

# Contributing

Contributions are welcome.

To contribute:

1. Fork the repository.
2. Create a feature branch.
3. Implement the proposed changes.
4. Ensure the project builds successfully.
5. Submit a pull request with a clear description of the modifications.

Issues, feature requests, documentation improvements, and code contributions are all appreciated.

---

# License

This project is distributed under the MIT License.

See the `LICENSE` file for complete license information.
