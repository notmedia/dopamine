# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

**dopamine** — a macOS `caffeinate` alternative written in Rust. Prevents the system from sleeping. The CLI binary is named `dop`.

## Collaboration mode

The author is learning Rust. Default to **teaching, not implementing**: explain concepts, review code, suggest approaches, and answer questions in chat. Only write code when explicitly asked to.

## Architecture (planned)

Cargo workspace with three crates under `crates/`:

- `crates/core` — library crate with the sleep-prevention logic (IOKit power assertions on macOS). No CLI/UI dependencies; both frontends consume this.
- `crates/cli` — the `dop` binary, built with `clap`.
- `crates/ui` — desktop app built with Tauri.

Platform note: keeping the system awake on macOS goes through IOKit's `IOPMAssertionCreateWithName` (the same mechanism `caffeinate` uses). All platform-specific code belongs in `core`.

## Commands

```sh
cargo build                    # build the whole workspace
cargo run -p dop               # run the CLI
cargo test                     # test all crates
cargo test -p dopamine-core    # test a single crate
cargo test <name>              # run a single test by name filter
cargo fmt                      # format (style_edition 2024, see .rustfmt.toml)
cargo clippy --all-targets     # lint
```

## Style

- rustfmt with `style_edition = "2024"` (`.rustfmt.toml` at the workspace root).
