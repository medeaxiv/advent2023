use std::time::{Duration, Instant};

use clap::Parser;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() -> Result<(), String> {
    let args = Args::parse();

    if let Some(puzzle) = args.puzzle {
        run_one(puzzle)
    } else {
        run_all();
        Ok(())
    }
}

fn run_all() {
    let total_time: Duration = (1..=25).map_while(run_puzzle).sum();
    println!("Total solve time: {}", DurationFormatter(total_time));
}

fn run_one(puzzle: u32) -> Result<(), String> {
    let total_time = run_puzzle(puzzle).ok_or_else(|| format!("No puzzle {puzzle}"))?;
    println!("Total solve time: {}", DurationFormatter(total_time));

    Ok(())
}

fn run_puzzle(puzzle: u32) -> Option<Duration> {
    match puzzle {
        0 => Some(call_puzzle_parts(
            puzzle,
            aoc2023_template::part1,
            aoc2023_template::part2,
        )),
        1 => Some(call_puzzle_parts(
            puzzle,
            aoc2023_01::part1,
            aoc2023_01::part2,
        )),
        _ => None,
    }
}

fn call_puzzle_parts<R1, R2, S>(
    puzzle: u32,
    p1: impl FnOnce() -> (R1, S),
    p2: impl FnOnce(S) -> R2,
) -> Duration
where
    R1: std::fmt::Display,
    R2: std::fmt::Display,
{
    let (d1, (r1, shared)) = measure(p1);
    println!("Day {puzzle} Part 1 ({}): {r1}", DurationFormatter(d1));
    let (d2, r2) = measure(|| p2(shared));
    println!("Day {puzzle} Part 2 ({}): {r2}", DurationFormatter(d2));
    d1 + d2
}

#[derive(Parser)]
struct Args {
    /// Optional puzzle to run
    puzzle: Option<u32>,
}

pub fn trace() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("AOC2023_LOG"))
        .init();
}

pub fn measure<R>(function: impl FnOnce() -> R) -> (Duration, R) {
    let start = Instant::now();
    let result = function();
    let duration = start.elapsed();
    (duration, result)
}

pub struct DurationFormatter(pub Duration);

impl std::fmt::Display for DurationFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nanos = self.0.as_nanos();
        match nanos {
            n if n > 1_500_000_000 => {
                write!(f, "{:.3}s", self.0.as_secs_f64())
            }
            n if n > 1_500_000 => {
                let millis = n as f64 / 1_000_000.0;
                write!(f, "{:.1}ms", millis)
            }
            n if n > 1_500 => {
                let millis = n as f64 / 1_000.0;
                write!(f, "{:.1}μs", millis)
            }
            n => {
                write!(f, "{n}ns")
            }
        }
    }
}
