use std::{str::FromStr, time::Duration};

use clap::Parser;

use crate::error::JigglerError;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Delay the first jiggle
    #[arg(short, long, default_value = "0:seconds")]
    pub start: CustomDuration,

    /// How long to run, measured from the first jiggle
    #[arg(short, long, default_value = "max")]
    pub end: CustomDuration,

    /// The time to wait in between jiggles
    #[arg(short, long, default_value = "1:minute")]
    pub wait: CustomDuration,

    /// The max level of log messages to display
    #[arg(short, long, default_value_t = tracing::Level::INFO)]
    pub log_level: tracing::Level,

    /// The time to sleep in between repetitions of a single jiggle
    #[arg(long, default_value = "70:ms")]
    pub jiggle_sleep: CustomDuration,

    /// The number of "back-and-forths" in a single jiggle
    #[arg(long, default_value_t = 3)]
    pub jiggle_reps: usize,

    /// The number of pixels to move in a single jiggle
    #[arg(long, default_value_t = 10)]
    pub jiggle_size: u16,
}

enum DurationUnit {
    Hours,
    Minutes,
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
}

impl FromStr for DurationUnit {
    type Err = JigglerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hours" | "hour" | "hr" | "h" => Ok(DurationUnit::Hours),
            "minutes" | "minute" | "min" | "m" => Ok(DurationUnit::Minutes),
            "seconds" | "second" | "sec" | "s" => Ok(DurationUnit::Seconds),
            "milliseconds" | "millisecond" | "ms" => Ok(DurationUnit::Milliseconds),
            "microseconds" | "microsecond" | "us" => Ok(DurationUnit::Microseconds),
            "nanoseconds" | "nanosecond" | "ns" => Ok(DurationUnit::Nanoseconds),
            u => Err(JigglerError::DurationParseError(format!(
                "unrecognized duration unit {0}",
                u
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CustomDuration {
    duration: Duration,
}

impl FromStr for CustomDuration {
    type Err = JigglerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let options = s.split(':').collect::<Vec<_>>();
        let (value, unit) =
            if options.len() == 1 && options.first().unwrap().to_lowercase() == "max" {
                return Ok(CustomDuration {
                    duration: Duration::MAX,
                });
            } else if options.len() == 2 {
                let value: u64 = options.first().unwrap().parse()?;
                let unit = options.last().unwrap().parse()?;

                Ok((value, unit))
            } else {
                Err(JigglerError::DurationParseError(
                    "expected a value & unit for duration".into(),
                ))
            }?;

        let duration = match unit {
            DurationUnit::Hours => Duration::from_secs(value * 60 * 24),
            DurationUnit::Minutes => Duration::from_secs(value * 60),
            DurationUnit::Seconds => Duration::from_secs(value),
            DurationUnit::Milliseconds => Duration::from_millis(value),
            DurationUnit::Microseconds => Duration::from_micros(value),
            DurationUnit::Nanoseconds => Duration::from_nanos(value),
        };

        Ok(CustomDuration { duration })
    }
}

impl Into<Duration> for CustomDuration {
    fn into(self) -> Duration {
        self.duration
    }
}
