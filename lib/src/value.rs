use serde::ser::{Serialize, SerializeMap};
use std::fmt::Debug;
use std::iter;

#[derive(Clone)]
pub struct MonthValues {
    pub values: [f32; 12],
}

impl Serialize for MonthValues {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("values", &self.values)?;
        map.serialize_entry("year", &self.year())?;
        map.serialize_entry("month_avg", &self.month_avg())?;
        map.end()
    }
}

impl MonthValues {
    pub fn new(values: [f32; 12]) -> Self {
        Self { values }
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: Iterator<Item = f32>,
    {
        Self::new(
            iter.chain(iter::repeat(0.0))
                .take(12)
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap(),
        )
    }

    pub fn year(&self) -> f32 {
        self.values.iter().sum()
    }

    pub fn month_avg(&self) -> f32 {
        self.year() / 12.0
    }
}

impl Debug for MonthValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("MonthValues");

        for (i, month) in [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ]
        .iter()
        .enumerate()
        {
            dbg.field(month, &self.values[i]);
        }

        dbg.field("MonthlyAverage", &self.month_avg());
        dbg.field("Year", &self.year());
        dbg.finish()
    }
}
