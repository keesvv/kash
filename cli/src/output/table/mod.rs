pub mod value;

use self::value::{Cell, Col, ValueTable};
use super::OutputOptions;
use kash::statements::{
    account::{Account, AccountType},
    budget::{Budget, Quota},
    fixed::FixedExpense,
    income::Income,
    transaction::Transaction,
    Statement,
};
use kash_convert::output::Output;
use std::io;

pub struct TableOutput {
    statements: Vec<Statement>,
    opts: OutputOptions,
}

impl TableOutput {
    pub fn new(statements: &[Statement], opts: OutputOptions) -> Self {
        Self {
            statements: statements.to_owned(),
            opts,
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
            self.opts,
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
            self.opts,
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

    pub fn format_transactions(
        &self,
        transactions: &[Transaction],
        budget: &[Budget],
        income: f32,
    ) -> ValueTable {
        let mut table = ValueTable::new(
            "Latest transactions",
            &[
                Col("date".into(), Cell::Text(Default::default())),
                Col("description".into(), Cell::Text(Default::default())),
                Col("mutation".into(), Cell::Text(Default::default())),
                Col("tag".into(), Cell::Text(Default::default())),
                Col("quota".into(), Cell::Text(Default::default())),
            ],
            self.opts,
        );

        let mut transactions = transactions.iter().collect::<Vec<&Transaction>>();
        transactions.sort_by(|a, b| b.date.cmp(&a.date));

        for transaction in transactions.iter().take(10) {
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
                match budget
                    .iter()
                    .find(|b| b.tag.eq(&transaction.tag.to_owned().unwrap_or_default()))
                {
                    Some(budget) => Cell::Quota(
                        transaction.mutation.abs(),
                        match budget.quota {
                            Quota::Absolute(a) => a,
                            Quota::Percentage(p) => (p / 100.0) * income,
                        },
                    ),
                    None => Cell::Text("".into()),
                },
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
            self.opts,
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
                if self.opts.discrete {
                    Cell::MaskedText(account.id.to_string())
                } else {
                    Cell::Text(account.id.to_string())
                },
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
        let mut budget: Vec<Budget> = Vec::new();

        for statement in &self.statements {
            match &statement {
                Statement::Fixed(f) => fixed.push(f.to_owned()),
                Statement::Income(i) => income.push(i.to_owned()),
                Statement::Transaction(t) => transactions.push(t.to_owned()),
                Statement::Account(a) => accounts.push(a.to_owned()),
                Statement::Budget(b) => budget.push(b.to_owned()),
            }
        }

        write!(
            writer,
            "{}",
            [
                self.format_accounts(&accounts),
                self.format_fixed(&fixed),
                self.format_income(&income),
                self.format_transactions(
                    &transactions,
                    &budget,
                    income.iter().map(|i| i.income.month_avg()).sum()
                ),
            ]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n\n")
        )
    }
}
