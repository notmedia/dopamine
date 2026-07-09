# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

**dopamine** — a macOS `caffeinate` alternative written in Rust. Prevents the system from sleeping. The CLI binary is named `dop`.

## Collaboration mode

The author is learning Rust. Default to **teaching, not implementing**: explain concepts, review code, suggest approaches, and answer questions in chat. Only write code when explicitly asked to.

## Architecture

Cargo workspace (`resolver = "3"`; edition 2024 inherited from `[workspace.package]` via `edition.workspace = true`):

- `crates/core` (`dopamine-core`) — library with the sleep-prevention logic. The public API is RAII-based: `AwakeGuard::acquire(&Config) -> Result<AwakeGuard, Error>` — the system stays awake exactly as long as the guard lives; `Drop` releases the assertion. The guard's `id` field is deliberately private so a guard can only be obtained via `AwakeGuard::acquire` ("guard exists ⇔ assertion held" is compiler-enforced). Keep it that way, and never make the guard `Copy`/`Clone`. `Config` is currently accepted but unused by core (its `timeout` is CLI policy) — its final shape is an open design question.
- `crates/cli` (`dop`) — clap-derive CLI; `src/cli.rs` holds the `Cli` parser struct. Design rule: **mechanism in core, policy in frontends** — the timeout is CLI policy. The CLI sleeps for `--timeout-ms` (or parks forever) and lets the guard drop on exit; core never blocks.
- `crates/ui` — planned Tauri desktop app consuming `dopamine-core` directly (no cli dependency). Not created yet; scaffold with `create-tauri-app` when started.

Platform FFI is hand-rolled (no bindgen/-sys crates) and lives in `crates/core/src/macos/`, gated by `#[cfg(target_os = "macos")]` and aliased in `lib.rs` as `use macos as platform;` — `lib.rs` only calls `platform::acquire` / `platform::release`, so a new platform is a sibling module exposing the same two functions (a `compile_error!` rejects unsupported targets). `macos/mod.rs` declares the IOKit externs (`IOPMAssertionCreateWithName` / `IOPMAssertionRelease`); `macos/cf.rs` declares the CoreFoundation externs and wraps them in `CfString`, an RAII owner that `CFRelease`s on drop (CF "Create rule"; like the guard, never `Copy`/`Clone`). Extern declarations stay private to the module that wraps them — sibling modules get safe wrappers, never raw FFI. All `unsafe` stays inside core; the public API remains safe. `Drop` impls must never panic — discard fallible cleanup results with `let _ =`.

## Commands

```sh
cargo build                    # build the whole workspace
cargo run -p dop -- -t 5000    # run the CLI (-t/--timeout-ms; omit to run forever)
cargo test                     # test all crates
cargo test -p dopamine-core    # test a single crate
cargo test <name>              # run a single test by name filter
cargo +nightly fmt             # format — MUST use nightly (see Style)
cargo clippy --all-targets     # lint (clippy `all` + `pedantic` at workspace level)
```

To verify a real power assertion is held: run `dop`, then check `pmset -g assertions` in another terminal for the "dopamine" entry.

## Style & tooling

- `rustfmt.toml` uses unstable options (`imports_granularity`, `group_imports`, ...), so formatting requires nightly rustfmt: `cargo +nightly fmt`. Stable rustfmt silently ignores those options — don't format with it. VS Code is wired up in `.vscode/settings.json` (`rust-analyzer.rustfmt.extraArgs: ["+nightly"]`).
- Lints are defined once in `[workspace.lints.clippy]` (root `Cargo.toml`); every crate opts in with `[lints] workspace = true` in its manifest. New crates must include that stanza.
