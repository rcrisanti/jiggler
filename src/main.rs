use clap::Parser;
use enigo::Settings;

use jiggler::cli::Args;
use jiggler::{JiggleConfig, Jiggler, JigglerResult};

fn main() -> JigglerResult<()> {
    env_logger::init();
    let cli = Args::parse();
    let config = JiggleConfig::new(cli.jiggle_sleep.into(), cli.jiggle_reps, cli.jiggle_size);
    let mut jiggler = Jiggler::new(
        cli.start.into(),
        cli.end.map(|end| end.into()),
        cli.wait.into(),
        config,
        &Settings::default(),
    )?;
    jiggler.run()?;
    Ok(())
}
