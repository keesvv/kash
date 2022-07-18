use crate::format::Statement;
use std::fmt::{self, Display};

pub struct StatementsJson {
    statements: Vec<Statement>,
}

impl StatementsJson {
    pub fn new(statements: &[Statement]) -> Self {
        Self {
            statements: statements.to_owned(),
        }
    }
}

impl Display for StatementsJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self.statements).unwrap()
        )
    }
}
