use std::fs;

use html_parser::{Dom, Element, Node};

use crate::handelsbanken;
use crate::handelsbanken::table::*;

use log::info;

const HANDELSBANKEN_SHITTY_STRING: &str = " PUBLIC \"-//W3C//DTD HTML 4.01 Transitional//EN\"";

const HTML_TABLE_ROW_TAG: &str = "tr";
const HTML_DATA_CELL_TAG: &str = "td";

const HTML_NON_BREAKING_SPACE: &str = "&nbsp";

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    /// Parses a questionable handelsbanken XLS.
    pub fn read_xls(&self, input_file: String) -> anyhow::Result<Vec<handelsbanken::Transaction>> {
        let dom = read_dom_from_file(input_file)?;
        let rows = traverse_parse(dom.children);
        let rows = remove_residual_table_data(rows);

        let table = Table::from(rows);
        let output_vec = table.convert_to_rows();
        Ok(output_vec)
    }
}

/// Since we are unable to parse \<table> elements separately, (because of shitty input data)
/// we assume that all rows of the expected length pertain to the same table
fn remove_residual_table_data(rows: Vec<RawTableRow>) -> Vec<RawTableRow> {
    rows.into_iter()
        .filter(|row| row.is_handelsbanken_length())
        .collect::<Vec<RawTableRow>>()
}

fn read_dom_from_file(input_file: String) -> anyhow::Result<Dom> {
    let file_contents = fs::read(input_file)?;
    let input_file_string = String::from_utf8_lossy(&file_contents).to_string();
    let better = input_file_string.replace(HANDELSBANKEN_SHITTY_STRING, "");
    let dom = Dom::parse(better.as_str())?;
    Ok(dom)
}

/// Traverses a HTML node until a \<TR> element is found, which then will be parsed
///
/// TODO: Figure out how to do this without collecting on each call
fn traverse_parse(nodes: Vec<Node>) -> Vec<RawTableRow> {
    nodes //
        .into_iter()
        .flat_map(|row| match row {
            Node::Element(element) if is_table_row(&element) => {
                vec![parse_row(element)]
            }
            Node::Element(element) => traverse_parse(element.children),
            _ => Vec::new(),
        })
        .collect()
}

/// Takes a \<TR> HTML element, and converts all \<TD> cells into one RawTableRow struct
fn parse_row(element: Element) -> RawTableRow {
    let mut cells = Vec::new();
    for child in element.children {
        match child {
            Node::Element(data_cell) if is_data_cell(&data_cell) => match parse_cell(data_cell) {
                None => {}
                Some(cell) => cells.push(cell),
            },
            x => info!("Not a cell... Ignoring {:?}", x),
        }
    }
    RawTableRow::new(cells)
}

fn is_data_cell(element: &Element) -> bool {
    element.name.to_lowercase() == HTML_DATA_CELL_TAG
}

fn is_table_row(element: &Element) -> bool {
    element.name.to_lowercase() == HTML_TABLE_ROW_TAG
}

fn parse_cell(element: Element) -> Option<String> {
    if element.children.len() != 1 {
        panic!(
            "Found cell element with zero or multiple children: {:?}",
            element
        )
    }

    let cell_node = &element.children[0];

    match cell_node {
        Node::Text(text) if text.contains(HTML_NON_BREAKING_SPACE) => None,
        Node::Text(text) => Some(text.clone()),
        n => panic!("Found unexpected node: {:?}", n),
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::handelsbanken::xlsparser;
    use crate::handelsbanken::Transaction;

    #[test]
    fn read_xls_happy() {
        // Given
        let parser = xlsparser::Parser::new();

        // When
        let result = parser.read_xls("testfiles/handelsbanken-input.xls".to_string());
        println!("Result: {:?}", result);

        // Then
        assert!(result.is_ok(), "unexpected error: {:?}", result);

        let rows = result.unwrap();
        let expected = vec![
            Transaction {
                ledger_date: NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(),
                transaction_date: NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(),
                text: "HSB Göteborg".to_string(),
                amount: -5552.0,
                balance: Some(54057.92),
            },
            Transaction {
                ledger_date: NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(),
                transaction_date: NaiveDate::from_ymd_opt(2024, 3, 28).unwrap(),
                text: "HSB Göteborg".to_string(),
                amount: -450.0,
                balance: Some(59609.92),
            },
        ];

        assert_eq!(rows, expected)
    }
}
