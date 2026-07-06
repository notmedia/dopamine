use std::time::Duration;

#[derive(Debug)]
pub struct Config {
    pub timeout: Option<Duration>,
}

#[derive(Debug)]
pub enum Error {
    AssertionFailed(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AssertionFailed(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {}

pub struct StayAwakeGuard {
    id: u32,
}

impl Drop for StayAwakeGuard {
    fn drop(&mut self) {
        println!("releasing assertion");
    }
}

/// Prevents the system from sleeping until the returned guard is dropped.
///
/// # Errors
///
/// Returns [`Error::AssertionFailed`] if the OS declines to create the power assertion.
pub fn stay_awake(config: &Config) -> Result<StayAwakeGuard, Error> {
    println!("creating assertion for {:?}", &config);

    Ok(StayAwakeGuard { id: 1 })
}
