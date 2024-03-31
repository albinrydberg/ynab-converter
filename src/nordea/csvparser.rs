use crate::nordea::Transaction;

pub struct Parser;

pub fn new_parser() -> Parser {
    Parser {}
}

impl Parser {
    pub fn read_csv(&self, file_name: String) -> anyhow::Result<Vec<Transaction>> {
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
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::nordea::csvparser;

    #[test]
    fn read_csv() {
        // Given
        let parser = csvparser::new_parser();

        // When
        let result = parser.read_csv(String::from("testfiles/nordea-test-input.csv"));

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
