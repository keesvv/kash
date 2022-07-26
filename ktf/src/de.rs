use super::error::{Error, Result};

pub struct Deserializer<'a> {
    input: &'a str,
    col_index: usize,
    pub cur_header: Vec<String>,
}

impl<'a> Deserializer<'a> {
    pub fn from_str(input: &'a str) -> Self {
        Deserializer {
            input,
            col_index: 0,
            cur_header: vec![],
        }
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

    pub fn next_row(&mut self) -> Result<Row> {
        let row = self.input.lines().next().ok_or(Error::Eof)?;
        self.advance(row.len() + 1);
        Ok(Row::new(row))
    }

    pub fn next_col(&mut self) -> Option<String> {
        let col = self
            .cur_header
            .iter()
            .nth(self.col_index)
            .map(String::to_owned)?;

        self.col_index += 1;
        Some(col)
    }

    pub fn parse_header(&mut self) -> Result<Vec<String>> {
        let header: Vec<String> = match self.next_char()? {
            '>' => Ok(self.next_row()?.cols.iter().map(String::to_owned).collect()),
            _ => Err(Error::ExpectedHeader),
        }?;

        self.cur_header = header.clone();
        Ok(header)
    }
}

#[derive(Debug, Clone)]
pub struct Row {
    pub cols: Vec<String>,
}

impl Row {
    pub fn new(row: &str) -> Self {
        Self {
            cols: row.split('|').map(str::trim).map(str::to_owned).collect(),
        }
    }
}
