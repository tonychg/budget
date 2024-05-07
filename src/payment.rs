use crate::Date;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Recurence {
    Monthly,
    NumberOfMonths(u32),
}

impl Recurence {
    pub fn months(&self, months: u32) -> u32 {
        match self {
            Recurence::Monthly => months,
            Recurence::NumberOfMonths(n) => *n,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Payment {
    label: String,
    amount: f64,
    date: Date,
    recurence: Option<Recurence>,
}

impl Payment {
    pub fn new(label: &str, amount: f64, date: &str, recurence: Option<Recurence>) -> Self {
        Payment {
            label: label.to_string(),
            amount,
            date: date.to_string().into(),
            recurence,
        }
    }

    pub fn date(&self) -> Date {
        self.date.clone()
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn label_match(&self, label: &str) -> bool {
        self.label.to_lowercase().contains(&label.to_lowercase())
    }

    pub fn recurence(&self) -> Recurence {
        self.recurence
            .clone()
            .unwrap_or(Recurence::NumberOfMonths(1))
    }

    pub fn repeat(&self, months: u32) -> Vec<Payment> {
        self.date
            .repeat_by_month(self.recurence().months(months))
            .into_iter()
            .map(|date| Payment {
                label: self.label.clone(),
                amount: self.amount,
                date,
                recurence: None,
            })
            .collect()
    }
}

impl Display for Payment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:6}) {}", self.date, self.amount, self.label)
    }
}

#[derive(Clone, Debug)]
pub struct PaymentGroup(Vec<Payment>);

impl PaymentGroup {
    pub fn sum(&self) -> f64 {
        self.0.iter().map(|payment| payment.amount()).sum()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Payment> {
        self.0.iter()
    }
}

impl std::iter::FromIterator<Payment> for PaymentGroup {
    fn from_iter<T: IntoIterator<Item = Payment>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
