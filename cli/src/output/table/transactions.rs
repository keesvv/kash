use super::{
    value::{Cell, Col, ValueTable},
    ToTable,
};
use crate::output::OutputOptions;
use kash::{
    statements::{budget::Budget, transaction::Transaction},
    value::MonthValues,
};

pub struct TransactionsTable {
    pub transactions: Vec<Transaction>,
    pub budget: Vec<Budget>,
    pub disc_income: MonthValues,
}

impl ToTable<ValueTable> for TransactionsTable {
    fn to_table(&self, opts: OutputOptions) -> ValueTable {
        let mut table = ValueTable::new(
            "Latest transactions",
            &[
                Col::Text("date".into()),
                Col::Text("description".into()),
                Col::Value("mutation".into()),
                Col::Text("tag".into()),
                Col::Text("quota".into()),
            ],
            opts,
        );

        let mut transactions = self.transactions.iter().collect::<Vec<&Transaction>>();
        transactions.sort_by(|a, b| b.date.cmp(&a.date));

        for transaction in transactions.iter().take(10) {
            table.add_row(&[
                Cell::Text(transaction.date.format("%Y/%m/%d").to_string()),
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
                Cell::Value(transaction.amount * -1.0),
                Cell::Text(transaction.tag.to_owned().unwrap_or_default()),
                match self
                    .budget
                    .iter()
                    .find(|b| b.tag.eq(&transaction.tag.to_owned().unwrap_or_default()))
                {
                    Some(budget) => Cell::Quota(
                        transaction.amount,
                        budget.quota.get_month_values(self.disc_income).month_avg(),
                    ),
                    None => Cell::Text("".into()),
                },
            ]);
        }

        table
    }
}
