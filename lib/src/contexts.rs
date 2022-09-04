use crate::statements::Statement;

pub trait Context {
    fn add(&mut self, statements: &[Statement]);
    fn apply(&self, statement: Statement) -> Statement;
    fn apply_all(&self, statements: &[Statement]) -> Vec<Statement> {
        statements
            .iter()
            .map(|statement| self.apply(statement.to_owned()))
            .collect()
    }
}

pub fn apply_stacked(
    statements: &[Statement],
    contexts: &mut [Box<dyn Context>],
) -> Vec<Statement> {
    contexts
        .iter_mut()
        .fold(statements.to_owned(), |statements, context| {
            context.add(&statements);
            context.apply_all(&statements)
        })
}
