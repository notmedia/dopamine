mod config;
mod error;

pub use config::Config;
pub use error::Error;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
use macos as platform;

#[cfg(not(target_os = "macos"))]
compile_error!("dopamine currently only supports macOS");

pub struct AwakeGuard {
    _token: platform::Token,
}

impl AwakeGuard {
    /// Prevents the system from sleeping until the returned guard is dropped.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidConfig`] if `config` requests no assertions
    /// (both `idle` and `display` are `false`).
    ///
    /// Returns [`Error::AssertionFailure`] if the OS declines to create a
    /// power assertion. If several assertions were requested and one fails,
    /// any already created are released before the error is returned.
    pub fn acquire(config: &Config) -> Result<Self, Error> {
        if !config.idle && !config.display {
            return Err(Error::InvalidConfig("no assertions passed".into()));
        }

        let token = platform::acquire("dopamine", config)?;

        Ok(AwakeGuard { _token: token })
    }
}
