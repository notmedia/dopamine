use std::ffi::NulError;

#[derive(Debug)]
pub enum Error {
    AssertionFailure(String),
    InvalidName(NulError),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::AssertionFailure(_) => None,
            Error::InvalidName(err) => Some(err),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AssertionFailure(err) => write!(f, "power assertion failed: {err}"),
            Error::InvalidName(_) => write!(f, "assertion name contains a nul byte"),
        }
    }
}
