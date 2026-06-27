# Changelog

## [v0.1.0](https://github.com/hermes98761234/ervy/releases/tag/v0.1.0) — 2026-06-27

### Added
- Initial release
- 6 chart types: Bar, Pie, Bullet, Donut, Gauge, Scatter
- `ervy` library crate — pure `fn chart(data, options) -> String`
- `ervy-cli` binary — JSON input via stdin or file
- 256-color ANSI output with `fg()` / `bg()` helpers
- Cross-platform builds:
  - Linux x86_64, aarch64, armv7
  - macOS x86_64, aarch64 (Apple Silicon)
  - Windows x86_64
- CI/CD: lint (fmt+clippy), test, cross-compile, auto-release on tag
