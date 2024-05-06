use chrono::{DateTime, Datelike, FixedOffset, Months, Utc};
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

    pub fn modulo(&self) -> Self {
        Date::from(format!("{}-{}-{}", self.0.year(), self.0.month(), "01"))
    }
}

#[cfg(test)]
mod tests {
    use super::Date;
    use chrono::DateTime;

    #[test]
    fn test_from_string() {
        assert_eq!(
            Date::from("2021-02-01".to_string()),
            Date(
                DateTime::parse_from_str("2021-02-01 12:00:00 +0000", "%Y-%m-%d %H:%M:%S %z")
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_from_string_slash_format() {
        assert_eq!(
            Date::from("01/02/2021".to_string()),
            Date(
                DateTime::parse_from_str("01/02/2021 12:00:00 +0000", "%d/%m/%Y %H:%M:%S %z")
                    .unwrap()
            )
        );
    }

    #[test]
    fn test_display() {
        let date = Date::from("2021-02-01".to_string());
        assert_eq!(date.to_string(), "2021-02-01".to_string());
    }

    #[test]
    fn test_add_months() {
        let date = Date::from("2021-01-01".to_string());
        assert_eq!(date.add_months(1), Date::from("2021-02-01".to_string()));
        assert_eq!(date.add_months(12), Date::from("2022-01-01".to_string()));
    }

    #[test]
    fn test_modulo() {
        let date = Date::from("2021-01-15".to_string());
        assert_eq!(date.modulo(), Date::from("2021-01-01".to_string()));
    }
}
