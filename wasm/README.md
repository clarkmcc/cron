# Cron Extism SDK

This directory contains a Rust SDK for the cron library, implemented by wrapping the Go library with Extism.

## Structure

- `go/`: Contains the Go Extism plugin that exposes the cron library functionality
- `rust/`: Contains the Rust SDK that uses the Extism plugin
- `Makefile`: Build automation for the Extism plugin

## How It Works

1. The Go Extism plugin (`go/main.go`) exposes key functions from the cron library:
   - `parse_schedule`: Parse a cron expression
   - `get_next_time`: Get the next activation time
   - `get_prev_time`: Get the previous activation time

2. The Makefile compiles the Go code to a WebAssembly plugin for Extism, producing `build/cron.wasm`

3. The Rust SDK loads and executes the Extism plugin, providing a clean API:
   - `Schedule::parse()`: Parse a cron expression
   - `Schedule::next()`: Get the next activation time
   - `Schedule::prev()`: Get the previous activation time

## Building

To build the entire SDK:

```bash
# Build the Extism plugin
cd wasm
make build-plugin

# Build the Rust SDK
cd rust
cargo build
```

## Usage

See the [Rust SDK README](rust/README.md) for detailed usage instructions.

## Requirements

- Go 1.20 or later (with WASI support)
- Rust 1.56 or later
- Extism SDK
