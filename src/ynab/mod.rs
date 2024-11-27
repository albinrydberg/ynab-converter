pub use csv::YnabWriter;
pub use transaction::Transaction;

mod csv;
mod transaction;

pub trait Convertible {
    fn to_ynab(self) -> Transaction;
}
