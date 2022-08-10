use crate::value::MonthValues;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FixedExpense {
    pub tag: String,
    pub description: String,
    pub expenses: MonthValues,
}
