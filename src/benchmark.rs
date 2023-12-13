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
    Single(Duration),
    Multiple {
        runs: Vec<Duration>,
        min: Duration,
        max: Duration,
        median: Duration,
        mean: Duration,
        standard_deviation: Duration,
    },
}

impl RuntimeStats {
    pub fn mean(&self) -> Duration {
        match self {
            Self::Single(duration) => *duration,
            Self::Multiple { mean, .. } => *mean,
        }
    }
}

impl From<Duration> for RuntimeStats {
    fn from(value: Duration) -> Self {
        Self::Single(value)
    }
}

impl From<Vec<Duration>> for RuntimeStats {
    fn from(mut value: Vec<Duration>) -> Self {
        assert!(!value.is_empty());

        value.sort();

        let median = if value.len() % 2 == 0 {
            let middle = value.len() / 2;
            (value[middle - 1] + value[middle]) / 2
        } else {
            value[value.len() / 2]
        };

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
            median,
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
