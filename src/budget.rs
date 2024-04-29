mod calendar;

pub use calendar::Calendar;

use chrono::{DateTime, FixedOffset, Months, Utc};
use core::fmt;
use serde::Deserialize;
use std::{
    fmt::{Display, Formatter},
    fs,
    path::PathBuf,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Date(DateTime<FixedOffset>);

impl Default for Date {
    fn default() -> Self {
        Self(Utc::now().fixed_offset())
    }
}

impl From<&str> for Date {
    fn from(value: &str) -> Self {
        Self(
            DateTime::parse_from_str(&format!("{} 12:00:00 +0000", value), "%Y-%m-%d %H:%M:%S %z")
                .expect("Invalid date format: use YYYY-MM-DD"),
        )
    }
}

impl From<String> for Date {
    fn from(value: String) -> Self {
        Self(
            DateTime::parse_from_str(&format!("{} 12:00:00 +0000", value), "%Y-%m-%d %H:%M:%S %z")
                .expect("Invalid date format: use YYYY-MM-DD"),
        )
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

impl Date {
    pub fn add_months(&self, months: u32) -> Self {
        Date(
            self.0
                .checked_add_months(Months::new(months))
                .expect("Invalid months value"),
        )
    }
}

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
            date: date.into(),
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

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    start_at: String,
    subscriptions: Vec<SubscriptionConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SubscriptionConfig {
    label: String,
    amount: f64,
    date: String,
    recurence: Option<Recurence>,
}

impl From<&SubscriptionConfig> for Subscription {
    fn from(config: &SubscriptionConfig) -> Self {
        Subscription::new(
            &config.label,
            config.amount,
            &config.date,
            config.recurence.clone().unwrap_or(Recurence::Monthly),
        )
    }
}

#[derive(Clone, Debug)]
pub struct Budget {
    subscriptions: Vec<Subscription>,
    calendar: Calendar,
}

impl Budget {
    pub fn new(start_at: &str) -> Self {
        Budget {
            subscriptions: Vec::new(),
            calendar: Calendar::new(start_at),
        }
    }

    pub fn from_file(path: PathBuf) -> Self {
        let config = fs::read_to_string(path).expect("Unable to read file");
        let config: Config = toml::from_str(&config).expect("Invalid TOML file");
        Budget {
            subscriptions: config.subscriptions.iter().map(|s| s.into()).collect(),
            calendar: Calendar::new(&config.start_at),
        }
    }

    pub fn register(&mut self, label: &str, amount: f64, start_at: &str, recurence: Recurence) {
        self.subscriptions
            .push(Subscription::new(label, amount, start_at, recurence));
    }

    pub fn lines_at(&self, date: Date, months: u32) -> Group {
        self.subscriptions
            .clone()
            .into_iter()
            .flat_map(|subscription| subscription.lines(months))
            .filter(move |line| {
                line.timestamp.0 >= date.0 && line.timestamp.0 < date.add_months(1).0
            })
            .collect()
    }

    pub fn lines(&self, months: u32) -> Vec<(Date, Group)> {
        self.calendar
            .clone()
            .iter_months(months)
            .map(|date| (date.clone(), self.lines_at(date, months)))
            .collect()
    }
}
