use super::value::MonthValues;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct FixedStatement {
    pub tag: String,
    pub description: String,
    pub expenses: MonthValues,
}

#[derive(Serialize, Clone, Debug)]
pub struct IncomeStatement {
    pub description: String,
    pub income: MonthValues,
}
