use chrono::NaiveDate;
use crate::{util, ynab};

#[derive(serde::Deserialize, Debug)]
pub struct Row {
    #[serde(rename = "Bokföringsdag")]
    pub timestamp: NaiveDate,
    #[serde(rename = "Belopp", with = "float_with_comma")]
    pub amount: f32,
    #[serde(rename = "Avsändare")]
    pub sender: String,
    #[serde(rename = "Mottagare")]
    pub receiver: String,
    #[serde(rename = "Namn")]
    pub name: String,
    #[serde(rename = "Rubrik")]
    pub title: String,
    #[serde(rename = "Saldo", with = "float_with_comma")]
    pub saldo: f32,
    #[serde(rename = "Valuta")]
    pub currency: String,
}

impl ynab::Convertible for Row {
    fn to_ynab(self) -> ynab::Row {
        let flow = util::convert_amount_to_flow(self.amount);
        ynab::Row {
            date: self.timestamp,
            memo: self.title,
            outflow: flow.outflow,
            inflow: flow.inflow,
            ..ynab::Row::default()
        }
    }
}

pub struct NordeaParser;

impl ynab::Parser for NordeaParser {
    fn read_from_file(&self, file_path: String) -> anyhow::Result<Vec<impl ynab::Convertible>> {
        read_csv(file_path)
    }
}

pub fn new_parser() -> NordeaParser {
    NordeaParser{}
}

mod float_with_comma {
    use serde::{Deserialize, Deserializer};
    use serde::de::Error;

    use crate::util;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let float_string = String::deserialize(deserializer)?;
        util::parse_swedish_float(float_string).map_err(Error::custom)
    }
}

fn read_csv(file_name: String) -> anyhow::Result<Vec<Row>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(file_name)?;

    let mut result = Vec::new();
    for record in reader.deserialize() {
        let row: Row = record?;
        result.push(row)
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::nordea::Row;

    use crate::nordea;
    use crate::ynab::Convertible;

    #[test]
    fn read_csv() {
        // When
        let result = nordea::read_csv(String::from("testfiles/nordea-test-input.csv"));

        // Then
        assert!(result.is_ok(), "Error was {:?}", result);
        let csv_vec = result.unwrap();
        assert_eq!(1, csv_vec.len());

        let row = &csv_vec[0];
        assert_eq!(
            row.timestamp,
            NaiveDate::from_ymd_opt(2021, 11, 30).unwrap()
        );
        assert_eq!(row.amount, -34.0);
        assert_eq!(row.sender, "1234 00 56789");
        assert_eq!(row.receiver, "");
        assert_eq!(row.name, "");
        assert_eq!(row.title, "Swish betalning VÄSTTRAFIK AB");
        assert_eq!(row.saldo, 39309.72);
        assert_eq!(row.currency, "SEK");
    }

    #[test]
    fn to_ynab() {
        // Given
        let original_row = Row{
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
        let row = original_row.to_ynab();

        // Then
        assert_eq!(NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(), row.date);
        assert_eq!("", row.payee);
        assert_eq!("", row.category);
        assert_eq!("MEMO", row.memo);
        assert_eq!(5552.0, row.outflow);
        assert_eq!(0.0, row.inflow);
    }
}
