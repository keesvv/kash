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
    pub costs: FixedCosts,
}

#[derive(Clone, Debug)]
pub struct IncomeStatement {
    pub description: String,
    pub costs: FixedCosts,
}

#[derive(Clone)]
pub struct FixedCosts {
    costs: Vec<Option<f32>>,
}

impl FixedCosts {
    pub fn new(costs: Vec<Option<f32>>) -> Self {
        Self { costs }
    }

    pub fn year(&self) -> f32 {
        self.costs.iter().flatten().sum::<f32>()
    }

    pub fn month_avg(&self) -> f32 {
        self.year() / 12.0
    }
}

impl Debug for FixedCosts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("FixedCosts");

        for (i, month) in [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ]
        .iter()
        .enumerate()
        {
            dbg.field(month, &self.costs.iter().nth(i).unwrap());
        }

        dbg.field("MonthlyAverage", &self.month_avg());
        dbg.finish()
    }
}
