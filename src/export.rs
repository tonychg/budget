#![allow(dead_code)]
use std::path::PathBuf;

use serde::Deserialize;

use crate::{Budget, Calendar, Date, Payment};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    date_op: String,
    date_val: String,
    label: String,
    category: String,
    category_parent: String,
    supplier_found: String,
    amount: String,
    account_num: String,
    account_label: String,
    account_balance: String,
    comment: Option<String>,
    pointer: String,
}

impl From<Operation> for Payment {
    fn from(op: Operation) -> Self {
        Self::new(
            &op.label,
            match op.amount.replace(',', ".").replace(' ', "").parse() {
                Ok(amount) => amount,
                Err(e) => {
                    log::debug!("{}: {}", e.to_string(), op.amount);
                    0.0
                }
            },
            &op.date_op,
            None,
        )
    }
}

#[derive(Debug, Clone)]
pub struct Export(Vec<Operation>);

impl Export {
    pub fn from_file(path: PathBuf) -> Self {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b';')
            .from_path(path)
            .expect("Unable to read file");
        let mut operations = Vec::new();
        for op in rdr.deserialize::<Operation>() {
            match op {
                Ok(op) => operations.push(op),
                Err(e) => log::debug!("{:?}", e),
            }
        }
        Self(operations)
    }
}

impl From<Export> for Budget {
    fn from(value: Export) -> Self {
        let mut operations = value.0.clone();
        operations.sort_by(|a, b| {
            Date::from(a.date_op.clone())
                .partial_cmp(&Date::from(b.date_op.clone()))
                .unwrap()
        });
        Self {
            payments: operations.clone().into_iter().map(|op| op.into()).collect(),
            calendar: Calendar::new(&operations.first().expect("Empty export").date_op),
        }
    }
}
