mod transaction;

pub use transaction::Transaction;

const DELIMITER: u8 = b',';

pub struct Parser {
    csv_parser: crate::csv::Parser,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            csv_parser: crate::csv::Parser::new(DELIMITER),
        }
    }

    pub fn parse(&self, file_name: &String) -> anyhow::Result<Vec<Transaction>> {
        self.csv_parser.read_csv(file_name)
    }

    pub fn is_parsable(&self, file_name: &String) -> bool {
        self.csv_parser.is_parsable::<Transaction>(file_name)
    }
}

#[cfg(test)]
mod tests {
    use crate::amex::*;
    use chrono::NaiveDate;

    #[test]
    fn read_csv_big() {
        // Given
        let parser = Parser::new();

        // When
        let result = parser.parse(
            //
            &String::from("testfiles/amex-test-input.csv"),
        );

        // Then
        assert!(result.is_ok(), "Error was {:?}", result);
        let csv_vec = result.unwrap();

        let expected = vec![Transaction {
            date: NaiveDate::from_ymd_opt(2024, 11, 20).unwrap(),
            description: "Beskrivning av köpet".to_string(),
            amount: 109.05,
        }];
        assert_eq!(csv_vec, expected);
    }

    #[test]
    fn read_csv_small() {
        // Given
        let parser = Parser::new();

        // When
        let result = parser.parse(
            //
            &String::from("testfiles/amex-test-input-small.csv"),
        );

        // Then
        assert!(result.is_ok(), "Error was {:?}", result);
        let csv_vec = result.unwrap();

        let expected = vec![Transaction {
            date: NaiveDate::from_ymd_opt(2024, 11, 20).unwrap(),
            description: "Beskrivning av köpet".to_string(),
            amount: 109.05,
        }];
        assert_eq!(csv_vec, expected);
    }

    #[test]
    fn read_csv_broken() {
        // Given
        let parser = Parser::new();

        // When
        let result = parser.parse(
            //
            &String::from("testfiles/amex-test-input-smallest.csv"),
        );

        // Then
        assert!(result.is_ok(), "Error was {:?}", result);
        let csv_vec = result.unwrap();

        let expected = vec![
            Transaction {
                date: NaiveDate::from_ymd_opt(2024, 12, 01).unwrap(),
                description: "Ica Kvantum".to_string(),
                amount: 286.49,
            },
            Transaction {
                date: NaiveDate::from_ymd_opt(2024, 11, 22).unwrap(),
                description: "Mat".to_string(),
                amount: 332.00,
            },
        ];
        assert_eq!(csv_vec, expected);
    }

    #[test]
    fn is_parsable() {
        // Given
        let parser = Parser::new();

        // When
        let result = parser.is_parsable(&String::from("testfiles/amex-test-input-small.csv"));

        // Then
        assert_eq!(result, true)
    }

    #[test]
    fn is_parsable_negative() {
        // Given
        let parser = Parser::new();

        // When
        let result = parser.is_parsable(&String::from("testfiles/nordea_input_2024.csv"));

        // Then
        assert_eq!(result, false)
    }
}
