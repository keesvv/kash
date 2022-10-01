pub mod context;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Savings {
    #[serde(flatten)]
    pub model: SavingsModel,
    pub amount: f32,
    pub goal: String,
}

impl Savings {
    pub fn get_total_amount(&self, now: NaiveDate) -> f32 {
        match self.model {
            SavingsModel::Single => self.amount,
            SavingsModel::Recurring {
                start_date,
                end_date,
            } => {
                let diff = end_date.unwrap_or(now) - start_date;
                self.amount
                    * (match (diff.num_weeks() % 4).cmp(&0) {
                        Ordering::Equal => diff.num_weeks(),
                        Ordering::Greater => (diff.num_weeks() - diff.num_weeks() % 4),
                        Ordering::Less => 0,
                    } / 4) as f32
            }
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "model")]
pub enum SavingsModel {
    Single,
    #[serde(rename_all = "camelCase")]
    Recurring {
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
    },
}
