# Cron SDK for Rust

This is a Rust SDK for parsing cron expressions and calculating next and previous schedule times. It wraps the Go cron library using Extism, a framework for building and running WebAssembly plugins.

## Features

- Parse cron expressions
- Calculate the next activation time after a given time
- Calculate the previous activation time before a given time
- Support for all cron expression features from the Go library
- Customizable parse options to support different cron formats (5-field, 6-field with seconds, etc.)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cron-sdk = "0.1.0"
```

## Usage

```rust
use cron_sdk::{Schedule, ParseOptions, options};
use chrono::{DateTime, Utc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a cron expression with default options (5-field format)
    let schedule = Schedule::parse("0 0 * * *")?;  // Midnight every day

    // Get the current time
    let now = Utc::now();

    // Get the next activation time
    let next = schedule.next(&now)?;
    println!("Next activation: {}", next);

    // Get the previous activation time
    let prev = schedule.prev(&now)?;
    println!("Previous activation: {}", prev);

    Ok(())
}
```

### Custom Parse Options

You can specify custom parse options to control how cron expressions are interpreted:

```rust
use cron_sdk::{Schedule, ParseOptions, options};
use chrono::Utc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a cron expression with seconds field (6-field format)
    let schedule = Schedule::parse_with_options(
        "0 0 0 * * *",  // Midnight every day with seconds
        Some(ParseOptions::new(options::STANDARD_WITH_SECONDS))
    )?;

    let now = Utc::now();
    let next = schedule.next(&now)?;
    println!("Next activation: {}", next);

    Ok(())
}
```

Available options:

- `options::SECOND` - Include seconds field (default 0)
- `options::SECOND_OPTIONAL` - Make seconds field optional
- `options::MINUTE` - Include minutes field (default 0)
- `options::HOUR` - Include hours field (default 0)
- `options::DOM` - Include day of month field (default *)
- `options::MONTH` - Include month field (default *)
- `options::DOW` - Include day of week field (default *)
- `options::DOW_OPTIONAL` - Make day of week field optional
- `options::DESCRIPTOR` - Allow descriptors like @monthly, @weekly, etc.

Predefined combinations:

- `options::STANDARD` - Standard 5-field format (minute, hour, day of month, month, day of week)
- `options::STANDARD_WITH_SECONDS` - 6-field format with seconds

## Cron Expression Format

By default, the cron expressions follow the standard format with five required fields:

```
┌───────────── minute (0 - 59)
│ ┌───────────── hour (0 - 23)
│ │ ┌───────────── day of the month (1 - 31)
│ │ │ ┌───────────── month (1 - 12)
│ │ │ │ ┌───────────── day of the week (0 - 6) (Sunday to Saturday)
│ │ │ │ │
│ │ │ │ │
* * * * *
```

With custom parse options, you can use different formats, such as including a seconds field:

```
┌───────────── second (0 - 59)
│ ┌───────────── minute (0 - 59)
│ │ ┌───────────── hour (0 - 23)
│ │ │ ┌───────────── day of the month (1 - 31)
│ │ │ │ ┌───────────── month (1 - 12)
│ │ │ │ │ ┌───────────── day of the week (0 - 6) (Sunday to Saturday)
│ │ │ │ │ │
│ │ │ │ │ │
* * * * * *
```

Additionally, this library supports:

- Special characters: `*`, `,`, `-`, `/`, `L`
- Predefined schedules: `@yearly`, `@monthly`, `@weekly`, `@daily`, `@hourly`, `@every <duration>` (when using the `options::DESCRIPTOR` option)

## How It Works

This SDK uses Extism to run a Go plugin that exposes the functionality of the Go cron library. The Rust SDK loads this plugin and provides a clean API to interact with it.

The WASM plugin is embedded directly into the SDK as a binary resource, so you don't need to build or distribute the plugin separately. When the SDK is used, it will automatically extract the plugin to a temporary file if needed. You can also specify a custom plugin path by setting the `CRON_PLUGIN_PATH` environment variable.

Unlike the previous implementation that used JavaScript WASM, this version uses Extism which is designed for language interoperability in service contexts rather than web browsers.

## Benchmarks

This SDK includes criterion benchmarks to measure performance. The benchmarks cover:

- Parsing cron expressions with different formats and complexities
- Getting next activation times for different schedule types
- Getting previous activation times for different schedule types
- Combined operations (parsing and getting next/prev times)

To run the benchmarks:

```bash
cargo bench
```

This will run all benchmarks and generate HTML reports in the `target/criterion` directory.

## Building from Source

To build this SDK from source, you need:

1. Go 1.20 or later (with WASI support)
2. Rust 1.56 or later
3. Extism SDK

Clone the repository and build:

```bash
git clone https://github.com/yourusername/cron-sdk
cd cron-sdk
cargo build
```

## License

MIT
