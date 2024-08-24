use calamine;
use calamine::{Data, DataType, Reader, Xlsx};

use crate::handelsbanken;
use crate::handelsbanken::table::{RawTableRow, Table};

pub struct Parser;

const XLSX_DEFAULT_SHEET: &str = "Sheet1";

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_xls(&self, input_file: String) -> anyhow::Result<Vec<handelsbanken::Transaction>> {
        let mut workbook: Xlsx<_> = calamine::open_workbook(input_file)?;

        let rows: Vec<RawTableRow> = workbook.worksheet_range(XLSX_DEFAULT_SHEET)?
            .rows()
            .map(parse_row)
            .filter(RawTableRow::is_handelsbanken_length)
            .collect();

        let table = Table::from(rows);
        let output_vec = table.convert_to_rows();

        Ok(output_vec)
    }
}

fn parse_row(row: &[Data]) -> RawTableRow {
    let cells = row.iter()
        .map(Data::as_string)
        .flatten()
        .collect::<Vec<String>>();
    RawTableRow::new(cells)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::handelsbanken::{Transaction, xlsxparser};

    #[test]
    fn read_xlsx_happy() {
        // Given
        let parser = xlsxparser::Parser::new();

        // When
        let result = parser.read_xls("testfiles/handelsbanken-input.xlsx".to_string());
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
}