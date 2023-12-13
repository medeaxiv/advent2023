use std::path::{Path, PathBuf};

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

    pub fn run(
        &self,
        parts: [bool; 2],
        mut visitor: impl FnMut(u32, u32, RuntimeStats, String),
    ) -> Result<(), AocError> {
        let input = std::fs::read_to_string(&self.input_file)?;

        if parts[0] {
            let (stats, result) = (*self.p1)(input.as_str());
            visitor(self.puzzle, 1, stats, result);
        }

        if parts[1] {
            let (stats, result) = (*self.p2)(input.as_str());
            visitor(self.puzzle, 2, stats, result);
        }

        Ok(())
    }
}
