pub use convertible::Convertible;
pub use csv::write_csv;
pub use transaction::Transaction;

use crate::nordea;

mod convertible;
mod csv;
mod transaction;

pub trait Parser {
    fn read_from_file(&self, file_path: String) -> anyhow::Result<Vec<impl Convertible>>;

    fn convert(&self, rows: Vec<impl Convertible>) -> anyhow::Result<Vec<Transaction>> {
        let ynab_rows = rows
            .into_iter() //
            .map(|convertible| convertible.to_ynab())
            .collect();

        Ok(ynab_rows)
    }

    fn parse(&self, file_path: String) -> anyhow::Result<Vec<Transaction>> {
        let convertible_rows = self.read_from_file(file_path)?;
        self.convert(convertible_rows)
    }
}

impl Parser for nordea::Parser {
    fn read_from_file(&self, file_path: String) -> anyhow::Result<Vec<impl Convertible>> {
        // Wraps nordea specific implementation
        self.read_csv(file_path)
    }
}
