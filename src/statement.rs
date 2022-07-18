use serde::Serialize;
use std::fmt::Debug;
use std::iter;

#[derive(Serialize, Clone, Debug)]
pub struct FixedStatement {
    pub tag: String,
    pub description: String,
    pub expenses: MonthValues,
}

#[derive(Serialize, Clone, Debug)]
pub struct IncomeStatement {
    pub description: String,
    pub income: MonthValues,
}

#[derive(Serialize, Clone)]
pub struct MonthValues {
    pub values: [f32; 12],
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
