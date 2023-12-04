use std::time::{Duration, Instant};

use clap::Parser;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() -> anyhow::Result<()> {
    trace();

    let args = Args::parse();
    let parts = match args.part {
        Some(1) => [true, false],
        Some(2) => [false, true],
        None => [true, true],
        _ => [false, false],
    };

    let rounds = args.rounds;

    let puzzles = [
        Puzzle {
            puzzle: 0,
            p1: measure(aoc2023_template::part1, rounds),
            p2: measure(aoc2023_template::part2, rounds),
        },
        Puzzle {
            puzzle: 1,
            p1: measure(aoc2023_01::part1, rounds),
            p2: measure(aoc2023_01::part2, rounds),
        },
        Puzzle {
            puzzle: 2,
            p1: measure(aoc2023_02::part1, rounds),
            p2: measure(aoc2023_02::part2, rounds),
        },
        Puzzle {
            puzzle: 3,
            p1: measure(aoc2023_03::part1, rounds),
            p2: measure(aoc2023_03::part2, rounds),
        },
        Puzzle {
            puzzle: 4,
            p1: measure(aoc2023_04::part1, rounds),
            p2: measure(aoc2023_04::part2, rounds),
        },
    ];

    let start = Instant::now();
    let sum_of_means = if let Some(puzzle) = args.puzzle {
        run_one(puzzle, &puzzles, parts)?
    } else {
        run_all(&puzzles, parts)
    };
    let total = start.elapsed();

    println!(
        "Sum of mean solve times: {}",
        DurationFormatter(sum_of_means),
    );

    println!("Total time: {}", DurationFormatter(total));

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No puzzle {puzzle}")]
    NoSuchPuzzle { puzzle: u32 },
}

fn run_all(puzzles: &[Puzzle], parts: [bool; 2]) -> Duration {
    puzzles[1..].iter().map(|p| p.run(parts)).sum()
}

fn run_one(puzzle: u32, puzzles: &[Puzzle], parts: [bool; 2]) -> anyhow::Result<Duration> {
    let puzzle = puzzles
        .get(puzzle as usize)
        .ok_or(AocError::NoSuchPuzzle { puzzle })?;

    Ok(puzzle.run(parts))
}

#[derive(Parser)]
struct Args {
    /// Optional puzzle to run
    puzzle: Option<u32>,
    /// Optional part to run
    #[arg(short, long)]
    part: Option<u32>,
    #[arg(short = 'r', long = "rounds", default_value_t = 1)]
    rounds: u32,
}

struct Puzzle {
    puzzle: u32,
    p1: Box<dyn Fn() -> (RuntimeStats, String)>,
    p2: Box<dyn Fn() -> (RuntimeStats, String)>,
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

    fn run_part(&self, part: u32, func: &dyn Fn() -> (RuntimeStats, String)) -> Duration {
        let (stats, r) = func();

        println!("Day {:02} part {part} ({}): {r}", self.puzzle, stats);

        stats.mean()
    }
}

pub fn trace() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("AOC2023_LOG"))
        .init();
}

pub fn measure<R>(
    function: impl Fn() -> R + 'static,
    rounds: u32,
) -> Box<dyn Fn() -> (RuntimeStats, String)>
where
    R: std::fmt::Display,
{
    if rounds <= 1 {
        Box::new(move || {
            let start = Instant::now();
            let result = function();
            let duration = start.elapsed();
            (duration.into(), result.to_string())
        })
    } else {
        Box::new(move || {
            let mut accumulator = Vec::with_capacity(rounds as usize);

            let mut result = None;

            for _ in 0..rounds {
                let start = Instant::now();
                let round_result = function();
                let duration = start.elapsed();
                accumulator.push(duration);

                if result.is_none() {
                    result = Some(round_result);
                }
            }

            (accumulator.into(), result.unwrap().to_string())
        })
    }
}

pub enum RuntimeStats {
    Single(Duration),
    Multiple {
        runs: Vec<Duration>,
        min: Duration,
        max: Duration,
        mean: Duration,
        standard_deviation: Duration,
    },
}

impl RuntimeStats {
    pub fn min(&self) -> Duration {
        match self {
            Self::Single(duration) => *duration,
            Self::Multiple { min, .. } => *min,
        }
    }

    pub fn max(&self) -> Duration {
        match self {
            Self::Single(duration) => *duration,
            Self::Multiple { max, .. } => *max,
        }
    }

    pub fn mean(&self) -> Duration {
        match self {
            Self::Single(duration) => *duration,
            Self::Multiple { mean, .. } => *mean,
        }
    }

    pub fn standard_deviation(&self) -> Duration {
        match self {
            Self::Single(duration) => *duration,
            Self::Multiple {
                standard_deviation, ..
            } => *standard_deviation,
        }
    }
}

impl From<Duration> for RuntimeStats {
    fn from(value: Duration) -> Self {
        Self::Single(value)
    }
}

impl From<Vec<Duration>> for RuntimeStats {
    fn from(value: Vec<Duration>) -> Self {
        let mut iter = value.iter();
        let first = *iter.next().unwrap();
        let (min, max, total) = iter.fold((first, first, first), |(min, max, total), next| {
            (min.min(*next), max.max(*next), total + *next)
        });

        let mean = total / value.len() as u32;
        let mean_secs = mean.as_secs_f64();

        let variance = value
            .iter()
            .map(|duration| duration.as_secs_f64() - mean_secs)
            .map(|difference| difference * difference)
            .sum::<f64>()
            / value.len() as f64;

        let standard_deviation = Duration::from_secs_f64(variance.sqrt());

        Self::Multiple {
            runs: value,
            min,
            max,
            mean,
            standard_deviation,
        }
    }
}

impl std::fmt::Display for RuntimeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(duration) => write!(f, "{}", DurationFormatter(*duration)),
            Self::Multiple {
                min,
                max,
                mean,
                standard_deviation,
                ..
            } => {
                write!(
                    f,
                    "mean: {}, sd: {}, min: {}, max: {}",
                    DurationFormatter(*mean),
                    DurationFormatter(*standard_deviation),
                    DurationFormatter(*min),
                    DurationFormatter(*max)
                )
            }
        }
    }
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
