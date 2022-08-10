use super::{Input, InputError};
use camt053::{Document, Entry};
use chrono::{DateTime, Utc};
use kash::{
    date::Date,
    statements::{transaction::Transaction, Statement},
};
use quick_xml::de;
use std::io::BufReader;

pub struct Camt053Input;

impl Camt053Input {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_entry(entry: &Entry) -> Statement {
        Statement::Transaction(Transaction {
            date: Date(DateTime::from_utc(
                entry.value_date.date.and_hms(0, 0, 0),
                Utc,
            )),
            description: entry.additional_info.to_owned(),
            mutation: entry.amount.value * -1.0,
            tag: None,
        })
    }
}

impl Input for Camt053Input {
    fn from_read<R>(&self, reader: R) -> Result<Vec<Statement>, InputError>
    where
        R: std::io::Read,
    {
        let document: Document = de::from_reader(BufReader::new(reader))
            .map_err(|e| InputError::Invalid(e.to_string()))?;

        Ok(document.bank_to_customer.statements[0]
            .entries
            .iter()
            .map(Self::parse_entry)
            .collect())
    }
}
