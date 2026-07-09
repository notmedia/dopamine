mod cli;

use std::sync::mpsc;
use std::time::Duration;

use clap::Parser;
use dopamine_core::{AwakeGuard, Config};

use crate::cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let config = Config {
        timeout: args.timeout_ms.map(Duration::from_millis),
    };

    let _guard = AwakeGuard::acquire(&config)?;

    let (tx, rx) = mpsc::channel();

    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })?;

    match config.timeout {
        Some(timeout) => {
            let _ = rx.recv_timeout(timeout);
        }
        None => {
            let _ = rx.recv();
        }
    }

    Ok(())
}
