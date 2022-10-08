use super::Transaction;
use crate::statements::rule::{
    action_opts::{Action, ActionError},
    behaviour::RuleBehaviour,
    match_opts::Field,
    pattern::Pattern,
};

impl RuleBehaviour for Transaction {
    fn apply_action(&mut self, action: &Action) -> Result<(), ActionError> {
        match action {
            Action::ApplyTag { tag } => Ok(self.tag = Some(tag.to_owned())),
            Action::ReplaceDescription { replace, with } => Ok({
                self.description =
                    replace.0.replace_all(&self.description, with).to_string();
            }),
            Action::AddItems { items } => Ok(self.items.extend(items.to_owned())),
        }
    }

    fn does_match(&self, field: Field, pattern: &Pattern) -> bool {
        match field {
            Field::Description => pattern.0.is_match(&self.description),
        }
    }
}
