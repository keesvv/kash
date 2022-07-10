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
}
