use super::account::AccountId;
use crate::date::Date;
use crate::rules::{Action, ActionError, Field, Pattern, RuleBehaviour};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub tag: Option<String>,
    pub description: String,
    pub date: Date,
    pub mutation: f32,
    pub account_id: AccountId,
}

impl RuleBehaviour for Transaction {
    fn apply_action(&mut self, action: &Action) -> Result<(), ActionError> {
        match action {
            Action::ApplyTag { tag } => Ok(self.tag = Some(tag.to_owned())),
            Action::Set { .. } => todo!(),
        }
    }

    fn does_match(&self, field: Field, pattern: &Pattern) -> bool {
        match field {
            Field::Description => pattern.0.is_match(&self.description),
        }
    }
}
