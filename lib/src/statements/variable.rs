use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Variable {
    pub id: String,
    pub description: String,
    #[serde(flatten)]
    pub value: VariableKind,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum VariableKind {
    Value { value: f32 },
    Text { text: String },
}
