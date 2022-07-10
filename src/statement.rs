use std::fmt::Debug;

#[derive(Debug)]
pub enum Statement<'a> {
    None,
    Fixed(FixedStatement<'a>),
}

#[derive(Debug)]
pub struct FixedStatement<'a> {
    pub tag: &'a str,
    pub description: &'a str,
    pub costs: FixedCosts,
}

pub struct FixedCosts {
    costs: Vec<Option<i32>>,
}

impl FixedCosts {
    pub fn new(costs: Vec<Option<i32>>) -> Self {
        Self { costs }
    }

    pub fn month_avg(&self) -> f32 {
        let h: f32 = self.costs.iter().flatten().map(|c| *c as f32).sum();
        h / 12.0
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
