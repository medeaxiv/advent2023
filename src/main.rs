use std::time::{Duration, Instant};

use clap::Parser;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() -> Result<(), String> {
    trace();

    let args = Args::parse();
    let parts = match args.part {
        Some(1) => [true, false],
        Some(2) => [false, true],
        None => [true, true],
        _ => [false, false],
    };

    let puzzles = [
        Puzzle {
            puzzle: 0,
            p1: measure(aoc2023_template::part1),
            p2: measure(aoc2023_template::part2),
        },
        Puzzle {
            puzzle: 1,
            p1: measure(aoc2023_01::part1),
            p2: measure(aoc2023_01::part2),
        },
        Puzzle {
            puzzle: 2,
            p1: measure(aoc2023_02::part1),
            p2: measure(aoc2023_02::part2),
        },
        Puzzle {
            puzzle: 3,
            p1: measure(aoc2023_03::part1),
            p2: measure(aoc2023_03::part2),
        },
    ];

    if let Some(puzzle) = args.puzzle {
        run_one(puzzle, &puzzles, parts)
    } else {
        run_all(&puzzles, parts);
        Ok(())
    }
}

fn run_all(puzzles: &[Puzzle], parts: [bool; 2]) {
    let total_time: Duration = puzzles[1..].iter().map(|p| p.run(parts)).sum();
    println!("Total solve time: {}", DurationFormatter(total_time));
}

fn run_one(puzzle: u32, puzzles: &[Puzzle], parts: [bool; 2]) -> Result<(), String> {
    let puzzle = puzzles
        .get(puzzle as usize)
        .ok_or_else(|| format!("No puzzle {puzzle}"))?;

    let total_time = puzzle.run(parts);
    println!("Total solve time: {}", DurationFormatter(total_time));

    Ok(())
}

#[derive(Parser)]
struct Args {
    /// Optional puzzle to run
    puzzle: Option<u32>,
    /// Optional part to run
    #[arg(short, long)]
    part: Option<u32>,
}

struct Puzzle {
    puzzle: u32,
    p1: Box<dyn Fn() -> (Duration, String)>,
    p2: Box<dyn Fn() -> (Duration, String)>,
}

impl Puzzle {
    fn run(&self, parts: [bool; 2]) -> Duration {
        let d1 = if parts[0] {
            self.run_part(1, &self.p1)
        } else {
            Duration::ZERO
        };

        let d2 = if parts[1] {
            self.run_part(2, &self.p2)
        } else {
            Duration::ZERO
        };

        d1 + d2
    }

    fn run_part(&self, part: u32, func: &dyn Fn() -> (Duration, String)) -> Duration {
        let (d, r) = func();

        println!(
            "Day {:02} part {part} ({}): {r}",
            self.puzzle,
            DurationFormatter(d)
        );

        d
    }
}

pub fn trace() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("AOC2023_LOG"))
        .init();
}

pub fn measure<R>(function: impl Fn() -> R + 'static) -> Box<dyn Fn() -> (Duration, String)>
where
    R: std::fmt::Display,
{
    Box::new(move || {
        let start = Instant::now();
        let result = function();
        let duration = start.elapsed();
        (duration, result.to_string())
    })
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
