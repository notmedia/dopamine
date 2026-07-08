mod error;
pub use error::Error;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(not(target_os = "macos"))]
compile_error!("dopamine currently only supports macOS");

use std::time::Duration;

#[derive(Debug)]
pub struct Config {
    pub timeout: Option<Duration>,
}

pub struct StayAwakeGuard {
    id: u32,
}

impl Drop for StayAwakeGuard {
    fn drop(&mut self) {
        macos::release(self.id).unwrap();
    }
}

/// Prevents the system from sleeping until the returned guard is dropped.
///
/// # Errors
///
/// Returns [`Error::AssertionFailed`] if the OS declines to create the power
/// assertion.
///
/// # Panics
/// If can't acquire
pub fn stay_awake(config: &Config) -> Result<StayAwakeGuard, Error> {
    println!("creating assertion for {:?}", &config);

    let id = macos::acquire("dopamine").unwrap();

    Ok(StayAwakeGuard { id })
}
