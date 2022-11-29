mod cli;
mod error;
mod jiggler;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::FmtSubscriber;

use cli::Args;
use jiggler::{JiggleConfig, Jiggler};

fn main() -> Result<()> {
    let cli = Args::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(cli.log_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let jiggler = Jiggler::new(
        cli.start.into(),
        cli.end.into(),
        cli.wait.into(),
        JiggleConfig::new(cli.jiggle_sleep.into(), cli.jiggle_reps, cli.jiggle_size),
    );
    jiggler.run()?;
    Ok(())
}
