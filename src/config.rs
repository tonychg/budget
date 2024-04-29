use crate::{Budget, Calendar, Recurence};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    start_at: Option<String>,
    payments: Vec<Payment>,
}

#[derive(Clone, Debug, Deserialize)]
struct Payment {
    label: String,
    amount: f64,
    date: String,
    recurence: Option<Recurence>,
}

impl From<&Payment> for crate::Payment {
    fn from(config: &Payment) -> Self {
        crate::Payment::new(
            &config.label,
            config.amount,
            &config.date,
            config.recurence.clone().unwrap_or(Recurence::Monthly),
        )
    }
}

impl From<Config> for Budget {
    fn from(value: Config) -> Self {
        Budget {
            payments: value.payments.iter().map(|s| s.into()).collect(),
            calendar: Calendar::new(&value.start_at.unwrap_or_default()),
        }
    }
}
