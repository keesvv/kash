use serde::de::Deserialize;
use serde::ser::Serialize;
use std::fmt::Debug;
use std::iter::{self, Sum};
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Default)]
pub struct MonthValues {
    pub values: [f32; 12],
}

impl Add for MonthValues {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.values
                .iter()
                .enumerate()
                .map(|(i, v)| v + rhs.values.iter().nth(i).unwrap())
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl Sub for MonthValues {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.values
                .iter()
                .enumerate()
                .map(|(i, v)| v - rhs.values.iter().nth(i).unwrap())
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl Sum for MonthValues {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let init = iter.next().unwrap_or_default();

        iter.collect::<Vec<Self>>()
            .iter()
            .fold(init, |init, new| init + *new)
    }
}

impl Serialize for MonthValues {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Serialize::serialize(&self.values, serializer)
    }
}

impl<'de> Deserialize<'de> for MonthValues {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let items: Vec<f32> = Deserialize::deserialize(deserializer)?;
        Ok(Self::from_iter(items.iter().map(f32::to_owned)))
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
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov",
            "Dec",
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
