# cron

This is a straight fork from https://github.com/robfig/cron with (currently) two additional PRs merged:
* [Get the previous schedule time](https://github.com/robfig/cron/pull/361)
* [Support for 'L' expression](https://github.com/robfig/cron/pull/325)

I'll try my best to keep this up to date, tested and merge additional PRs moving forward.

## Rust SDK

A Rust SDK is available in the [wasm](./wasm) directory. It wraps this Go library using Extism (a WebAssembly-based plugin system) to provide cron parsing and scheduling functionality in Rust.

This SDK is designed for use in Rust services that need to call Go code, using Extism as the bridge between the languages.

### Basic Usage

```rust
use cron_sdk::Schedule;
use chrono::Utc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a cron expression
    let schedule = Schedule::parse("0 0 * * *")?;  // Midnight every day

    // Get the current time
    let now = Utc::now();

    // Get the next activation time
    let next = schedule.next(&now)?;
    println!("Next activation: {}", next);

    Ok(())
}
```

See the [Rust SDK README](./wasm/rust/README.md) for more details, including custom parse options, benchmarks, and advanced usage examples.
