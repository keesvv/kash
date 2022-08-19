use crate::statements::{rule::Rule, Context, Statement};

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
    fn apply(&self, statement: Statement) -> Statement {
        self.rules
            .iter()
            .fold(statement, |statement, rule| statement.apply_rule(rule))
    }
}
