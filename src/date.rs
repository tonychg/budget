use chrono::{DateTime, FixedOffset, Months, Utc};
use core::fmt;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(DateTime<FixedOffset>);

impl Default for Date {
    fn default() -> Self {
        Self(Utc::now().fixed_offset())
    }
}

impl From<String> for Date {
    fn from(value: String) -> Self {
        Self(
            match DateTime::parse_from_str(
                &format!("{} 12:00:00 +0000", value),
                "%Y-%m-%d %H:%M:%S %z",
            ) {
                Ok(date) => date,
                Err(_) => DateTime::parse_from_str(
                    &format!("{} 12:00:00 +0000", value),
                    "%d/%m/%Y %H:%M:%S %z",
                )
                .expect("Invalid date format: use YYYY-MM-DD"),
            },
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
