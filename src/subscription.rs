use crate::{Date, Line, Recurence};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Subscription {
    pub label: String,
    pub amount: f64,
    pub date: Date,
    pub recurence: Recurence,
}

impl Subscription {
    pub fn new(label: &str, amount: f64, date: &str, recurence: Recurence) -> Self {
        Subscription {
            label: label.to_string(),
            amount,
            date: date.to_string().into(),
            recurence,
        }
    }

    pub fn lines(&self, months: u32) -> Vec<Line> {
        (0..self.recurence.months(months))
            .map(|i| Line {
                label: self.label.clone(),
                amount: self.amount,
                timestamp: self.date.add_months(i),
            })
            .collect()
    }
}
