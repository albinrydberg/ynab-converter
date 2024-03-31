use chrono::{NaiveDate, NaiveDateTime};

#[derive(serde::Deserialize)]
pub struct BudgetSummaryResponse {
    data: Data,
}

#[derive(serde::Deserialize)]
pub struct Data {
    budgets: Vec<BudgetSummary>,
}

#[derive(serde::Deserialize)]
pub struct BudgetSummary {
    id: String,
    name: String,
    last_modified_on: NaiveDateTime,
    first_month: NaiveDate,
    last_month: NaiveDate,
    date_format: DateFormat,
    currency_format: CurrencyFormat,
}

#[derive(serde::Deserialize)]
pub struct DateFormat {
    format: String,
}

#[derive(serde::Deserialize)]
pub struct CurrencyFormat {
    iso_code: String,
    example_format: String,
    decimal_digits: usize,
    decimal_separator: char,
    symbol_first: bool,
    group_separator: char,
    currency_symbol: String,
    display_symbol: bool,
}
