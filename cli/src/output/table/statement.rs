use super::value::{Cell, Col, ValueTable};
use kash::statement::{Account, AccountType, FixedExpense, Income, Statement, Transaction};
use kash_convert::output::Output;
use std::io;

pub struct TableOutput {
    statements: Vec<Statement>,
}

impl TableOutput {
    pub fn new(statements: &[Statement]) -> Self {
        Self {
            statements: statements.to_owned(),
        }
    }

    pub fn format_fixed(&self, expenses: &[FixedExpense]) -> ValueTable {
        let mut table = ValueTable::new(
            "Fixed expenses",
            &[
                Col("tag".into(), Cell::Text(Default::default())),
                Col("description".into(), Cell::Text(Default::default())),
                Col("avg/mo".into(), Cell::Value(Default::default())),
                Col("year".into(), Cell::Value(Default::default())),
            ],
        );

        for expense in expenses {
            table.add_row(&[
                Cell::Text(expense.tag.to_owned()),
                Cell::Text(expense.description.to_owned()),
                Cell::Value(expense.expenses.month_avg() * -1.0),
                Cell::Value(expense.expenses.year() * -1.0),
            ]);
        }

        table.with_total(
            2,
            &[
                &expenses
                    .iter()
                    .map(|expense| expense.expenses.month_avg() * -1.0)
                    .collect::<Vec<f32>>(),
                &expenses
                    .iter()
                    .map(|expense| expense.expenses.year() * -1.0)
                    .collect::<Vec<f32>>(),
            ],
        )
    }

    pub fn format_income(&self, income: &[Income]) -> ValueTable {
        let mut table = ValueTable::new(
            "Income",
            &[
                Col("description".into(), Cell::Text(Default::default())),
                Col("avg/mo".into(), Cell::Value(Default::default())),
                Col("year".into(), Cell::Value(Default::default())),
            ],
        );

        for statement in income {
            table.add_row(&[
                Cell::Text(statement.description.to_owned()),
                Cell::Value(statement.income.month_avg()),
                Cell::Value(statement.income.year()),
            ]);
        }

        table.with_total(
            1,
            &[
                &income
                    .iter()
                    .map(|statement| statement.income.month_avg())
                    .collect::<Vec<f32>>(),
                &income
                    .iter()
                    .map(|statement| statement.income.year())
                    .collect::<Vec<f32>>(),
            ],
        )
    }

    pub fn format_transactions(&self, transactions: &[Transaction]) -> ValueTable {
        let mut table = ValueTable::new(
            "Transactions",
            &[
                Col("date".into(), Cell::Text(Default::default())),
                Col("description".into(), Cell::Text(Default::default())),
                Col("mutation".into(), Cell::Text(Default::default())),
                Col("tag".into(), Cell::Text(Default::default())),
            ],
        );

        for transaction in transactions {
            table.add_row(&[
                Cell::Text(transaction.date.0.format("%Y/%m/%d").to_string()),
                Cell::Text(
                    transaction
                        .description
                        .chars()
                        .take(60)
                        .collect::<String>()
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .join(" "),
                ),
                Cell::Value(transaction.mutation),
                Cell::Text(transaction.tag.to_owned().unwrap_or_default()),
            ]);
        }

        table
    }

    pub fn format_accounts(&self, accounts: &[Account]) -> ValueTable {
        let mut table = ValueTable::new(
            "Accounts",
            &[
                Col("type".into(), Cell::Text(Default::default())),
                Col("id".into(), Cell::Text(Default::default())),
                Col("name".into(), Cell::Text(Default::default())),
                Col("bank".into(), Cell::Text(Default::default())),
            ],
        );

        for account in accounts {
            table.add_row(&[
                Cell::Text(
                    match account.account_type {
                        AccountType::Payment => "payment",
                        AccountType::Savings => "savings",
                    }
                    .to_string(),
                ),
                Cell::Text(account.id.to_string()),
                Cell::Text(account.name.to_owned()),
                Cell::Text(account.bank.to_owned()),
            ]);
        }

        table
    }
}

impl Output for TableOutput {
    fn to_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        let mut fixed: Vec<FixedExpense> = Vec::new();
        let mut income: Vec<Income> = Vec::new();
        let mut transactions: Vec<Transaction> = Vec::new();
        let mut accounts: Vec<Account> = Vec::new();

        for statement in &self.statements {
            match &statement {
                Statement::Fixed(f) => fixed.push(f.to_owned()),
                Statement::Income(i) => income.push(i.to_owned()),
                Statement::Transaction(t) => transactions.push(t.to_owned()),
                Statement::Account(a) => accounts.push(a.to_owned()),
            }
        }

        write!(
            writer,
            "{}",
            [
                self.format_accounts(&accounts),
                self.format_fixed(&fixed),
                self.format_income(&income),
                self.format_transactions(&transactions),
            ]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n\n")
        )
    }
}
