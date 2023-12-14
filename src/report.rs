use std::path::Path;

use itertools::Itertools;

use crate::{benchmark::RuntimeStats, AocError};

#[derive(Default)]
pub struct Report {
    entries: Vec<(u32, u32, Vec<f64>)>,
}

impl Report {
    pub fn push_entry(&mut self, puzzle: u32, part: u32, stats: &RuntimeStats) {
        self.entries.push((
            puzzle,
            part,
            stats
                .runs()
                .iter()
                .map(|run| run.as_nanos() as f64 / 1_000_000.0)
                .collect_vec(),
        ));
    }

    pub fn save_to(&self, path: impl AsRef<Path>) -> Result<(), AocError> {
        let mut writer = csv::Writer::from_path(path)?;

        for (puzzle, part, values) in self.entries.iter() {
            let mut record = csv::StringRecord::with_capacity(1024, values.len() + 2);
            record.push_field(format!("Day {puzzle}").as_str());
            record.push_field(format!("Part {part}").as_str());

            for value in values.iter() {
                record.push_field(format!("{value}").as_str());
            }

            writer.write_record(&record)?;
        }

        Ok(())
    }
}
