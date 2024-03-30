pub use csv::Row;
pub use csv::write_csv;

mod csv;

pub trait Convertible {
    fn to_ynab(self) -> Row;
}

pub trait Parser {
    fn read_from_file(&self, file_path: String) -> anyhow::Result<Vec<impl Convertible>>;

    fn convert(&self, rows: Vec<impl Convertible>) -> anyhow::Result<Vec<Row>> {
        let ynab_rows = rows
            .into_iter() //
            .map(|convertible| convertible.to_ynab())
            .collect();

        Ok(ynab_rows)
    }

    fn parse(&self, file_path: String) -> anyhow::Result<Vec<Row>> {
        let convertible_rows = self.read_from_file(file_path)?;
        self.convert(convertible_rows)
    }
}
