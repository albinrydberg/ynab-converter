use csv::Writer;

use super::transaction::Transaction;

pub fn write_csv(file_name: String, records: Vec<Transaction>) -> anyhow::Result<()> {
    let mut writer = Writer::from_path(file_name)?;
    for record in records {
        writer.serialize(record)?
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use chrono::NaiveDate;

    use crate::ynab;
    use crate::ynab::Transaction;

    #[test]
    fn write_csv() {
        // Given
        let row = Transaction {
            date: NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(),
            payee: String::from("payee"),
            category: String::from("category"),
            memo: String::from("memo"),
            outflow: 100.0,
            inflow: 200.0,
        };
        let row2 = Transaction {
            date: NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(),
            payee: String::from("payee2"),
            category: String::from("category2"),
            memo: String::from("memo2"),
            outflow: 1000.0,
            inflow: 2000.0,
        };

        let output_file = "testfiles/ynab-unit-test-output.csv";

        // When
        let result = ynab::write_csv(String::from(output_file), vec![row, row2]);

        // Then
        assert!(result.is_ok());
        let file_contents = fs::read_to_string(output_file).unwrap();
        let expected = "date,payee,category,memo,outflow,inflow\n2024/03/28,payee,category,memo,100.0,200.0\n2024/03/28,payee2,category2,memo2,1000.0,2000.0\n";
        assert_eq!(file_contents, expected);
    }
}
