use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use benchmark::RuntimeStats;
use clap::Parser;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::{
    benchmark::{measure, DurationFormatter},
    puzzle::Puzzle,
    report::Report,
};

mod benchmark;
mod puzzle;
mod report;

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
    /// Optional benchmark report output location
    #[arg(short = 'o', long = "out", id = "PATH")]
    report: Option<PathBuf>,
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
        Puzzle::new(
            12,
            "inputs/day-12.txt",
            measure(aoc2023_12::part1, rounds),
            measure(aoc2023_12::part2, rounds),
        ),
        Puzzle::new(
            13,
            "inputs/day-13.txt",
            measure(aoc2023_13::part1, rounds),
            measure(aoc2023_13::part2, rounds),
        ),
        Puzzle::new(
            14,
            "inputs/day-14.txt",
            measure(aoc2023_14::part1, rounds),
            measure(aoc2023_14::part2, rounds),
        ),
        Puzzle::new(
            15,
            "inputs/day-15.txt",
            measure(aoc2023_15::part1, rounds),
            measure(aoc2023_15::part2, rounds),
        ),
        Puzzle::new(
            16,
            "inputs/day-16.txt",
            measure(aoc2023_16::part1, rounds),
            measure(aoc2023_16::part2, rounds),
        ),
        Puzzle::new(
            17,
            "inputs/day-17.txt",
            measure(aoc2023_17::part1, rounds),
            measure(aoc2023_17::part2, rounds),
        ),
        Puzzle::new(
            18,
            "inputs/day-18.txt",
            measure(aoc2023_18::part1, rounds),
            measure(aoc2023_18::part2, rounds),
        ),
        Puzzle::new(
            19,
            "inputs/day-19.txt",
            measure(aoc2023_19::part1, rounds),
            measure(aoc2023_19::part2, rounds),
        ),
        Puzzle::new(
            20,
            "inputs/day-20.txt",
            measure(aoc2023_20::part1, rounds),
            measure(aoc2023_20::part2, rounds),
        ),
        Puzzle::new(
            21,
            "inputs/day-21.txt",
            measure(aoc2023_21::part1, rounds),
            measure(aoc2023_21::part2, rounds),
        ),
    ];

    let start = Instant::now();

    let mut report = args.report.as_ref().map(|_| Report::default());

    let mut sum_of_medians = Duration::ZERO;
    let visitor = |puzzle, part, stats: RuntimeStats, result| {
        println!("Day {puzzle:02} part {part} ({stats}): {result}");
        sum_of_medians += stats.median();

        if let Some(report) = report.as_mut() {
            report.push_entry(puzzle, part, &stats);
        }

        Ok(())
    };

    if let Some(puzzle) = args.puzzle {
        run_one(puzzle, &puzzles, parts, visitor)?;
    } else {
        run_all(&puzzles, parts, visitor)?;
    }

    let total = start.elapsed();

    if rounds > 1 {
        println!(
            "Sum of median solve times: {}",
            DurationFormatter(sum_of_medians),
        );
    } else {
        println!("Sum of solve times: {}", DurationFormatter(sum_of_medians),);
    }

    println!("Total time: {}", DurationFormatter(total));

    if let Some(report) = report {
        report.save_to(args.report.unwrap())?;
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("No puzzle {puzzle}")]
    NoSuchPuzzle { puzzle: u32 },
    #[error(transparent)]
    Report(csv::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<csv::Error> for AocError {
    fn from(value: csv::Error) -> Self {
        match value.kind() {
            csv::ErrorKind::Io(_) => {
                let io = match value.into_kind() {
                    csv::ErrorKind::Io(io) => io,
                    _ => unreachable!(),
                };

                Self::Io(io)
            }
            _ => Self::Report(value),
        }
    }
}

fn run_all(
    puzzles: &[Puzzle],
    parts: [bool; 2],
    mut visitor: impl FnMut(u32, u32, RuntimeStats, String) -> Result<(), AocError>,
) -> Result<(), AocError> {
    for puzzle in puzzles[1..].iter() {
        puzzle.run(parts, &mut visitor)?;
    }

    Ok(())
}

fn run_one(
    puzzle: u32,
    puzzles: &[Puzzle],
    parts: [bool; 2],
    visitor: impl FnMut(u32, u32, RuntimeStats, String) -> Result<(), AocError>,
) -> Result<(), AocError> {
    let puzzle = puzzles
        .get(puzzle as usize)
        .ok_or(AocError::NoSuchPuzzle { puzzle })?;

    puzzle.run(parts, visitor)
}

pub fn trace() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("AOC2023_LOG"))
        .init();
}
