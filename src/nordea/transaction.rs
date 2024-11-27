use crate::{util, ynab};
use chrono::NaiveDate;

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Transaction {
    #[serde(rename = "Bokföringsdag", with = "nordea_ledger_date")]
    pub timestamp: NaiveDate,
    #[serde(rename = "Belopp", with = "util::swedish_float")]
    pub amount: f32,
    #[serde(rename = "Avsändare")]
    pub sender: String,
    #[serde(rename = "Mottagare")]
    pub receiver: String,
    #[serde(rename = "Namn")]
    pub name: String,
    #[serde(rename = "Rubrik")]
    pub title: String,
    #[serde(rename = "Saldo", with = "util::swedish_float")]
    pub saldo: f32,
    #[serde(rename = "Valuta")]
    pub currency: String,
}

impl ynab::Convertible for Transaction {
    fn to_ynab(self) -> ynab::Transaction {
        let flow = util::convert_amount_to_flow(self.amount);
        ynab::Transaction {
            date: self.timestamp,
            memo: self.title,
            outflow: flow.outflow,
            inflow: flow.inflow,
            ..ynab::Transaction::default()
        }
    }
}

mod nordea_ledger_date {
    use chrono::NaiveDate;
    use serde::de::Error;
    use serde::{Deserialize, Deserializer};

    const DATE_FMT: &str = "%Y-%m-%d";
    const DATE_FMT_2: &str = "%Y/%m/%d";
    const RESERVED_KEYWORD: &str = "Reserverat";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_string = String::deserialize(deserializer)?;

        if date_string.eq(RESERVED_KEYWORD) {
            return Err(Error::custom("Ledger date was 'reserved'"));
        }

        if let Ok(date) = NaiveDate::parse_from_str(&date_string, DATE_FMT) {
            return Ok(date);
        }

        if let Ok(date) = NaiveDate::parse_from_str(&date_string, DATE_FMT_2) {
            return Ok(date);
        }

        Err(Error::custom("Failed to deserialize ledger_date"))
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::nordea;
    use crate::ynab::Convertible;

    #[test]
    fn nordea_to_ynab() {
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
