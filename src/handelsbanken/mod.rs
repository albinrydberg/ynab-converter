use chrono::NaiveDate;

pub use xlsparser::new_parser;
pub use xlsparser::Parser;

mod table;
mod xlsparser;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub ledger_date: NaiveDate,
    pub transaction_date: NaiveDate,
    pub text: String,
    pub amount: f32,
    pub balance: Option<f32>,
}

