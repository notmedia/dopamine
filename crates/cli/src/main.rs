mod cli;

use std::sync::mpsc;
use std::time::Duration;

use clap::Parser;
use dopamine_core::AwakeGuard;

use crate::cli::Cli;

const POLL_INTERVAL: Duration = Duration::from_secs(1);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let timeout = args.timeout;
    let pid = args.pid;

    let config = args.into_config();

    let _guard = AwakeGuard::acquire(&config)?;

    let (tx, rx) = mpsc::channel();

    if let Some(pid) = pid {
        let tx = tx.clone();
        std::thread::spawn(move || {
            loop {
                if !dopamine_core::process::is_alive(pid) {
                    let _ = tx.send(());
                    return;
                }
                std::thread::sleep(POLL_INTERVAL);
            }
        });
    }

    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })?;

    match timeout {
        Some(timeout) => {
            let _ = rx.recv_timeout(timeout);
        }
        None => {
            let _ = rx.recv();
        }
    }

    Ok(())
}
