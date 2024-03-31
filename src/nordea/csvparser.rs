use crate::nordea::Transaction;
use crate::ynab;

pub struct NordeaParser;

impl ynab::Parser for NordeaParser {
    fn read_from_file(&self, file_path: String) -> anyhow::Result<Vec<impl ynab::Convertible>> {
        read_csv(file_path)
    }
}

pub fn new_parser() -> NordeaParser {
    NordeaParser {}
}

fn read_csv(file_name: String) -> anyhow::Result<Vec<Transaction>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(file_name)?;

    let mut result = Vec::new();
    for record in reader.deserialize() {
        let row: Transaction = record?;
        result.push(row)
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::nordea;

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
}
