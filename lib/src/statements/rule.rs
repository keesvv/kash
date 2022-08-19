use crate::rules::{Action, Field, Pattern, Target};
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MatchOpts {
    pub target: Target,
    pub field: Field,
    pub pattern: Pattern,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ActionOpts {
    #[serde(flatten)]
    pub action: Action,
}
