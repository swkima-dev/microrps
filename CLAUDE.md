# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

microrps is a self-made TCP/IP protocol stack written in Rust (`no_std`, with `alloc`). This is a learning project — all source code must be written manually by the developer. **AI must not generate code.** AI may only be used for review, analysis, and conceptual assistance.

## Build & Development Commands

```bash
cargo build                              # Build the library
cargo test --verbose                     # Run all tests
cargo test <test_name>                   # Run a single test
cargo fmt --all -- --check               # Check formatting
cargo clippy --all-targets --all-features  # Lint (CI runs with -Dwarnings)
```

CI enforces `RUSTFLAGS="-Dwarnings"` — all Clippy warnings are treated as errors.

## Architecture

This is a `no_std` library crate (`src/lib.rs`) using `extern crate alloc`.

The platform abstraction layer lives in `src/platform.rs`, which defines the `Platform` trait. OS-specific implementations are submodules (e.g., `src/platform/linux.rs` provides the `Linux` struct). The intent is to abstract raw network I/O per platform behind this trait.

## Coding Standards (from docs/coding-standards.md)

Key rules beyond standard Rust practices:

- **No AI code generation** — AI may review/analyze but must not write code
- Use `expect("reason")` instead of `unwrap()` — panics should be rare; prefer returning `Result`
- Minimize nesting — use early returns
- Follow DRY and single-responsibility principles
- Use `rustfmt` and Clippy without exception
- Rust edition: 2024
