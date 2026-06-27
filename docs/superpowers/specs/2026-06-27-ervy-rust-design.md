# Ervy Rust Rewrite — Design Spec

**Date:** 2026-06-27
**Version:** 0.1.0
**Status:** Approved

## Overview

Rewrite [Ervy](https://github.com/chunqiuyiyu/ervy) — a JavaScript library for rendering charts in the terminal — into idiomatic Rust. The result is a Cargo library crate (`ervy`) and a CLI binary (`ervy-cli`) that renders all 6 chart types with colored ASCII/Unicode output.

## Architecture

```
ervy/
├── Cargo.toml              # workspace root
├── Cargo.lock
├── README.md
├── LICENSE                  # MIT
├── CHANGELOG.md
├── .github/
│   └── workflows/
│       └── ci.yml
├── crates/
│   ├── ervy/              # library crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── datum.rs
│   │       ├── bar.rs
│   │       ├── pie.rs
│   │       ├── bullet.rs
│   │       ├── donut.rs
│   │       ├── gauge.rs
│   │       ├── scatter.rs
│   │       ├── color.rs
│   │       └── options.rs
│   └── ervy-cli/           # CLI binary
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
├── docs/
│   └── superpowers/specs/
│       └── 2026-06-27-ervy-rust-design.md
└── examples/
    └── demo.rs
```

Workspace layout separates library from binary. `ervy` library exports via `pub use`. `ervy-cli` wraps with `clap`. No `unsafe`.

## Core Types

```rust
// color.rs
pub enum Color { Black, Red, Green, Yellow, Blue, Magenta, Cyan, White }

#[derive(Clone, Debug, Default)]
pub struct Style {
    pub fg: Option<(Color, char)>,
    pub bg: Option<(Color, usize)>,
}

// datum.rs
pub struct Datum {
    pub key: String,
    pub value: DatumValue,
    pub style: Option<Style>,
    pub sides: Option<[usize; 2]>,
}

pub enum DatumValue { Scalar(f64), Point([f64; 2]) }
```

Helpers: `pub fn fg(color: Color, ch: char) -> Style`, `pub fn bg(color: Color, len: usize) -> Style`.

## Chart Functions

All return `String`. Gracefully handle edge cases (empty data → empty string, no panics).

| Function | Signature |
|----------|-----------|
| `pub fn bar(data: &[Datum], opts: &BarOptions) -> String` | |
| `pub fn pie(data: &[Datum], opts: &PieOptions) -> String` | |
| `pub fn bullet(data: &[Datum], opts: &BulletOptions) -> String` | |
| `pub fn donut(data: &[Datum], opts: &DonutOptions) -> String` | |
| `pub fn gauge(data: &[Datum], opts: &GaugeOptions) -> String` | |
| `pub fn scatter(data: &[Datum], opts: &ScatterOptions) -> String` | |

## Options

All derive `Default` + `Clone + Debug`. All `pub` fields.

```rust
pub struct BarOptions {
    pub bar_width: usize,    // default: 3
    pub left: usize,         // default: 1
    pub height: usize,        // default: 6
    pub padding: usize,      // default: 3
    pub style: char,         // default: '*'
}

pub struct PieOptions {
    pub left: usize,         // default: 1
}

pub struct BulletOptions {
    pub width: usize,        // default: 30
    pub bar_width: usize,    // default: 1
    pub style: char,         // default: '+'
    pub height: usize,        // default: 6
}

pub struct DonutOptions {
    pub left: usize,         // default: 1
    pub gap_char: Option<Style>,
}

pub struct GaugeOptions {
    pub radius // default: 5
    pub style: Option<Style>,
    pub bg_style: Option<Style>,
    pub show_percentage: bool,
}

pub struct ScatterOptions {
    pub width: usize,        // default: 15
    pub legend_gap: usize,   // default: 18
}
```

## CLI Design

Via `clap`:

```
ervy bar    [--data <json>] [--data-file <path>] [--width 3] [--height 6] [--padding 3] [--left 1] [--style '*']
ervy pie    [--data <json>] [--left 1]
ervy bullet [--data <json>] [--width 30] [--bar-width 2] [--style '+']
ervy donut  [--data <json>] [--left 1] [--gap-char <style>]
ervy gauge  [--data <json>] [--radius 7]
ervy scatter [--data <json>] [--width 15] [--legend-gap 18]
ervy fg     <color> <character>
ervy bg     <color> [--length <n>]
```

Data JSON format:
```json
[{"key": "A", "value": 5, "style": {"fg": "red", "char": "*"}}, ...]
```

Scatter:
```json
[{"key": "A", "value": [3, 4], "style": {"fg": "red", "char": "*"}}, ...]
```

## CI/CD

- `cargo test` on stable/beta/nightly × Linux/macOS/Windows
- `cargo clippy` + `cargo fmt --check`
- Cross-compile: x86_64-linux, aarch64-linux, x86_64-macos, x86_64-windows-msvc
- Release on tag `v*` → GitHub Release with binary artifacts

## Cargo.toml metadata

- library: `ervy` v0.1.0, MIT
- categories: `visualization`, `command-line-utilities`
- keywords: `chart`, `terminal`, `ascii`, `tui`

## LICENSE

MIT (matching original).
