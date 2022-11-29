use mouce::Mouse;
use std::{
    thread,
    time::{Duration, Instant},
};
use tracing::{debug, info, trace};

use crate::error::JigglerError;

pub struct Jiggler {
    start_in: Duration,
    run_for: Duration,
    jiggle_every: Duration,
    jiggle_config: JiggleConfig,
}

impl Jiggler {
    pub fn new(
        start_in: Duration,
        run_for: Duration,
        jiggle_every: Duration,
        jiggle_config: JiggleConfig,
    ) -> Self {
        Jiggler {
            start_in,
            run_for,
            jiggle_every,
            jiggle_config,
        }
    }

    pub fn run(&self) -> Result<(), JigglerError> {
        info!(
            "jiggling every {:?} - starting in {:?} and ending after {:?}",
            self.jiggle_every, self.start_in, self.run_for
        );
        thread::sleep(self.start_in);
        let mut start_jiggle_time = Instant::now();
        let end_jiggle_time = if self.run_for == Duration::MAX {
            None
        } else {
            Some(start_jiggle_time + self.run_for)
        };

        loop {
            debug!("jiggling mouse");
            jiggle(
                self.jiggle_config.sleep,
                self.jiggle_config.repetitions,
                self.jiggle_config.half_size,
            )?;
            trace!("sleeping for {:?}", self.jiggle_every);
            thread::sleep(self.jiggle_every);

            if let Some(end) = end_jiggle_time {
                start_jiggle_time = Instant::now();
                if start_jiggle_time > end {
                    break;
                }
            }
        }

        Ok(())
    }
}

pub struct JiggleConfig {
    sleep: Duration,
    repetitions: usize,
    half_size: u16,
}

impl JiggleConfig {
    pub fn new(sleep: Duration, repetitions: usize, half_size: u16) -> Self {
        JiggleConfig {
            sleep,
            repetitions,
            half_size,
        }
    }
}

fn jiggle(sleep: Duration, repetitions: usize, half_size: u16) -> Result<(), JigglerError> {
    let mouse = Mouse::new();
    let half_size: i32 = half_size.into();

    for _ in 0..repetitions {
        mouse.move_relative(half_size, 0)?;
        thread::sleep(sleep);

        mouse.move_relative(-half_size, 0)?;
        thread::sleep(sleep);
    }

    Ok(())
}
