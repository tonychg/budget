use crate::Date;

#[derive(Default, Clone, Debug)]
pub struct Calendar {
    start_at: Date,
}

impl Calendar {
    pub fn new(start_at: &str) -> Self {
        Calendar {
            start_at: start_at.to_string().into(),
        }
    }

    pub fn iter_months(self, months: u32) -> impl Iterator<Item = Date> {
        (0..months).map(move |i| self.start_at.add_months(i).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::Calendar;

    #[test]
    fn iter_months() {
        let calendar = Calendar::new("2024-01-01");
        let months: Vec<String> = calendar
            .iter_months(3)
            .map(|date| date.to_string())
            .collect();
        assert_eq!(months, vec!["2024-01-01", "2024-02-01", "2024-03-01"]);
    }

    #[test]
    fn iter_months_offset() {
        let calendar = Calendar::new("2024-02-01");
        let months: Vec<String> = calendar
            .iter_months(5)
            .map(|date| date.to_string())
            .collect();
        assert_eq!(
            months,
            vec![
                "2024-02-01",
                "2024-03-01",
                "2024-04-01",
                "2024-05-01",
                "2024-06-01"
            ]
        )
    }

    #[test]
    fn iter_months_single() {
        let calendar = Calendar::new("2024-02-01");
        let months: Vec<String> = calendar
            .iter_months(1)
            .map(|date| date.to_string())
            .collect();
        assert_eq!(months, vec!["2024-02-01"])
    }

    #[test]
    fn iter_months_empty() {
        let calendar = Calendar::new("2024-02-01");
        let months: Vec<String> = calendar
            .iter_months(0)
            .map(|date| date.to_string())
            .collect();
        assert!(months.is_empty())
    }
}
