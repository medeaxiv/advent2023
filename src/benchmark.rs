use std::time::{Duration, Instant};

#[allow(clippy::type_complexity)]
pub fn measure<R>(
    function: impl Fn(&str) -> R + 'static,
    rounds: u32,
) -> Box<dyn Fn(&str) -> (RuntimeStats, String)>
where
    R: std::fmt::Display,
{
    if rounds <= 1 {
        Box::new(move |input| {
            let start = Instant::now();
            let result = function(input);
            let duration = start.elapsed();
            (duration.into(), result.to_string())
        })
    } else {
        Box::new(move |input| {
            let mut accumulator = Vec::with_capacity(rounds as usize);

            let mut result = None;

            for _ in 0..rounds {
                let start = Instant::now();
                let round_result = function(input);
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
    Single([Duration; 1]),
    Multiple {
        runs: Vec<Duration>,
        median: Duration,
        min: Duration,
        max: Duration,
    },
}

impl RuntimeStats {
    pub fn runs(&self) -> &[Duration] {
        match self {
            Self::Single(duration) => duration.as_slice(),
            Self::Multiple { runs, .. } => runs.as_slice(),
        }
    }

    pub fn median(&self) -> Duration {
        match self {
            Self::Single([duration]) => *duration,
            Self::Multiple { median, .. } => *median,
        }
    }
}

impl From<Duration> for RuntimeStats {
    fn from(value: Duration) -> Self {
        Self::Single([value])
    }
}

impl From<Vec<Duration>> for RuntimeStats {
    fn from(value: Vec<Duration>) -> Self {
        assert!(!value.is_empty());

        let mut sorted = value.clone();
        sorted.sort();

        let median = median(sorted.as_slice());
        let min = sorted[0];
        let max = sorted[sorted.len() - 1];

        Self::Multiple {
            runs: value,
            median,
            min,
            max,
        }
    }
}

impl std::fmt::Display for RuntimeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single([duration]) => write!(f, "{}", DurationFormatter(*duration)),
            Self::Multiple {
                min, max, median, ..
            } => {
                write!(
                    f,
                    "median: {}, min: {}, max: {}",
                    DurationFormatter(*median),
                    DurationFormatter(*min),
                    DurationFormatter(*max)
                )
            }
        }
    }
}

fn median(slice: &[Duration]) -> Duration {
    assert!(!slice.is_empty());

    let middle = slice.len() / 2;
    if slice.len() % 2 == 0 {
        (slice[middle - 1] + slice[middle]) / 2
    } else {
        slice[middle]
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
