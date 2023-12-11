use std::time::{Duration, Instant};

use clap::Parser;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::{
    benchmark::{measure, DurationFormatter},
    puzzle::Puzzle,
};

mod benchmark;
mod puzzle;

#[derive(Parser)]
struct Args {
    /// Optional puzzle to run
    puzzle: Option<u32>,
    /// Optional part to run
    #[arg(short, long)]
    part: Option<u32>,
    /// Benchmarking rounds
    #[arg(short = 'r', long = "rounds", default_value_t = 1)]
    rounds: u32,
}

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
        Puzzle::new(
            0,
            "inputs/template.txt",
            measure(aoc2023_template::part1, rounds),
            measure(aoc2023_template::part2, rounds),
        ),
        Puzzle::new(
            1,
            "inputs/day-01.txt",
            measure(aoc2023_01::part1, rounds),
            measure(aoc2023_01::part2, rounds),
        ),
        Puzzle::new(
            2,
            "inputs/day-02.txt",
            measure(aoc2023_02::part1, rounds),
            measure(aoc2023_02::part2, rounds),
        ),
        Puzzle::new(
            3,
            "inputs/day-03.txt",
            measure(aoc2023_03::part1, rounds),
            measure(aoc2023_03::part2, rounds),
        ),
        Puzzle::new(
            4,
            "inputs/day-04.txt",
            measure(aoc2023_04::part1, rounds),
            measure(aoc2023_04::part2, rounds),
        ),
        Puzzle::new(
            5,
            "inputs/day-05.txt",
            measure(aoc2023_05::part1, rounds),
            measure(aoc2023_05::part2, rounds),
        ),
        Puzzle::new(
            6,
            "inputs/day-06.txt",
            measure(aoc2023_06::part1, rounds),
            measure(aoc2023_06::part2, rounds),
        ),
        Puzzle::new(
            7,
            "inputs/day-07.txt",
            measure(aoc2023_07::part1, rounds),
            measure(aoc2023_07::part2, rounds),
        ),
        Puzzle::new(
            8,
            "inputs/day-08.txt",
            measure(aoc2023_08::part1, rounds),
            measure(aoc2023_08::part2, rounds),
        ),
        Puzzle::new(
            9,
            "inputs/day-09.txt",
            measure(aoc2023_09::part1, rounds),
            measure(aoc2023_09::part2, rounds),
        ),
        Puzzle::new(
            10,
            "inputs/day-10.txt",
            measure(aoc2023_10::part1, rounds),
            measure(aoc2023_10::part2, rounds),
        ),
        Puzzle::new(
            11,
            "inputs/day-11.txt",
            measure(aoc2023_11::part1, rounds),
            measure(aoc2023_11::part2, rounds),
        ),
    ];

    let start = Instant::now();
    let sum_of_means = if let Some(puzzle) = args.puzzle {
        run_one(puzzle, &puzzles, parts)?
    } else {
        run_all(&puzzles, parts)?
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
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

fn run_all(puzzles: &[Puzzle], parts: [bool; 2]) -> Result<Duration, AocError> {
    puzzles[1..].iter().map(|p| p.run(parts)).sum()
}

fn run_one(puzzle: u32, puzzles: &[Puzzle], parts: [bool; 2]) -> Result<Duration, AocError> {
    let puzzle = puzzles
        .get(puzzle as usize)
        .ok_or(AocError::NoSuchPuzzle { puzzle })?;

    puzzle.run(parts)
}

pub fn trace() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("AOC2023_LOG"))
        .init();
}
