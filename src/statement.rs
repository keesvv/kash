use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum Statement {
    None,
    Fixed(FixedStatement),
    Income(IncomeStatement),
}

#[derive(Clone, Debug)]
pub struct FixedStatement {
    pub tag: String,
    pub description: String,
    pub expenses: MonthValues,
}

#[derive(Clone, Debug)]
pub struct IncomeStatement {
    pub description: String,
    pub income: MonthValues,
}

#[derive(Clone)]
pub struct MonthValues {
    values: Vec<Option<f32>>,
}

impl MonthValues {
    pub fn new(values: Vec<Option<f32>>) -> Self {
        Self { values }
    }

    pub fn year(&self) -> f32 {
        self.values.iter().flatten().sum()
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
            dbg.field(month, &self.values.iter().nth(i).unwrap());
        }

        dbg.field("MonthlyAverage", &self.month_avg());
        dbg.finish()
    }
}
