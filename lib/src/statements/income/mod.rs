pub mod discretionary;

use crate::value::MonthValues;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Income {
    pub description: String,
    pub income: MonthValues,
}
