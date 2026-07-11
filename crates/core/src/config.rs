#[derive(Debug)]
pub struct Config {
    pub idle: bool,
    pub display: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            idle: true,
            display: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_default() {
        let config = Config::default();
        assert!(config.idle && !config.display);
    }
}
