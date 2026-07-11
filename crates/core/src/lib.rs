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
    id: u32,
}

impl AwakeGuard {
    /// Prevents the system from sleeping until the returned guard is dropped.
    ///
    /// # Errors
    ///
    /// Returns [`Error::AssertionFailure`] if the OS declines to create the
    /// power assertion.
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
