mod calendar;
mod config;
mod date;
mod export;
mod payment;

pub(crate) use {
    calendar::Calendar,
    config::Config,
    date::Date,
    export::Export,
    payment::{Payment, PaymentGroup, Recurence},
};

use anyhow::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, thiserror::Error)]
pub enum BudgetError {
    #[error("No file provided")]
    NoFileProvided,
    #[error("File unreadable: {0:?}")]
    UnreadableFile(#[source] std::io::Error),
    #[error("Invalid TOML file: {0}")]
    InvalidToml(#[from] toml::de::Error),
}

#[derive(Clone, Debug)]
pub struct Budget {
    payments: Vec<Payment>,
    calendar: Calendar,
}

impl Budget {
    pub fn load(paths: Vec<PathBuf>) -> Result<Self> {
        match paths.is_empty() {
            true => bail!(BudgetError::NoFileProvided),
            false => {
                let mut budget = Self::from_file(paths.first().unwrap().to_path_buf())?;
                for path in paths.iter().skip(1) {
                    budget.merge(Self::from_file(path.to_path_buf())?);
                }
                Ok(budget)
            }
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Self> {
        match Self::is_csv(&path) {
            true => Ok(Export::from_file(path).into()),
            false => Self::from_toml(path),
        }
    }

    pub fn from_toml(path: PathBuf) -> Result<Self> {
        Ok(toml::from_str::<Config>(&fs::read_to_string(path)?)?.into())
    }

    fn extract_extension(path: &Path) -> Option<String> {
        path.extension()?
            .to_str()?
            .to_string()
            .to_lowercase()
            .into()
    }

    fn is_csv(path: &Path) -> bool {
        match Self::extract_extension(path) {
            Some(extension) => extension == "csv",
            None => false,
        }
    }

    pub fn show(&self, months: u32, filter: Vec<String>, all: bool) {
        let mut total = 0.0;

        self.group_by_month(months, filter)
            .iter()
            .for_each(|(date, group)| {
                println!("{} total={:>8.2} month={:>8.2}", date, total, group.sum());
                if all {
                    group.iter().for_each(|line| println!("  {}", line));
                }
                total += group.sum();
            });
    }

    fn merge(&mut self, budget: Self) {
        self.payments.extend(budget.payments);
        self.payments
            .sort_by(|a, b| a.date().partial_cmp(&b.date()).unwrap());
        self.calendar = Calendar::new(&self.payments.first().unwrap().date().modulo().to_string());
    }

    fn payments_at(&self, date: Date, months: u32, filter: Vec<String>) -> PaymentGroup {
        self.payments
            .clone()
            .into_iter()
            .flat_map(|payment| payment.repeat(months))
            .filter(move |payment| {
                let (start, end) = date.to_month_interval();
                payment.date() >= start && payment.date() <= end
            })
            .filter(move |payment| match filter.is_empty() {
                false => filter.iter().any(|f| payment.label_match(f)),
                true => true,
            })
            .collect()
    }

    fn group_by_month(&self, months: u32, filter: Vec<String>) -> Vec<(Date, PaymentGroup)> {
        self.calendar
            .clone()
            .iter_months(months)
            .map(|date| (date.clone(), self.payments_at(date, months, filter.clone())))
            .collect()
    }
}
