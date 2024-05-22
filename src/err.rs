use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Error {
    Other(String),
    NotLogin(String),
}

impl Error {
    pub fn msg(&self) -> &str {
        match self {
            Error::Other(msg) => msg,
            Error::NotLogin(msg) => msg,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Other(msg) => write!(f, "Other: {msg}"),
            Error::NotLogin(msg) => write!(f, "NotLogin: {msg}"),
        }
    }
}
