use crate::ynab::Convertible;
use crate::{handelsbanken, util};
use chrono::NaiveDate;

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Transaction {
    #[serde(rename = "Datum", with = "amex_date")]
    pub date: NaiveDate,
    #[serde(rename = "Beskrivning")]
    pub description: String,
    #[serde(rename = "Kortmedlem")]
    pub card_holder: String,
    #[serde(rename = "Konto #")]
    pub card_number: String,
    #[serde(rename = "Belopp", with = "util::swedish_float")]
    pub amount: f32,
}

impl Convertible for handelsbanken::Transaction {
    fn to_ynab(self) -> crate::ynab::Transaction {
        let flow = util::convert_amount_to_flow(self.amount);
        crate::ynab::Transaction {
            date: self.ledger_date,
            memo: self.text,
            outflow: flow.outflow,
            inflow: flow.inflow,
            ..crate::ynab::Transaction::default()
        }
    }
}

mod amex_date {
    use chrono::NaiveDate;
    use serde::de::Error;
    use serde::{Deserialize, Deserializer};

    const DATE_FMT: &str = "%m-%d-%Y";
    const DATE_FMT_2: &str = "%m/%d/%Y";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_string = String::deserialize(deserializer)?;

        if let Ok(date) = NaiveDate::parse_from_str(&date_string, DATE_FMT) {
            return Ok(date);
        }

        if let Ok(date) = NaiveDate::parse_from_str(&date_string, DATE_FMT_2) {
            return Ok(date);
        }

        Err(Error::custom("Failed to deserialize date"))
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::amex;
    use crate::ynab::Convertible;

    #[test]
    fn amex_to_ynab() {
        // Given
        let amex = amex::Transaction {
            date: NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(),
            description: "Description".to_string(),
            card_holder: "Name".to_string(),
            amount: 5552.0,
            card_number: "12345".to_string(),
        };

        // When
        let row = amex.to_ynab();

        // Then
        assert_eq!(NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(), row.date);
        assert_eq!("", row.payee);
        assert_eq!("", row.category);
        assert_eq!("Description", row.memo);
        assert_eq!(5552.0, row.outflow);
        assert_eq!(0.0, row.inflow);
    }
}
