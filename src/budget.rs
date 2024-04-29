mod calendar;
mod config;
mod date;
mod payment;

pub(crate) use {calendar::Calendar, config::Config, date::Date, payment::Payment};

use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Clone, Debug)]
pub struct Line {
    pub label: String,
    pub amount: f64,
    pub timestamp: Date,
}

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

#[derive(Clone, Debug)]
pub struct Group(Vec<Line>);

impl Group {
    pub fn sum(&self) -> f64 {
        self.0.iter().map(|line| line.amount).sum()
    }
}

impl std::iter::FromIterator<Line> for Group {
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[derive(Clone, Debug)]
pub struct Budget {
    payments: Vec<Payment>,
    calendar: Calendar,
}

impl Budget {
    pub fn new(start_at: &str) -> Self {
        Budget {
            payments: Vec::new(),
            calendar: Calendar::new(start_at),
        }
    }

    pub fn from_file(path: PathBuf) -> Self {
        let config: Config =
            toml::from_str(&fs::read_to_string(path).expect("Unable to read file"))
                .expect("Invalid TOML file");
        config.into()
    }

    pub fn register(&mut self, label: &str, amount: f64, start_at: &str, recurence: Recurence) {
        self.payments
            .push(Payment::new(label, amount, start_at, recurence));
    }

    pub fn lines_at(&self, date: Date, months: u32) -> Group {
        self.payments
            .clone()
            .into_iter()
            .flat_map(|payment| payment.lines(months))
            .filter(move |line| line.timestamp >= date && line.timestamp < date.add_months(1))
            .collect()
    }

    pub fn group_by_date(&self, months: u32) -> Vec<(Date, Group)> {
        self.calendar
            .clone()
            .iter_months(months)
            .map(|date| (date.clone(), self.lines_at(date, months)))
            .collect()
    }
}
