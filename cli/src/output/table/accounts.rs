use super::{
    value::{Cell, Col, ValueTable},
    ToTable,
};
use crate::output::OutputOptions;
use kash::statements::account::{Account, AccountType};

pub struct AccountsTable {
    pub accounts: Vec<Account>,
}

impl ToTable<ValueTable> for AccountsTable {
    fn to_table(&self, opts: OutputOptions) -> ValueTable {
        let mut table = ValueTable::new(
            "Accounts",
            &[
                Col::Text("type".into()),
                Col::Text("id".into()),
                Col::Text("name".into()),
                Col::Text("bank".into()),
            ],
            opts,
        );

        for account in &self.accounts {
            table.add_row(&[
                Cell::Text(
                    match account.account_type {
                        AccountType::Payment => "payment",
                        AccountType::Savings => "savings",
                    }
                    .to_string(),
                ),
                Cell::AccountId(account.id.to_owned()),
                Cell::Text(account.name.to_owned()),
                Cell::Text(account.bank.to_owned().unwrap_or_default()),
            ]);
        }

        table
    }
}
