# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Language

The user is a Japanese speaker. All conversations and responses should be in Japanese by default.

## Project Overview

microrps is a self-made TCP/IP protocol stack written in Rust, purely for learning purposes. The domain crate is `no_std` to ensure platform independence.

## Build & Development Commands

```bash
cargo build                    # Build default members (domain, platform/linux, samples/linux)
cargo test                     # Run all tests
cargo fmt --all -- --check     # Check formatting
cargo clippy --all-targets --all-features  # Lint (CI runs with RUSTFLAGS="-Dwarnings")
cargo run -p samples-linux     # Run the Linux sample binary
```

## Architecture

The project uses a **layered architecture with dependency injection** via traits:

```
samples/linux        -- Binary: wires platform impl to domain
  └─ depends on ─→ protocolstack/domain         -- Core logic (#![no_std], uses alloc)
  └─ depends on ─→ protocolstack/platform/linux  -- Linux-specific PAL implementation
                     └─ depends on ─→ protocolstack/domain
```

- **`protocolstack/domain`** (`domain` crate): Platform-agnostic core (`#![no_std]`). Defines the `Platform` trait in `pal.rs` which abstracts platform-specific behavior (e.g., logger creation/initialization via `Log` trait bound). Uses `log` crate facade for logging.
- **`protocolstack/platform/linux`** (`pal-linux` crate): Implements `Platform` trait for Linux (e.g., `Linux` struct with `SimpleLogger`).
- **`samples/linux`**: Binary crate that calls domain functions with concrete platform types (e.g., `net_init::<Linux>()`).

Workspace `default-members` includes only domain, platform/linux, and samples/linux, so bare `cargo build` targets the Linux platform.

## Coding Standards

Full standards are in `docs/coding-standards.md`. Key points:

- **AI code generation is prohibited** -- this is a learning project. AI may be used for review/analysis only, not for writing code.
- **Formatting**: Use `rustfmt`. **Linting**: Use Clippy. Both are enforced in CI.
- Prefer lines under 80 characters (rustfmt allows up to 100).
- Minimize nesting depth; use early returns.
- Comments should have high information-to-space ratio. Don't explain HOW code works; write obvious code instead. Comments are complete sentences (capital letter, period).
- Panicking should be very rare. Prefer `Result`. Use `expect` over `unwrap` with context about why the operation should succeed.
- Follow Rust naming conventions: `UpperCamelCase` for types/variants, `snake_case` for functions/fields/variables/macros, `SCREAMING_SNAKE_CASE` for constants.
