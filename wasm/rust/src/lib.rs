//! Rust SDK for cron scheduling using Go via Extism
//!
//! This crate provides a Rust interface to the Go cron library via Extism.
//! It allows parsing cron expressions and calculating next and previous schedule times.

use chrono::{DateTime, TimeZone, Utc};
use extism::{Manifest, Plugin, Wasm};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, Once};
use thiserror::Error;
use crate::options::STANDARD;

// Static initialization
static INIT: Once = Once::new();
static mut PLUGIN_INSTANCE: Option<Arc<Mutex<Plugin>>> = None;

/// Errors that can occur in the cron SDK
#[derive(Error, Debug)]
pub enum CronError {
    #[error("Failed to initialize plugin: {0}")]
    PluginInitError(String),

    #[error("Failed to parse cron expression: {0}")]
    ParseError(String),

    #[error("Plugin execution error: {0}")]
    PluginExecutionError(String),

    #[error("Invalid response from plugin: {0}")]
    InvalidResponse(String),
}

/// Result type for cron operations
pub type Result<T> = std::result::Result<T, CronError>;

/// Parse options for cron expressions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseOptions(i32);

impl ParseOptions {
    /// Create a new ParseOptions with the given options
    pub fn new(options: i32) -> Self {
        ParseOptions(options)
    }

    /// Get the raw options value
    pub fn value(&self) -> i32 {
        self.0
    }
}

/// Parse option constants
pub mod options {
    /// Seconds field, default 0
    pub const SECOND: i32 = 1 << 0;
    /// Optional seconds field, default 0
    pub const SECOND_OPTIONAL: i32 = 1 << 1;
    /// Minutes field, default 0
    pub const MINUTE: i32 = 1 << 2;
    /// Hours field, default 0
    pub const HOUR: i32 = 1 << 3;
    /// Day of month field, default *
    pub const DOM: i32 = 1 << 4;
    /// Month field, default *
    pub const MONTH: i32 = 1 << 5;
    /// Day of week field, default *
    pub const DOW: i32 = 1 << 6;
    /// Optional day of week field, default *
    pub const DOW_OPTIONAL: i32 = 1 << 7;
    /// Allow descriptors such as @monthly, @weekly, etc.
    pub const DESCRIPTOR: i32 = 1 << 8;

    /// Standard 5-field format (minute, hour, day of month, month, day of week)
    pub const STANDARD: i32 = MINUTE | HOUR | DOM | MONTH | DOW | DESCRIPTOR;
    /// Standard 6-field format with seconds (second, minute, hour, day of month, month, day of week)
    pub const STANDARD_WITH_SECONDS: i32 = SECOND | MINUTE | HOUR | DOM | MONTH | DOW | DESCRIPTOR;
}

/// Request structure for parsing a cron expression
#[derive(Serialize, Deserialize)]
struct ParseRequest {
    expression: String,
    options: i32,
}

/// Request structure for getting next/prev times
#[derive(Serialize, Deserialize)]
struct TimeRequest {
    expression: String,
    timestamp: i64,
    options: i32,
}

/// Response structure from the plugin
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    success: Option<bool>,
    error: Option<String>,
    timestamp: Option<i64>,
}

/// A parsed cron schedule
#[derive(Debug, Clone)]
pub struct Schedule {
    expression: String,
    options: ParseOptions,
}

impl Schedule {
    /// Parse a cron expression into a Schedule with default options
    pub fn parse(expression: &str) -> Result<Self> {
        Self::parse_with_options(expression, ParseOptions::new(STANDARD))
    }

    /// Parse a cron expression into a Schedule with custom options
    pub fn parse_with_options(expression: &str, options: ParseOptions) -> Result<Self> {
        // Initialize plugin if not already done
        ensure_plugin_initialized()?;

        // Create the request
        let request = ParseRequest {
            expression: expression.to_string(),
            options: options.value(),
        };

        // Call the plugin function to parse the schedule
        let result = call_plugin_function("parse_schedule", &request)?;

        // Check for errors
        if let Some(error) = result.error {
            return Err(CronError::ParseError(error));
        }

        Ok(Schedule {
            expression: expression.to_string(),
            options,
        })
    }

    /// Get the next time after the given time
    pub fn next<Tz: TimeZone>(&self, after: &DateTime<Tz>) -> Result<DateTime<Utc>> {
        // Convert to Unix timestamp
        let timestamp = after.timestamp();

        // Create the request
        let request = TimeRequest {
            expression: self.expression.clone(),
            timestamp,
            options: self.options.value(),
        };

        // Call the plugin function to get the next time
        let result = call_plugin_function("get_next_time", &request)?;

        // Check for errors
        if let Some(error) = result.error {
            return Err(CronError::PluginExecutionError(error));
        }

        // Extract the timestamp from the result
        let next_timestamp = result.timestamp
            .ok_or_else(|| CronError::InvalidResponse("Missing timestamp in response".to_string()))?;

        // Convert back to DateTime
        Ok(Utc.timestamp_opt(next_timestamp, 0)
            .single()
            .ok_or_else(|| CronError::InvalidResponse("Invalid timestamp".to_string()))?)
    }

