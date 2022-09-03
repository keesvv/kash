use crate::{
    contexts::Context,
    statements::{rule::Rule, Statement},
};

#[derive(Clone)]
pub struct RuleContext {
    rules: Vec<Rule>,
}

impl RuleContext {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
}

impl Context for RuleContext {
    fn add(&mut self, statements: &[Statement]) {
        self.rules.extend(
            statements
                .iter()
                .filter_map(|s| match s {
                    Statement::Rule(rule) => Some(rule.to_owned()),
                    _ => None,
                })
                .collect::<Vec<Rule>>(),
        )
    }

    fn apply(&self, statement: Statement) -> Statement {
        self.rules
            .iter()
            .fold(statement, |statement, rule| statement.apply_rule(rule))
    }
}
