use crate::ynab::Transaction;

pub trait Convertible {
    fn to_ynab(self) -> Transaction;
}