    /// Get the previous time before the given time
    pub fn prev<Tz: TimeZone>(&self, before: &DateTime<Tz>) -> Result<DateTime<Utc>> {
        // Convert to Unix timestamp
        let timestamp = before.timestamp();

        // Create the request
        let request = TimeRequest {
            expression: self.expression.clone(),
            timestamp,
            options: self.options.value(),
        };

        // Call the plugin function to get the previous time
        let result = call_plugin_function("get_prev_time", &request)?;

        // Check for errors
        if let Some(error) = result.error {
            return Err(CronError::PluginExecutionError(error));
        }

        // Extract the timestamp from the result
        let prev_timestamp = result.timestamp
            .ok_or_else(|| CronError::InvalidResponse("Missing timestamp in response".to_string()))?;

        // Convert back to DateTime
        Ok(Utc.timestamp_opt(prev_timestamp, 0)
            .single()
            .ok_or_else(|| CronError::InvalidResponse("Invalid timestamp".to_string()))?)
    }

    /// Get the cron expression
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// Get the parse options used for this schedule
    pub fn options(&self) -> ParseOptions {
        self.options
    }
}

/// Initialize the plugin
fn ensure_plugin_initialized() -> Result<()> {
    INIT.call_once(|| {
        // Initialize the plugin
        match init_plugin() {
            Ok(plugin) => {
                unsafe {
                    PLUGIN_INSTANCE = Some(Arc::new(Mutex::new(plugin)));
                }
            }
            Err(e) => {
                eprintln!("Failed to initialize plugin: {}", e);
            }
        }
    });

    // Check if initialization was successful
    unsafe {
        if PLUGIN_INSTANCE.is_some() {
            Ok(())
        } else {
            Err(CronError::PluginInitError("Plugin initialization failed".to_string()))
        }
    }
}

/// Initialize the plugin
fn init_plugin() -> Result<Plugin> {
    // Get the path to the plugin
    let plugin_path = std::env::var("CRON_PLUGIN_PATH")
        .map_err(|_| CronError::PluginInitError("CRON_PLUGIN_PATH not set".to_string()))?;

    // Create a context
    // Load the plugin
    println!("{}", plugin_path);
    let wasm = Wasm::file(plugin_path);
    let manifest = Manifest::new([wasm]);
    let plugin = Plugin::new(&manifest, [], true)
        .map_err(|e| CronError::PluginInitError(format!("Failed to load plugin: {}", e)))?;

    Ok(plugin)
}

/// Call a function in the plugin
fn call_plugin_function<T: Serialize>(name: &str, request: &T) -> Result<Response> {
    // Get the plugin instance
    let plugin = unsafe {
        PLUGIN_INSTANCE.as_ref()
            .ok_or_else(|| CronError::PluginInitError("Plugin not initialized".to_string()))?
            .clone()
    };

    // Serialize the request
    let input = serde_json::to_vec(request)
        .map_err(|e| CronError::PluginExecutionError(format!("Failed to serialize request: {}", e)))?;

    // Call the function
    let output: Vec<u8> = {
        let mut plugin = plugin.lock().unwrap();
        plugin.call(name, input)
            .map_err(|e| CronError::PluginExecutionError(format!("Failed to call {}: {}", name, e)))?
    };

    // Deserialize the response
    let response: Response = serde_json::from_slice(&output)
        .map_err(|e| CronError::InvalidResponse(format!("Failed to deserialize response: {}", e)))?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_parse_schedule() {
        let schedule = Schedule::parse("0 0 * * *").expect("Failed to parse schedule");
        assert_eq!(schedule.expression(), "0 0 * * *");
    }

    #[test]
    fn test_parse_schedule_with_options() {
        // Test with standard options
        let schedule = Schedule::parse_with_options(
            "0 0 * * *", 
            ParseOptions::new(options::STANDARD)
        ).expect("Failed to parse schedule with standard options");
        assert_eq!(schedule.expression(), "0 0 * * *");

        // Test with seconds field
        let schedule = Schedule::parse_with_options(
            "0 0 0 * * *", 
            ParseOptions::new(options::STANDARD_WITH_SECONDS)
        ).expect("Failed to parse schedule with seconds");
        assert_eq!(schedule.expression(), "0 0 0 * * *");
    }

    #[test]
    fn test_next_prev() {
        let schedule = Schedule::parse("0 0 * * *").expect("Failed to parse schedule");
        let now = Utc::now();

        let next = schedule.next(&now).expect("Failed to get next time");
        assert!(next > now);
        println!("Now: {}", now);
        println!("Next time: {}", next);

        let prev = schedule.prev(&now).expect("Failed to get previous time");
        assert!(prev < now);
        println!("Now: {}", now);
        println!("Next time: {}", next);
    }

    #[test]
    fn test_next_prev_with_options() {
        // Test with seconds field
        let schedule = Schedule::parse_with_options(
            "0 0 0 * * *", 
            ParseOptions::new(options::STANDARD_WITH_SECONDS)
        ).expect("Failed to parse schedule with seconds");

        let now = Utc::now();

        let next = schedule.next(&now).expect("Failed to get next time");
        assert!(next > now);
        println!("Now: {}", now);
        println!("Next time: {}", next);

        let prev = schedule.prev(&now).expect("Failed to get previous time");
        assert!(prev < now);
        println!("Now: {}", now);
        println!("Next time: {}", next);
    }
}
