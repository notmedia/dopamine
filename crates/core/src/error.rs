#[derive(Debug)]
pub enum Error {
    AssertionFailure(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AssertionFailure(err) => write!(f, "power assertion failed: {err}"),
        }
    }
}

impl std::error::Error for Error {}
