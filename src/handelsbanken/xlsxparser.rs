use crate::handelsbanken;
use crate::handelsbanken::table::{RawTableRow, Table};
use calamine;
use calamine::{Data, DataType, Reader, Xlsx};

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_xlsx(&self, input_file: String) -> anyhow::Result<Vec<handelsbanken::Transaction>> {
        let mut workbook: Xlsx<_> = calamine::open_workbook(input_file)?;

        let rows: Vec<RawTableRow> = workbook
            .worksheets()
            .into_iter()
            .next()
            .map(|map| map.1)
            .ok_or(anyhow::Error::msg("No sheets found"))?
            .rows()
            .map(parse_row)
            .filter(RawTableRow::is_handelsbanken_length)
            .collect();

        let table = Table::from(rows)?;
        let output_vec = table.convert_to_rows();

        Ok(output_vec)
    }
}

fn parse_row(row: &[Data]) -> RawTableRow {
    let cells = row
        .iter()
        .map(Data::as_string)
        .flatten()
        .collect::<Vec<String>>();
    RawTableRow::new(cells)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::handelsbanken::{xlsxparser, Transaction};

    #[test]
    fn read_xlsx_happy() {
        // Given
        let parser = xlsxparser::Parser::new();

        // When
        let result = parser.read_xlsx("testfiles/handelsbanken-input.xlsx".to_string());
        println!("Result: {:?}", result);

        // Then
        assert!(result.is_ok(), "unexpected error: {:?}", result);

        let rows = result.unwrap();
        let expected = vec![
            Transaction {
                ledger_date: NaiveDate::from_ymd_opt(2024, 8, 23).unwrap(),
                transaction_date: NaiveDate::from_ymd_opt(2024, 8, 23).unwrap(),
                text: "ins".to_string(),
                amount: 2000.0,
                balance: Some(9634.81),
            },
            Transaction {
                ledger_date: NaiveDate::from_ymd_opt(2024, 8, 22).unwrap(),
                transaction_date: NaiveDate::from_ymd_opt(2024, 8, 22).unwrap(),
                text: "övern disp".to_string(),
                amount: -140.0,
                balance: Some(7634.81),
            },
        ];

        assert_eq!(rows, expected)
    }

    #[test]
    fn read_xlsx_bug_2026() {
        // Given
        let parser = xlsxparser::Parser::new();

        // When
        let result = parser.read_xlsx("testfiles/handelsbanken-bug-2026.xlsx".to_string());
        println!("Result: {:?}", result);

        // Then
        assert!(result.is_ok(), "unexpected error: {:?}", result);

        let rows = result.unwrap();
        let expected = vec![
            Transaction {
                ledger_date: NaiveDate::from_ymd_opt(2026, 1, 26).unwrap(),
                transaction_date: NaiveDate::from_ymd_opt(2026, 1, 26).unwrap(),
                text: "sdf".to_string(),
                amount: -5000.0,
                balance: Some(1.0),
            },
            Transaction {
                ledger_date: NaiveDate::from_ymd_opt(2026, 1, 23).unwrap(),
                transaction_date: NaiveDate::from_ymd_opt(2026, 1, 23).unwrap(),
                text: "sdf".to_string(),
                amount: 22400.0,
                balance: Some(2.0),
            },
            Transaction {
                ledger_date: NaiveDate::from_ymd_opt(2026, 1, 20).unwrap(),
                transaction_date: NaiveDate::from_ymd_opt(2026, 1, 20).unwrap(),
                text: "sdf".to_string(),
                amount: -527.0,
                balance: Some(3.0),
            },
        ];

        assert_eq!(rows, expected)
    }
}
