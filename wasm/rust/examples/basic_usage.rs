use cron_sdk::{ParseOptions, Schedule};
use chrono::{Utc};
use cron_sdk::options::STANDARD_WITH_SECONDS;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a cron expression
    println!("Parsing cron expression: '0 */5 * * * *' (every 5 minutes)");
    let schedule = Schedule::parse_with_options("0 */5 * * * *", ParseOptions::new(STANDARD_WITH_SECONDS))?;

    // Get the current time
    let now = Utc::now();
    println!("Current time: {}", now);

    // Get the next activation time
    let next = schedule.next(&now)?;
    println!("Next activation: {}", next);
    println!("Time until next activation: {} seconds", (next - now).num_seconds());

    // Get the previous activation time
    let prev = schedule.prev(&now)?;
    println!("Previous activation: {}", prev);
    println!("Time since previous activation: {} seconds", (now - prev).num_seconds());

    // Calculate multiple future activations
    println!("\nNext 5 activations:");
    let mut time = now;
    for i in 1..=5 {
        time = schedule.next(&time)?;
        println!("  {}. {}", i, time);
    }

    // Calculate multiple past activations
    println!("\nPrevious 5 activations:");
    let mut time = now;
    for i in 1..=5 {
        time = schedule.prev(&time)?;
        println!("  {}. {}", i, time);
    }

    // Different cron expressions
    println!("\nExamples of different cron expressions:");

    // Daily at midnight
    let daily = Schedule::parse_with_options("0 */5 * * * *", ParseOptions::new(STANDARD_WITH_SECONDS))?;
    println!("Daily at midnight: Next = {}", daily.next(&now)?);

    // Weekdays at 9am
    let weekday = Schedule::parse_with_options("0 */5 * * * *", ParseOptions::new(STANDARD_WITH_SECONDS))?;
    println!("Weekdays at 9am: Next = {}", weekday.next(&now)?);

    // Every 15 minutes
    let every15min = Schedule::parse_with_options("0 */5 * * * *", ParseOptions::new(STANDARD_WITH_SECONDS))?;
    println!("Every 15 minutes: Next = {}", every15min.next(&now)?);

    // Monthly on the 1st at midnight
    let monthly = Schedule::parse_with_options("0 */5 * * * *", ParseOptions::new(STANDARD_WITH_SECONDS))?;
    println!("Monthly on the 1st: Next = {}", monthly.next(&now)?);

    Ok(())
}
