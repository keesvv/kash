pub mod action_opts;
pub mod behaviour;
pub mod context;
pub mod match_opts;
pub mod pattern;

use self::{
    action_opts::ActionOpts,
    match_opts::{MatchOpts, Target},
};
use crate::statements::Statement;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Rule {
    pub description: Option<String>,
    #[serde(default)]
    pub active: bool,
    #[serde(rename = "match")]
    pub match_opts: MatchOpts,
    #[serde(rename = "action")]
    pub action_opts: ActionOpts,
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
