use crate::value::MonthValues;

impl MonthValues {
    pub fn get_discretionary(&self, expenses: Self) -> Self {
        *self - expenses
    }
}
