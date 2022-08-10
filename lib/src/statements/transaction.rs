use crate::date::Date;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Transaction {
    pub tag: Option<String>,
    pub description: String,
    pub date: Date,
    pub mutation: f32,
}
