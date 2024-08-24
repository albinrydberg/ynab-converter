use chrono::NaiveDate;

pub use parserpicker::Parser;

mod table;
mod xlsparser;
mod xlsxparser;
mod parserpicker;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub ledger_date: NaiveDate,
    pub transaction_date: NaiveDate,
    pub text: String,
    pub amount: f32,
    pub balance: Option<f32>,
}

