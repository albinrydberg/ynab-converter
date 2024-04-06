use crate::nordea::Transaction;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_csv(&self, file_name: String) -> anyhow::Result<Vec<Transaction>> {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_path(file_name)?;

        let mut result = Vec::new();
        for record in reader.deserialize() {
            let row: Transaction = match record {
                Ok(record) => record,
                Err(e) => {
                    println!("Skipping record: {:?}", e);
                    continue;
                }
            };
            result.push(row)
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::nordea::{csvparser, Transaction};

    #[test]
    fn read_csv() {
        // Given
        let parser = csvparser::Parser::new();

        // When
        let result = parser.read_csv(String::from("testfiles/nordea-test-input.csv"));

        // Then
        assert!(result.is_ok(), "Error was {:?}", result);
        let csv_vec = result.unwrap();

        let expected = vec![Transaction {
            timestamp: NaiveDate::from_ymd_opt(2021, 11, 30).unwrap(),
            amount: -34.0,
            sender: "1234 00 56789".to_string(),
            receiver: "".to_string(),
            name: "".to_string(),
            title: "Swish betalning VÄSTTRAFIK AB".to_string(),
            saldo: 39309.72,
            currency: "SEK".to_string(),
        }];
        assert_eq!(csv_vec, expected);
    }

    #[test]
    fn read_csv_2024() {
        // Given
        let parser = csvparser::Parser::new();

        // When
        let result = parser.read_csv(String::from("testfiles/nordea-input-2024.csv"));

        // Then
        assert!(result.is_ok(), "Error was {:?}", result);
        let csv_vec = result.unwrap();

        let expected = vec![
            Transaction {
                timestamp: NaiveDate::from_ymd_opt(2024, 4, 5).unwrap(),
                amount: -3000.0,
                sender: "1234 00 56789".to_string(),
                receiver: "".to_string(),
                name: "".to_string(),
                title: "Överföring 1234 00 56789".to_string(),
                saldo: 33245.79,
                currency: "SEK".to_string(),
            },
            Transaction {
                timestamp: NaiveDate::from_ymd_opt(2024, 4, 4).unwrap(),
                amount: -130.0,
                sender: "1234 00 56789".to_string(),
                receiver: "".to_string(),
                name: "".to_string(),
                title: "Kortköp 240403 KOOPERATIVET LINDHOL".to_string(),
                saldo: 36245.79,
                currency: "SEK".to_string(),
            },
        ];
        assert_eq!(csv_vec, expected);
    }

    #[test]
    fn read_csv_reserved() {
        // Given
        let parser = csvparser::Parser::new();

        // When
        let result = parser.read_csv(String::from("testfiles/nordea-reserverat.csv"));

        // Then
        assert!(result.is_ok(), "Error was {:?}", result);
        let csv_vec = result.unwrap();

        assert!(csv_vec.is_empty());
    }
}
