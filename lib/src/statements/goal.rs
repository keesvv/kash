use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Goal {
    pub id: String,
    pub description: String,
    #[serde(default)]
    pub components: Vec<GoalComponent>,
    #[serde(default)]
    pub progress: f32,
}

impl Goal {
    pub fn get_total(&self) -> f32 {
        self.components.iter().map(|c| c.amount).sum()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GoalComponent {
    pub description: String,
    pub amount: f32,
}
