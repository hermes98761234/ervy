<div align="center">

# Ervy

**Bring charts to your terminal** — Rust rewrite of [Ervy](https://github.com/chunqiuyiyu/ervy)

[![CI](https://github.com/hermes98761234/ervy/actions/workflows/ci.yml/badge.svg)](https://github.com/hermes98761234/ervy/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/ervy)](https://crates.io/crates/ervy)

*From this* → *this* :

```
  ██████████████████████████  A (5.0)
  █████████████              B (3.0)
```

</div>

## Features

| Chart | Description |
|-------|-------------|
| **Bar** | Horizontal/vertical bar chart |
| **Pie** | Colored pie chart |
| **Bullet** | Bullet meter (usage / capacity) |
| **Donut** | Donut / ring chart |
| **Gauge** | Semicircular gauge (0–100%) |
| **Scatter** | Scatter plot (x, y, label, color) |

- 🎨 **256-color** ANSI output with `fg()` / `bg()` helpers
- 📦 Pure Rust, no `unsafe`, zero rendering dependencies
- 🔢 JSON input via CLI or library API
- 📊 6 chart types from a single crate

## Installation

```bash
cargo install ervy-cli
```

Or grab a prebuilt binary from the [Releases](https://github.com/hermes98761234/ervy/releases) page.

## Usage

### Library

```rust
use ervy::{bar, Datum, fg, Color};

let data = vec![
    Datum::styled("A", 5.0, fg(Color::Red, '*')),
    Datum::scalar("B", 3.0),
];
println!("{}", bar(&data, &ervy::BarOptions::default()));
```

### CLI

```bash
# Bar chart from stdin
echo '[{"key":"A","value":5},{"key":"B","value":3}]' | ervy bar

# Gauge with radius
ervy gauge --data '[{"key":"CPU","value":0.75}]' --radius 7

# Scatter from JSON file
ervy scatter --data-file points.json
```

## Supported Platforms

| Platform | Architectures |
|----------|--------------|
| Linux | x86_64, aarch64, armv7 |
| macOS | x86_64, aarch64 (Apple Silicon) |
| Windows | x86_64 |

## Architecture

```
ervy/
├── crates/
│   ├── ervy/          # Library (chart rendering)
│   └── ervy-cli/      # Binary (clap + JSON input)
├── docs/
│   ├── spec.md        # Project specification
│   └── plans/         # Implementation plans
└── .github/
    └── workflows/ci.yml  # CI / CD + cross-compilation
```

## How It Works

Charts are pure functions with this signature:

```rust
fn chart(data: &[Datum], options: &Options) -> String
```

Colors are applied using ANSI escape codes. The `Datum` type supports:
- `Datum::scalar("label", 5.0)` — plain value
- `Datum::styled("label", 5.0, fg(Color::Red, '*'))` — custom color + marker

## Development

```bash
# Run all checks (same as CI)
cargo fmt -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace

# Build release binary locally
cargo build --release -p ervy-cli
```

## License

MIT

---

<sub>Built with 🦀 by Volodymyr Kopytsia</sub>
