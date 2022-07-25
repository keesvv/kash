use super::error::{Error, Result};

pub struct Deserializer<'a> {
    input: &'a str,
}

impl<'a> Deserializer<'a> {
    pub fn from_str(input: &'a str) -> Self {
        Deserializer { input }
    }

    pub fn peek_char(&mut self) -> Result<char> {
        self.input.chars().next().ok_or(Error::Eof)
    }

    pub fn next_char(&mut self) -> Result<char> {
        let c = self.peek_char()?;
        self.input = &self.input[c.len_utf8()..];
        Ok(c)
    }

    pub fn parse_f32(&self) -> Result<f32> {
        match self.input.parse() {
            Ok(val) => Ok(val),
            Err(_) => Err(Error::ExpectedFloat),
        }
    }
}
