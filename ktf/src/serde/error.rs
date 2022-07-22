use serde::de;
use std::fmt::Display;
use std::{error, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    ExpectedFloat,
    ExpectedMap,
    Syntax,
    Eof,
}

impl error::Error for Error {}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(m) => f.write_str(m),
            Error::ExpectedFloat => f.write_str("expected float"),
            Error::ExpectedMap => f.write_str("expected map"),
            Error::Syntax => f.write_str("syntax error"),
            Error::Eof => f.write_str("unexpected EOF"),
        }
    }
}
