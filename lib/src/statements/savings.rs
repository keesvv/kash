use chrono::{NaiveDate, Utc};
use serde::{ser::SerializeMap, Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Deserialize, Clone, Debug)]
pub struct Goal {
    pub id: String,
    pub description: String,
    #[serde(default)]
    pub components: Vec<GoalComponent>,
    #[serde(skip_deserializing)]
    pub progress: f32,
}

impl Goal {
    pub fn get_total(&self) -> f32 {
        self.components.iter().map(|c| c.amount).sum()
    }
}

impl Serialize for Goal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("description", &self.description)?;
        map.serialize_entry("components", &self.components)?;
        map.serialize_entry("total", &self.get_total())?;
        map.serialize_entry("progress", &self.progress)?;
        map.end()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GoalComponent {
    pub description: String,
    pub amount: f32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Savings {
    #[serde(flatten)]
    pub model: SavingsModel,
    pub amount: f32,
    pub goal: String,
}

impl Savings {
    pub fn get_total_amount(&self) -> f32 {
        match self.model {
            SavingsModel::Single => self.amount,
            SavingsModel::Recurring {
                start_date,
                end_date,
            } => {
                let diff = end_date.unwrap_or(Utc::now().date_naive()) - start_date;
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
