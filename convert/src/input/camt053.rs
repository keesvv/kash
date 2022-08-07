use super::{Input, InputError};
use camt053::Document;
use chrono::{DateTime, FixedOffset};
use kash::{
    date::Date,
    statement::{Statement, Transaction},
};
use quick_xml::de;
use std::io::BufReader;

pub struct Camt053Input;

impl Camt053Input {
    pub fn new() -> Self {
        Self
    }
}

impl Input for Camt053Input {
    fn from_read<R>(&self, reader: R) -> Result<Vec<Statement>, InputError>
    where
        R: std::io::Read,
    {
        let document: Document = de::from_reader(BufReader::new(reader))
            .map_err(|e| InputError::Invalid(e.to_string()))?;
        let entry = &document.bank_to_customer.statements[0].entries[0];

        let transaction = Statement::Transaction(Transaction {
            date: Date(DateTime::from_local(
                entry.value_date.date.and_hms(0, 0, 0),
                FixedOffset::east(0),
            )),
            description: entry.additional_info.to_owned(),
            mutation: entry.amount.value * -1.0,
            tag: None,
        });

        Ok(vec![transaction])
    }
}
