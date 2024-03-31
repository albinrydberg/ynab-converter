use crate::{handelsbanken, nordea};
use crate::ynab::Transaction;

pub trait Convertible {
    fn to_ynab(self) -> Transaction;
}

impl Convertible for handelsbanken::Transaction {
    fn to_ynab(self) -> Transaction {
        let flow = convert_amount_to_flow(self.amount);
        Transaction {
            date: self.ledger_date,
            memo: self.text,
            outflow: flow.outflow,
            inflow: flow.inflow,
            ..Transaction::default()
        }
    }
}

impl Convertible for nordea::Transaction {
    fn to_ynab(self) -> Transaction {
        let flow = convert_amount_to_flow(self.amount);
        Transaction {
            date: self.timestamp,
            memo: self.title,
            outflow: flow.outflow,
            inflow: flow.inflow,
            ..Transaction::default()
        }
    }
}


#[derive(Default)]
struct Flow {
    pub inflow: f32,
    pub outflow: f32,
}

fn convert_amount_to_flow(amount: f32) -> Flow {
    if amount < 0.0 {
        Flow {
            outflow: amount.abs(),
            ..Flow::default()
        }
    } else {
        Flow {
            inflow: amount,
            ..Flow::default()
        }
    }
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::{handelsbanken, nordea};
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

    #[test]
    fn to_ynab() {
        // Given
        let nordea = nordea::Transaction {
            timestamp: NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(),
            amount: -5552.0,
            sender: "UNUSED".to_string(),
            receiver: "UNUSED".to_string(),
            name: "UNUSED".to_string(),
            title: "MEMO".to_string(),
            saldo: -5555.0,
            currency: "SEK".to_string(),
        };

        // When
        let row = nordea.to_ynab();

        // Then
        assert_eq!(NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(), row.date);
        assert_eq!("", row.payee);
        assert_eq!("", row.category);
        assert_eq!("MEMO", row.memo);
        assert_eq!(5552.0, row.outflow);
        assert_eq!(0.0, row.inflow);
    }
}
