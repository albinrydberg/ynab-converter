use chrono::{NaiveDate, NaiveDateTime};

#[derive(serde::Deserialize, Debug)]
pub struct BudgetSummaryResponse {
    data: Data,
}

#[derive(serde::Deserialize, Debug)]
pub struct Data {
    budgets: Vec<BudgetSummary>,
}

#[derive(serde::Deserialize, Debug)]
pub struct BudgetSummary {
    id: String,
    name: String,
    last_modified_on: String, // NaiveDateTime
    first_month: String, // NaiveDate
    last_month: String, // NaiveDate
    date_format: DateFormat,
    currency_format: CurrencyFormat,
}

#[derive(serde::Deserialize, Debug)]
pub struct DateFormat {
    format: String,
}

#[derive(serde::Deserialize, Debug)]
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

#[cfg(test)]
mod tests {
    use crate::ynab::client::schema::BudgetSummaryResponse;

    #[test]
    fn serialization() {
        let input = "{\"data\":{\"budgets\":[{\"id\":\"14232323-336e-4646-90e6-dfdaa1ed5516\",\"name\":\"Budget\",\"last_modified_on\":\"2024-04-06T14:55:09Z\",\"first_month\":\"2018-09-01\",\"last_month\":\"2024-04-01\",\"date_format\":{\"format\":\"YYYY-MM-DD\"},\"currency_format\":{\"iso_code\":\"SEK\",\"example_format\":\"123.456,78\",\"decimal_digits\":2,\"decimal_separator\":\",\",\"symbol_first\":false,\"group_separator\":\".\",\"currency_symbol\":\"kr\",\"display_symbol\":true}}]}}";
        let result = serde_json::from_str::<BudgetSummaryResponse>(input);
        match result {
            Err(e) => println!("{:?}", e),
            Ok(content) => println!("{:?}", content),
        }
        
    }
}

