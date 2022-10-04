pub mod accounts;
pub mod expenses;
pub mod goals;
pub mod income;
pub mod transactions;
pub mod value;

use super::OutputOptions;
use accounts::AccountsTable;
use expenses::ExpensesTable;
use goals::GoalsTable;
use income::IncomeTable;
use kash::{
    statements::{
        account::Account, budget::Budget, fixed::FixedExpense, goal::Goal,
        income::Income, savings::Savings, transaction::Transaction, Statement,
    },
    value::MonthValues,
};
use kash_convert::output::Output;
use std::{fmt::Display, io};
use transactions::TransactionsTable;

pub struct TableOutput {
    statements: Vec<Statement>,
    opts: OutputOptions,
}

pub trait TableLike: Display {}
pub trait ToTable<T: TableLike> {
    fn to_table(&self, opts: OutputOptions) -> T;
}

impl TableOutput {
    pub fn new(statements: &[Statement], opts: OutputOptions) -> Self {
        Self {
            statements: statements.to_owned(),
            opts,
        }
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
        let mut goals: Vec<Goal> = Vec::new();
        let mut savings: Vec<Savings> = Vec::new();

        for statement in &self.statements {
            match &statement {
                Statement::Fixed(f) => expenses.push(f.to_owned()),
                Statement::Income(i) => income.push(i.to_owned()),
                Statement::Transaction(t) => transactions.push(t.to_owned()),
                Statement::Account(a) => accounts.push(a.to_owned()),
                Statement::Budget(b) => budget.push(b.to_owned()),
                Statement::Goal(g) => goals.push(g.to_owned()),
                Statement::Savings(s) => savings.push(s.to_owned()),
                _ => (),
            }
        }

        let gross_income: MonthValues = income.iter().map(|i| i.income).sum();
        let total_expenses: MonthValues = expenses.iter().map(|e| e.expenses).sum();
        let reserved_budget: MonthValues = budget
            .iter()
            .filter(|b| b.reserved)
            .map(|b| b.quota.get_month_values(gross_income))
            .sum();

        let disc_income =
            gross_income.get_discretionary(total_expenses + reserved_budget, &savings);

        write!(
            writer,
            "{}",
            [
                AccountsTable { accounts }.to_table(self.opts),
                GoalsTable { goals }.to_table(self.opts),
                ExpensesTable { expenses }.to_table(self.opts),
                IncomeTable {
                    income,
                    disc_income,
                    gross_income
                }
                .to_table(self.opts),
                TransactionsTable {
                    budget,
                    transactions,
                    disc_income
                }
                .to_table(self.opts),
            ]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n\n")
        )
    }
}
