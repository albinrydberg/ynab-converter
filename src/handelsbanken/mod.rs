use chrono::NaiveDate;

use crate::ynab::Convertible;
use crate::{amex, util};
pub use parserpicker::Parser;

mod parserpicker;
mod table;
mod xlsparser;
mod xlsxparser;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub ledger_date: NaiveDate,
    pub transaction_date: NaiveDate,
    pub text: String,
    pub amount: f32,
    pub balance: Option<f32>,
}

impl Convertible for amex::Transaction {
    fn to_ynab(self) -> crate::ynab::Transaction {
        let negative_amount = -self.amount;
        let flow = util::convert_amount_to_flow(negative_amount);
        crate::ynab::Transaction {
            date: self.date,
            memo: self.description,
            outflow: flow.outflow,
            inflow: flow.inflow,
            ..crate::ynab::Transaction::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::handelsbanken;
    use crate::ynab::Convertible;

    #[test]
    fn handelsbanken_to_ynab() {
        // Given
        let handelsbanken = handelsbanken::Transaction {
            ledger_date: NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(),
            transaction_date: NaiveDate::from_ymd_opt(2020, 3, 22).unwrap(),
            text: "HSB Göteborg".to_string(),
            amount: -5552.0,
            balance: None,
        };

        // When
        let row = handelsbanken.to_ynab();

        // Then
        assert_eq!(NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(), row.date);
        assert_eq!("", row.payee);
        assert_eq!("", row.category);
        assert_eq!("HSB Göteborg", row.memo);
        assert_eq!(5552.0, row.outflow);
        assert_eq!(0.0, row.inflow);
    }
}
