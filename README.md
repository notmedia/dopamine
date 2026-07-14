# dopamine

A tiny macOS [`caffeinate`](https://ss64.com/mac/caffeinate.html) alternative written in Rust. Keeps your Mac awake exactly as long as you ask.

## Usage

```sh
dop                 # prevent idle sleep until Ctrl-C
dop -d              # keep the display awake
dop -id -t 1h30m    # both assertions, released after 1.5 hours
dop -t 20m          # idle sleep prevented for 20 minutes
```

| Flag | Meaning |
| --- | --- |
| `-i`, `--idle` | Prevent system idle sleep (default when no flags are given) |
| `-d`, `--display` | Prevent the display from sleeping |
| `-t`, `--timeout` | How long to stay awake — human-friendly strings like `30s`, `5m`, `1h30m`. Forever if omitted |

`dop` releases its power assertions on any exit: timeout, Ctrl-C, `SIGTERM`, or `SIGHUP`.

To see the assertions live while `dop` runs:

```sh
pmset -g assertions | grep dopamine
```

## Install

Requires macOS and a Rust toolchain.

```sh
cargo install --path crates/cli
```

Or build without installing: `cargo build --release` → `target/release/dop`.

## How it works

`dop` creates [IOKit power assertions](https://developer.apple.com/documentation/iokit/iopmlib_h) (`PreventUserIdleSystemSleep` / `PreventUserIdleDisplaySleep`) through hand-rolled FFI — no bindgen, no `-sys` crates. Assertions are owned by RAII guards, so releasing them is not a code path that can be missed: dropping the guard releases the assertion, and the OS cleans up if the process dies.

## Workspace

- `crates/core` — `dopamine-core`, the library. Safe public API (`AwakeGuard::acquire(&Config)`); all `unsafe` FFI stays inside, gated per platform.
- `crates/cli` — `dop`, the command-line frontend. Flag parsing, timeout, and signal handling live here; the core stays policy-free.

## Development

```sh
cargo test                  # all crates
cargo clippy --all-targets  # clippy `all` + `pedantic`, workspace-wide
cargo +nightly fmt          # rustfmt uses unstable options — nightly required
```
