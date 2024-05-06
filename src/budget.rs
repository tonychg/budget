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

use std::{fs, path::PathBuf};

#[derive(Clone, Debug)]
pub struct Budget {
    payments: Vec<Payment>,
    calendar: Calendar,
}

impl Budget {
    pub fn from_file(path: PathBuf) -> Self {
        let config: Config =
            toml::from_str(&fs::read_to_string(path).expect("Unable to read file"))
                .expect("Invalid TOML file");
        config.into()
    }

    pub fn from_export(path: PathBuf) -> Self {
        let export = Export::from_file(path);
        export.into()
    }

    pub fn show(&self, months: u32, filter: Vec<String>, all: bool) {
        let mut total = 0.0;

        self.group_by_month(months, filter)
            .iter()
            .for_each(|(date, group)| {
                println!("{} total={} month={}", date, total, group.sum());
                if all {
                    group.iter().for_each(|line| println!("  {}", line));
                }
                total += group.sum();
            });
    }

    fn payments_at(&self, date: Date, months: u32, filter: Vec<String>) -> PaymentGroup {
        self.payments
            .clone()
            .into_iter()
            .flat_map(|payment| payment.flatten(months))
            .filter(move |payment| payment.date() >= date && payment.date() < date.add_months(1))
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
