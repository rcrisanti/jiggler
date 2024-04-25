pub mod cli;
pub mod error;

use std::{
    thread,
    time::{Duration, Instant},
};

use enigo::{Coordinate, Enigo, Mouse, Settings};
use log::{debug, info, trace};

pub use error::{JigglerError, JigglerResult};

pub struct Jiggler {
    enigo: Enigo,
    start_in: Duration,
    run_for: Option<Duration>,
    jiggle_every: Duration,
    jiggle_config: JiggleConfig,
}

impl Jiggler {
    pub fn new(
        start_in: Duration,
        run_for: Option<Duration>,
        jiggle_every: Duration,
        jiggle_config: JiggleConfig,
        enigo_settings: &Settings,
    ) -> JigglerResult<Self> {
        Ok(Jiggler {
            enigo: Enigo::new(enigo_settings)?,
            start_in,
            run_for,
            jiggle_every,
            jiggle_config,
        })
    }

    pub fn run(&mut self) -> Result<(), JigglerError> {
        info!(
            "jiggling every {:?} - starting in {:?} and ending after {:?}",
            self.jiggle_every, self.start_in, self.run_for
        );
        thread::sleep(self.start_in);
        let start_jiggle_time = Instant::now();
        let end_jiggle_time = self.run_for.map(|dur| start_jiggle_time + dur);
        debug!("jiggling will end at {end_jiggle_time:?}");
        loop {
            trace!("jiggling mouse");
            jiggle(
                &mut self.enigo,
                self.jiggle_config.sleep,
                self.jiggle_config.repetitions,
                self.jiggle_config.half_size,
            )?;
            trace!("sleeping for {:?}", self.jiggle_every);
            thread::sleep(self.jiggle_every);
            if let Some(end) = end_jiggle_time {
                if Instant::now() >= end {
                    info!("ending jiggler process because max time was reached");
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
    half_size: i32,
}

impl JiggleConfig {
    pub fn new(sleep: Duration, repetitions: usize, half_size: i32) -> Self {
        JiggleConfig {
            sleep,
            repetitions,
            half_size,
        }
    }
}

fn jiggle(
    enigo: &mut Enigo,
    sleep: Duration,
    repetitions: usize,
    half_size: i32,
) -> Result<(), JigglerError> {
    for _ in 0..repetitions {
        enigo.move_mouse(half_size, 0, Coordinate::Rel)?;
        thread::sleep(sleep);
        enigo.move_mouse(-half_size, 0, Coordinate::Rel)?;
        thread::sleep(sleep);
    }
    Ok(())
}
