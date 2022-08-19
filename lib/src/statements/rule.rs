use super::{Context, Statement};
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

impl Rule {
    fn match_apply<R: RuleBehaviour>(&self, mut rb: R) -> R {
        if rb.does_match(self.match_opts.field, &self.match_opts.pattern) {
            rb.apply_action(&self.action_opts.action).unwrap();
        }

        rb
    }

    pub fn apply(&self, statement: &Statement) -> Statement {
        if !self.active {
            return statement.to_owned();
        }

        match (&self.match_opts.target, statement) {
            (Target::Transaction, Statement::Transaction(t)) => {
                Statement::Transaction(self.match_apply(t.to_owned()))
            }
            _ => statement.to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum ActionError {
    NotImplemented,
}

pub trait RuleBehaviour {
    fn apply_action(&mut self, action: &Action) -> Result<(), ActionError>;
    fn does_match(&self, field: Field, pattern: &Pattern) -> bool;
}

#[derive(Clone)]
pub struct RuleContext {
    rules: Vec<Rule>,
}

impl RuleContext {
    pub fn from_statements(statements: &[Statement]) -> Self {
        Self {
            rules: statements
                .iter()
                .filter_map(|s| match s {
                    Statement::Rule(rule) => Some(rule.to_owned()),
                    _ => None,
                })
                .collect::<Vec<Rule>>(),
        }
    }
}

impl Context for RuleContext {
    fn apply(&self, statement: &Statement) -> Statement {
        self.rules
            .iter()
            .fold(statement.to_owned(), |statement, rule: &Rule| {
                rule.apply(&statement)
            })
    }
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

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
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
pub struct Pattern(pub String);

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Action {
    Set { field: Field, value: String },
    ApplyTag { tag: String },
}
