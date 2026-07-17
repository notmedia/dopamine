use std::time::Duration;

use clap::Parser;
use dopamine_core::Config;

fn parse_duration(s: &str) -> Result<Duration, String> {
    duration_str::parse(s)
}

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// Create an assertion to prevent the system from idle sleeping.
    #[arg(short, long)]
    pub idle: bool,

    /// Create an assertion to prevent the display from sleeping.
    #[arg(short, long)]
    pub display: bool,

    /// How long to stay awake, e.g. "30s", "5m", "1h30m". Forever if omitted.
    #[arg(short, long, value_parser = parse_duration)]
    pub timeout: Option<Duration>,

    /// Wait for the process with the specified pid to exit.
    /// Once the process exits, the assertion is also released.
    #[arg(short, long)]
    pub pid: Option<i32>,
}

impl Cli {
    pub fn into_config(self) -> Config {
        if !self.idle && !self.display {
            Config::default()
        } else {
            Config {
                idle: self.idle,
                display: self.display,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_conversion_with_default() {
        let config = Cli {
            idle: false,
            display: false,
            timeout: None,
            pid: None,
        }
        .into_config();

        assert!(config.idle && !config.display);

        let config = Cli {
            idle: false,
            display: true,
            timeout: None,
            pid: None,
        }
        .into_config();

        assert!(!config.idle && config.display);
    }
}
