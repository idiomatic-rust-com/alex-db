use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotFound,
    ValueParse,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            NotFound => write!(f, "Not found."),
            ValueParse => write!(f, "Problem with parsing value."),
        }
    }
}
