use serde::{ser::SerializeMap, Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug)]
pub struct Goal {
    pub description: String,
    pub components: Vec<GoalComponent>,
}

impl Goal {
    pub fn get_amount(&self) -> f32 {
        self.components.iter().map(|c| c.amount).sum()
    }

    pub fn get_progress(&self) -> f32 {
        0.0 // TODO
    }
}

impl Serialize for Goal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("description", &self.description)?;
        map.serialize_entry("components", &self.components)?;
        map.serialize_entry("amount", &self.get_amount())?;
        map.serialize_entry("progress", &self.get_progress())?;
        map.end()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GoalComponent {
    pub description: String,
    pub amount: f32,
}
