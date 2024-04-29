use crate::{Budget, Calendar, Recurence};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    start_at: Option<String>,
    subscriptions: Vec<Subscription>,
}

#[derive(Clone, Debug, Deserialize)]
struct Subscription {
    label: String,
    amount: f64,
    date: String,
    recurence: Option<Recurence>,
}

impl From<&Subscription> for crate::Subscription {
    fn from(config: &Subscription) -> Self {
        crate::Subscription::new(
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
            subscriptions: value.subscriptions.iter().map(|s| s.into()).collect(),
            calendar: Calendar::new(&value.start_at.unwrap_or_default()),
        }
    }
}
