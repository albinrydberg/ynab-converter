pub use transaction::Transaction;

mod transaction;

const DELIMITER: u8 = b';';

pub struct Parser {
    csv_parser: crate::csv::Parser,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            csv_parser: crate::csv::Parser::new(DELIMITER),
        }
    }

    pub fn parse(&self, input_file: &String) -> anyhow::Result<Vec<Transaction>> {
        self.csv_parser.read_csv(input_file)
    }

    pub fn is_parsable(&self, file_name: &String) -> bool {
        self.csv_parser.is_parsable::<Transaction>(file_name)
    }
}

#[cfg(test)]
mod tests {
    use crate::nordea::Parser;
    use crate::nordea::Transaction;
    use chrono::NaiveDate;

    #[test]
    fn read_csv() {
        // Given
        let parser = Parser::new();

        // When
        let result = parser.parse(
            //
            &String::from("testfiles/nordea-test-input.csv"),
        );

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
        let parser = Parser::new();

        // When
        let result = parser.parse(
            //
            &String::from("testfiles/nordea-input-2024.csv"),
        );

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
        let parser = Parser::new();

        // When
        let result = parser.parse(
            //
            &String::from("testfiles/nordea-reserverat.csv"),
        );

        // Then
        assert!(result.is_ok(), "Error was {:?}", result);
        let csv_vec = result.unwrap();

        assert!(csv_vec.is_empty());
    }
}
