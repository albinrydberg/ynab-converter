use chrono::NaiveDate;

use crate::{util, ynab};

mod table;
mod xlsparser;

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub ledger_date: NaiveDate,
    pub transaction_date: NaiveDate,
    pub text: String,
    pub amount: f32,
    pub balance: Option<f32>,
}

pub fn new_parser() -> HandelsbankenParser {
    HandelsbankenParser{}
}

impl ynab::Convertible for Row {
    fn to_ynab(self) -> ynab::Row {
        let flow = util::convert_amount_to_flow(self.amount);
        ynab::Row {
            date: self.ledger_date,
            memo: self.text,
            outflow: flow.outflow,
            inflow: flow.inflow,
            ..ynab::Row::default()
        }
    }
}

pub struct HandelsbankenParser;

impl ynab::Parser for HandelsbankenParser {
    fn read_from_file(&self, file_path: String) -> anyhow::Result<Vec<impl ynab::Convertible>> {
        xlsparser::read_xls(file_path)
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::handelsbanken::Row;
    use crate::ynab::Convertible;

    #[test]
    fn to_ynab() {
        // Given
        let original_row = Row {
            ledger_date: NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(),
            transaction_date: NaiveDate::from_ymd_opt(2020, 3, 22).unwrap(),
            text: "HSB Göteborg".to_string(),
            amount: -5552.0,
            balance: None,
        };

        // When
        let row = original_row.to_ynab();

        // Then
        assert_eq!(NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(), row.date);
        assert_eq!("", row.payee);
        assert_eq!("", row.category);
        assert_eq!("HSB Göteborg", row.memo);
        assert_eq!(5552.0, row.outflow);
        assert_eq!(0.0, row.inflow);
    }
}
