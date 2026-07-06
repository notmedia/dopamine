use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// Timeout in milliseconds. Runs forever if omitted.
    #[arg(short, long)]
    pub timeout_ms: Option<u64>,
}
