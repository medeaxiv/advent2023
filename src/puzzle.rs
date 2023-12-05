use std::time::Duration;

use crate::benchmark::RuntimeStats;

pub struct Puzzle {
    puzzle: u32,
    p1: Box<dyn Fn() -> (RuntimeStats, String)>,
    p2: Box<dyn Fn() -> (RuntimeStats, String)>,
}

impl Puzzle {
    pub fn new(
        id: u32,
        p1: Box<dyn Fn() -> (RuntimeStats, String)>,
        p2: Box<dyn Fn() -> (RuntimeStats, String)>,
    ) -> Self {
        Self { puzzle: id, p1, p2 }
    }

    pub fn run(&self, parts: [bool; 2]) -> Duration {
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
