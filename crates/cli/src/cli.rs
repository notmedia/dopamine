use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// Timeout in seconds for which this assertion has to be valid.
    #[arg(short, long)]
    pub timeout_ms: Option<u64>,
}
