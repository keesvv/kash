use super::{Input, InputError};
use camt053::{CreditOrDebit, Document, Entry};
use chrono::{DateTime, Utc};
use kash::{
    date::Date,
    statements::{account::AccountId, transaction::Transaction, Statement},
};
use quick_xml::de;
use std::io::BufReader;

pub struct Camt053Input;

impl Camt053Input {
    pub fn new() -> Self {
        Self
    }

    pub fn get_description(entry: &camt053::Entry) -> Option<String> {
        // FIXME: can we improve this?
        match entry.additional_info.to_owned() {
            Some(info) => Some(info),
            None => Some(
                entry.details.to_owned()?.transactions[0]
                    .to_owned()
                    .remittance_info
                    .to_owned()?
                    .unstructured?
                    .to_owned(),
            ),
        }
    }

    pub fn parse_statement(statement: &camt053::Statement) -> Vec<Statement> {
        statement
            .entries
            .iter()
            .filter_map(|entry: &Entry| match entry.credit_or_debit.value {
                CreditOrDebit::Debit => Some(entry),
                CreditOrDebit::Credit => None,
            })
            .map(|entry: &Entry| {
                Statement::Transaction(Transaction {
                    date: Date(DateTime::from_utc(
                        entry.value_date.date.and_hms(0, 0, 0),
                        Utc,
                    )),
                    description: Self::get_description(entry).unwrap_or_default(),
                    mutation: entry.amount.value * -1.0,
                    tag: None,
                    account_id: AccountId::Iban(statement.account.id.value.as_str_id().into()),
                })
            })
            .collect()
    }
}

impl Input for Camt053Input {
    fn from_read<R>(&self, reader: R) -> Result<Vec<Statement>, InputError>
    where
        R: std::io::Read,
    {
        let document: Document = de::from_reader(BufReader::new(reader))
            .map_err(|e| InputError::Invalid(e.to_string()))?;

        Ok(document
            .bank_to_customer
            .statements
            .iter()
            .flat_map(Self::parse_statement)
            .collect())
    }
}
