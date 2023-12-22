use tracing_subscriber::{
    fmt::{self, TestWriter},
    prelude::*,
    EnvFilter,
};

pub fn setup_tracing() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(TestWriter::new()))
        .with(EnvFilter::from_env("AOC2023_LOG"))
        .init();
}
