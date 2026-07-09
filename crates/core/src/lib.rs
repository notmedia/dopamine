mod error;
pub use error::Error;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
use macos as platform;

#[cfg(not(target_os = "macos"))]
compile_error!("dopamine currently only supports macOS");

use std::time::Duration;

#[derive(Debug)]
pub struct Config {
    pub timeout: Option<Duration>,
}

pub struct AwakeGuard {
    id: u32,
}

impl AwakeGuard {
    /// Prevents the system from sleeping until the returned guard is dropped.
    ///
    /// # Errors
    ///
    /// Returns [`Error::AssertionFailed`] if the OS declines to create the power assertion.
    pub fn acquire(_config: &Config) -> Result<Self, Error> {
        let id = platform::acquire("dopamine")?;

        Ok(AwakeGuard { id })
    }
}

impl Drop for AwakeGuard {
    fn drop(&mut self) {
        let _ = platform::release(self.id);
    }
}
