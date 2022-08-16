use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Rule {
    pub description: Option<String>,
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

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Field {
    Description,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Target {
    Transaction,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Pattern(String);

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Action {
    Set { field: Field, value: String },
}
