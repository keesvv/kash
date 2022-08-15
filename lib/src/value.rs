use rust_decimal::prelude::*;
use serde::de::Deserialize;
use serde::ser::{Serialize, SerializeMap};
use std::fmt::Debug;
use std::iter;

#[derive(Clone)]
pub struct MonthValues {
    pub values: [Decimal; 12],
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

impl<'de> Deserialize<'de> for MonthValues {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let items: Vec<Decimal> = Deserialize::deserialize(deserializer)?;
        Ok(Self::from_iter(items.iter().map(Decimal::to_owned)))
    }
}

impl MonthValues {
    pub fn new(values: [Decimal; 12]) -> Self {
        Self { values }
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: Iterator<Item = Decimal>,
    {
        Self::new(
            iter.chain(iter::repeat(Decimal::ZERO))
                .take(12)
                .collect::<Vec<Decimal>>()
                .try_into()
                .unwrap(),
        )
    }

    pub fn year(&self) -> Decimal {
        self.values.iter().sum()
    }

    pub fn month_avg(&self) -> Decimal {
        self.year() / Decimal::from(12)
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
