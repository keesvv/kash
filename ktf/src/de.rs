use super::error::{Error, Result};

pub struct Deserializer<'a> {
    input: &'a str,
}

impl<'a> Deserializer<'a> {
    pub fn from_str(input: &'a str) -> Self {
        Deserializer { input }
    }

    pub fn advance(&mut self, n: usize) {
        self.input = &self.input[n..];
    }

    pub fn peek_char(&self) -> Result<char> {
        self.input.chars().next().ok_or(Error::Eof)
    }

    pub fn next_char(&mut self) -> Result<char> {
        let c = self.peek_char()?;
        self.advance(c.len_utf8());
        Ok(c)
    }

    pub fn parse_header(&mut self) -> Result<Vec<String>> {
        let mut header = Vec::new();

        if self.next_char()? != '>' {
            return Err(Error::ExpectedHeader);
        }

        header.push(self.parse_header_col()?);

        Ok(header)
    }

    pub fn parse_header_col(&mut self) -> Result<String> {
        let col = self.input;
        self.advance(col.len());
        Ok(col.to_owned())
    }

    pub fn parse_f32(&self) -> Result<f32> {
        match self.input.parse() {
            Ok(val) => Ok(val),
            Err(_) => Err(Error::ExpectedFloat),
        }
    }
}
