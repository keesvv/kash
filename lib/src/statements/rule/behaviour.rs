use super::{
    action_opts::{Action, ActionError},
    match_opts::Field,
    pattern::Pattern,
    Rule,
};

impl Rule {
    pub fn match_apply<R: RuleBehaviour>(&self, mut rb: R) -> R {
        if rb.does_match(self.match_opts.field, &self.match_opts.pattern) {
            rb.apply_action(&self.action_opts.action).unwrap();
        }

        rb
    }
}

pub trait RuleBehaviour {
    fn apply_action(&mut self, action: &Action) -> Result<(), ActionError>;
    fn does_match(&self, field: Field, pattern: &Pattern) -> bool;
}
