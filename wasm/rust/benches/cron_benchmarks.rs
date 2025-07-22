use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cron_sdk::{ParseOptions, Schedule, options};
use chrono::Utc;

fn bench_parse_schedule(c: &mut Criterion) {
    c.bench_function("parse_schedule_standard", |b| {
        b.iter(|| {
            let _ = Schedule::parse(black_box("0 0 * * *")).unwrap();
        });
    });

    c.bench_function("parse_schedule_with_seconds", |b| {
        b.iter(|| {
            let _ = Schedule::parse_with_options(
                black_box("0 0 0 * * *"),
                ParseOptions::new(options::STANDARD_WITH_SECONDS)
            ).unwrap();
        });
    });

    // Benchmark parsing different types of expressions
    c.bench_function("parse_schedule_complex", |b| {
        b.iter(|| {
            let _ = Schedule::parse(black_box("*/15 */2 1-7 1,6,12 *")).unwrap();
        });
    });

    c.bench_function("parse_schedule_descriptor", |b| {
        b.iter(|| {
            let _ = Schedule::parse(black_box("@daily")).unwrap();
        });
    });
}

fn bench_next_time(c: &mut Criterion) {
    let now = Utc::now();
    
    // Simple schedule (every day at midnight)
    let simple_schedule = Schedule::parse("0 0 * * *").unwrap();
    c.bench_function("next_time_simple", |b| {
        b.iter(|| {
            let _ = simple_schedule.next(black_box(&now)).unwrap();
        });
    });
    
    // Complex schedule (every 15 minutes on weekdays)
    let complex_schedule = Schedule::parse("*/15 * * * 1-5").unwrap();
    c.bench_function("next_time_complex", |b| {
        b.iter(|| {
            let _ = complex_schedule.next(black_box(&now)).unwrap();
        });
    });
    
    // With seconds precision
    let seconds_schedule = Schedule::parse_with_options(
        "0 */5 * * * *",
        ParseOptions::new(options::STANDARD_WITH_SECONDS)
    ).unwrap();
    c.bench_function("next_time_with_seconds", |b| {
        b.iter(|| {
            let _ = seconds_schedule.next(black_box(&now)).unwrap();
        });
    });
}

fn bench_prev_time(c: &mut Criterion) {
    let now = Utc::now();
    
    // Simple schedule (every day at midnight)
    let simple_schedule = Schedule::parse("0 0 * * *").unwrap();
    c.bench_function("prev_time_simple", |b| {
        b.iter(|| {
            let _ = simple_schedule.prev(black_box(&now)).unwrap();
        });
    });
    
    // Complex schedule (every 15 minutes on weekdays)
    let complex_schedule = Schedule::parse("*/15 * * * 1-5").unwrap();
    c.bench_function("prev_time_complex", |b| {
        b.iter(|| {
            let _ = complex_schedule.prev(black_box(&now)).unwrap();
        });
    });
    
    // With seconds precision
    let seconds_schedule = Schedule::parse_with_options(
        "0 */5 * * * *",
        ParseOptions::new(options::STANDARD_WITH_SECONDS)
    ).unwrap();
    c.bench_function("prev_time_with_seconds", |b| {
        b.iter(|| {
            let _ = seconds_schedule.prev(black_box(&now)).unwrap();
        });
    });
}

fn bench_multiple_operations(c: &mut Criterion) {
    c.bench_function("parse_and_next", |b| {
        b.iter(|| {
            let schedule = Schedule::parse(black_box("0 0 * * *")).unwrap();
            let now = Utc::now();
            let _ = schedule.next(black_box(&now)).unwrap();
        });
    });
    
    c.bench_function("parse_and_prev", |b| {
        b.iter(|| {
            let schedule = Schedule::parse(black_box("0 0 * * *")).unwrap();
            let now = Utc::now();
            let _ = schedule.prev(black_box(&now)).unwrap();
        });
    });
    
    c.bench_function("next_5_times", |b| {
        b.iter(|| {
            let schedule = Schedule::parse(black_box("0 0 * * *")).unwrap();
            let mut time = Utc::now();
            for _ in 0..5 {
                time = schedule.next(black_box(&time)).unwrap();
            }
        });
    });
}

criterion_group!(
    benches,
    bench_parse_schedule,
    bench_next_time,
    bench_prev_time,
    bench_multiple_operations
);
criterion_main!(benches);