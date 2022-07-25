use crate::error::Error;
use serde::de;
use std::fmt::Display;

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}
