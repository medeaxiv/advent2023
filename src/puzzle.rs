use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use crate::{benchmark::RuntimeStats, AocError};

#[allow(clippy::type_complexity)]
pub struct Puzzle {
    puzzle: u32,
    input_file: PathBuf,
    p1: Box<dyn Fn(&str) -> (RuntimeStats, String)>,
    p2: Box<dyn Fn(&str) -> (RuntimeStats, String)>,
}

impl Puzzle {
    #[allow(clippy::type_complexity)]
    pub fn new(
        id: u32,
        input_file: impl AsRef<Path>,
        p1: Box<dyn Fn(&str) -> (RuntimeStats, String)>,
        p2: Box<dyn Fn(&str) -> (RuntimeStats, String)>,
    ) -> Self {
        Self {
            puzzle: id,
            input_file: input_file.as_ref().to_owned(),
            p1,
            p2,
        }
    }

    pub fn run(&self, parts: [bool; 2]) -> Result<Duration, AocError> {
        let input = std::fs::read_to_string(&self.input_file)?;

        let d1 = if parts[0] {
            self.run_part(1, input.as_str(), &self.p1)
        } else {
            Duration::ZERO
        };

        let d2 = if parts[1] {
            self.run_part(2, input.as_str(), &self.p2)
        } else {
            Duration::ZERO
        };

        Ok(d1 + d2)
    }

    fn run_part(
        &self,
        part: u32,
        input: &str,
        func: &dyn Fn(&str) -> (RuntimeStats, String),
    ) -> Duration {
        let (stats, r) = func(input);

        println!("Day {:02} part {part} ({}): {r}", self.puzzle, stats);

        stats.mean()
    }
}
