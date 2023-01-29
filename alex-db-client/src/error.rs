use std::fmt;

#[derive(Debug)]
pub enum ClientError<'a> {
    NoActiveConnection,
    Repl(reedline_repl_rs::Error),
    Request(reqwest::Error),
    String(&'a str),
}

impl fmt::Display for ClientError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::NoActiveConnection => write!(f, "No active connection."),
            ClientError::Repl(e) => write!(f, "REPL error: {e}."),
            ClientError::Request(e) => write!(f, "Request error: {e}."),
            ClientError::String(s) => write!(f, "{s}."),
        }
    }
}

impl From<reedline_repl_rs::Error> for ClientError<'_> {
    fn from(e: reedline_repl_rs::Error) -> Self {
        ClientError::Repl(e)
    }
}

impl From<reqwest::Error> for ClientError<'_> {
    fn from(e: reqwest::Error) -> Self {
        ClientError::Request(e)
    }
}

impl std::error::Error for ClientError<'_> {}
