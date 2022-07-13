use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct Value(pub f32);

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:.2}", self.0))
    }
}
