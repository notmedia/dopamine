use clap::Parser;
use dopamine_core::Config;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// Create an assertion to prevent the system from idle sleeping.
    #[arg(short, long)]
    pub idle: bool,

    /// Create an assertion to prevent the display from sleeping.
    #[arg(short, long)]
    pub display: bool,

    /// Timeout in seconds for which this assertion has to be valid.
    #[arg(short, long)]
    pub timeout: Option<u64>,
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
        }
        .into_config();

        assert!(config.idle && !config.display);

        let config = Cli {
            idle: false,
            display: true,
            timeout: None,
        }
        .into_config();

        assert!(!config.idle && config.display);
    }
}
