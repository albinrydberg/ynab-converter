use crate::ynab::Convertible;
use csv::Writer;

pub struct YnabWriter {}

impl YnabWriter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write_csv(
        &self,
        file_name: String,
        records: Vec<impl Convertible>,
    ) -> anyhow::Result<()> {
        let mut writer = Writer::from_path(file_name)?;
        for record in records {
            let transaction = record.to_ynab();
            writer.serialize(transaction)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use chrono::NaiveDate;

    use crate::ynab;
    use crate::ynab::{Convertible, Transaction};

    struct TestConvertible {}

    impl Convertible for TestConvertible {
        fn to_ynab(self) -> Transaction {
            Transaction {
                date: NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(),
                payee: String::from("payee"),
                category: String::from("category"),
                memo: String::from("memo"),
                outflow: 100.0,
                inflow: 200.0,
            }
        }
    }

    #[test]
    fn write_csv() {
        // Given
        let writer = ynab::YnabWriter::new();

        let output_file = "testfiles/ynab-unit-test-output.csv";

        // When
        let result = writer.write_csv(String::from(output_file), vec![TestConvertible {}]);

        // Then
        assert!(result.is_ok());
        let file_contents = fs::read_to_string(output_file).unwrap();
        let expected = "date,payee,category,memo,outflow,inflow\n2024/03/28,payee,category,memo,100.0,200.0\n";
        assert_eq!(file_contents, expected);
    }
}
