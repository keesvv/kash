pub mod context;

use crate::statements::{rule::Rule, Statement};
use serde::{Deserialize, Serialize};

impl Rule {
    fn match_apply<R: RuleBehaviour>(&self, mut rb: R) -> R {
        if rb.does_match(self.match_opts.field, &self.match_opts.pattern) {
            rb.apply_action(&self.action_opts.action).unwrap();
        }

        rb
    }
}

impl Statement {
    pub fn apply_rule(&self, rule: &Rule) -> Self {
        if !rule.active {
            return self.to_owned();
        }

        match (rule.match_opts.target, self) {
            (Target::Transaction, Self::Transaction(t)) => {
                Self::Transaction(rule.match_apply(t.to_owned()))
            }
            _ => self.to_owned(),
        }
    }
}

pub trait RuleBehaviour {
    fn apply_action(&mut self, action: &Action) -> Result<(), ActionError>;
    fn does_match(&self, field: Field, pattern: &Pattern) -> bool;
}

#[derive(Debug)]
pub enum ActionError {
    NotImplemented,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Field {
    Description,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Target {
    Transaction,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Pattern(pub String);

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Action {
    Set { field: Field, value: String },
    ApplyTag { tag: String },
}
