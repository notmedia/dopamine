mod cli;

use std::time::Duration;

use clap::Parser;
use dopamine_core::Config;

use crate::cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let config = Config {
        timeout: args.timeout_ms.map(Duration::from_millis),
    };

    dopamine_core::stay_awake(config)?;

    Ok(())
}
