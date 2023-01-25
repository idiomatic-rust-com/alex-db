use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    NotFound,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            NotFound => write!(f, "Not found."),
        }
    }
}
