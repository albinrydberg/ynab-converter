use chrono::NaiveDate;

use crate::{handelsbanken, util};

pub(super) const KNOWN_HANDELSBANKEN_HEADERS: [&str; 5] = [
    "Reskontradatum",
    "Transaktionsdatum",
    "Text",
    "Belopp",
    "Saldo",
];

const DATE_FMT: &str = "%Y-%m-%d";

#[derive(Debug, Clone)]
pub(super) struct Table {
    rows: Vec<RawTableRow>,
}

impl Table {
    pub(super) fn from(rows: Vec<RawTableRow>) -> Self {
        if rows.is_empty() {
            panic!("Can't create a table from empty input: {:?}", rows)
        }

        for row in &rows {
            if row.cells.len() != KNOWN_HANDELSBANKEN_HEADERS.len() {
                panic!(
                    "Can't create a table with rows of varying breadth: {:?}",
                    rows
                )
            }
        }
        Self { rows }
    }

    pub(super) fn to_rows(&self) -> Vec<handelsbanken::Row> {
        let _header_row = &self.rows[0]; // TODO: Decide whether to do something with this
        let data_rows = &self.rows[1..];

        data_rows.iter().map(Self::convert_row).collect()
    }

    fn convert_row(row: &RawTableRow) -> handelsbanken::Row {
        let ledger_date = NaiveDate::parse_from_str(&row.cells[0], DATE_FMT).unwrap();
        let transaction_date = NaiveDate::parse_from_str(&row.cells[1], DATE_FMT).unwrap();
        let text = row.cells[2].clone();
        let amount = util::parse_swedish_float(row.cells[3].to_string()).unwrap();
        let balance = match util::parse_swedish_float(row.cells[4].to_string()) {
            Ok(balance) => Some(balance),
            Err(_) => None,
        };

        handelsbanken::Row {
            ledger_date,
            transaction_date,
            text,
            amount,
            balance,
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct RawTableRow {
    cells: Vec<String>,
}

impl RawTableRow {
    pub(super) fn new(cells: Vec<String>) -> Self {
        Self { cells }
    }

    pub(super) fn len(&self) -> usize {
        self.cells.len()
    }
}
