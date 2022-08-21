pub mod value;

use self::value::{Cell, Col, ValueTable};
use super::OutputOptions;
use kash::{
    statements::{
        account::{Account, AccountType},
        budget::Budget,
        fixed::FixedExpense,
        income::Income,
        transaction::Transaction,
        Statement,
    },
    value::MonthValues,
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
                Col::Text("tag".into()),
                Col::Text("description".into()),
                Col::Value("avg/mo".into()),
                Col::Value("year".into()),
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

    pub fn format_income(
        &self,
        income: &[Income],
        gross_income: MonthValues,
        disc_income: MonthValues,
    ) -> ValueTable {
        let mut table = ValueTable::new(
            "Income",
            &[
                Col::Text("description".into()),
                Col::Value("avg/mo".into()),
                Col::Value("year".into()),
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

        table.add_row(&[
            Cell::Text("total (gross)".into()),
            Cell::Value(gross_income.month_avg()),
            Cell::Value(gross_income.year()),
        ]);

        table.add_row(&[
            Cell::Text("total (discr.)".into()),
            Cell::Value(disc_income.month_avg()),
            Cell::Value(disc_income.year()),
        ]);

        table
    }

    pub fn format_transactions(
        &self,
        transactions: &[Transaction],
        budget: &[Budget],
        disc_income: MonthValues,
    ) -> ValueTable {
        let mut table = ValueTable::new(
            "Latest transactions",
            &[
                Col::Text("date".into()),
                Col::Text("description".into()),
                Col::Value("mutation".into()),
                Col::Text("tag".into()),
                Col::Text("quota".into()),
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
                        budget.quota.get_month_values(disc_income).month_avg(),
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
                Col::Text("type".into()),
                Col::Text("id".into()),
                Col::Text("name".into()),
                Col::Text("bank".into()),
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
                Cell::AccountId(account.id.to_owned()),
                Cell::Text(account.name.to_owned()),
                Cell::Text(account.bank.to_owned().unwrap_or_default()),
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
        let mut expenses: Vec<FixedExpense> = Vec::new();
        let mut income: Vec<Income> = Vec::new();
        let mut transactions: Vec<Transaction> = Vec::new();
        let mut accounts: Vec<Account> = Vec::new();
        let mut budget: Vec<Budget> = Vec::new();

        for statement in &self.statements {
            match &statement {
                Statement::Fixed(f) => expenses.push(f.to_owned()),
                Statement::Income(i) => income.push(i.to_owned()),
                Statement::Transaction(t) => transactions.push(t.to_owned()),
                Statement::Account(a) => accounts.push(a.to_owned()),
                Statement::Budget(b) => budget.push(b.to_owned()),
                Statement::Rule(_) => (),
            }
        }

        let gross_income: MonthValues = income.iter().map(|i| i.income).sum();
        let total_expenses: MonthValues = expenses.iter().map(|e| e.expenses).sum();
        let reserved_budget: MonthValues = budget
            .iter()
            .filter(|b| b.reserved)
            .map(|b| b.quota.get_month_values(gross_income))
            .sum();
        let disc_income = gross_income.get_discretionary(total_expenses + reserved_budget);

        write!(
            writer,
            "{}",
            [
                self.format_accounts(&accounts),
                self.format_fixed(&expenses),
                self.format_income(&income, gross_income, disc_income),
                self.format_transactions(&transactions, &budget, disc_income),
            ]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n\n")
        )
    }
}
