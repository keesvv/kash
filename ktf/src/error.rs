use std::fmt::Display;
use std::{error, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    ExpectedHeader,
    ExpectedFloat,
    ExpectedMap,
    ExpectedMapValue,
    MapEnd,
    Syntax,
    Eof,
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Error::Message(m) => m,
            Error::ExpectedHeader => "expected header",
            Error::ExpectedFloat => "expected float",
            Error::ExpectedMap => "expected map",
            Error::ExpectedMapValue => "expected map value",
            Error::MapEnd => "unexpected end of map",
            Error::Syntax => "syntax error",
            Error::Eof => "unexpected EOF",
        })
    }
}