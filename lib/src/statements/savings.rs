use serde::{ser::SerializeMap, Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug)]
pub struct Goal {
    pub description: String,
    #[serde(default)]
    pub components: Vec<GoalComponent>,
    #[serde(skip_deserializing)]
    pub progress: f32,
}

impl Goal {
    pub fn get_total(&self) -> f32 {
        self.components.iter().map(|c| c.amount).sum()
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
        map.serialize_entry("total", &self.get_total())?;
        map.serialize_entry("progress", &self.progress)?;
        map.end()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GoalComponent {
    pub description: String,
    pub amount: f32,
}
