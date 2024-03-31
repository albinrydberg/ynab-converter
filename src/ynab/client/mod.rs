use anyhow::Context;
use reqwest;

use schema::*;

mod schema;

const BASE_URL: &str = "https://api.ynab.com/v1/";

struct Client {
    inner: reqwest::blocking::Client,
    pat: String,
}

impl Client {
    pub fn new(pat: String) -> anyhow::Result<Self> {
        let client = reqwest::blocking::ClientBuilder::new().build()?;
        Ok(Self { inner: client, pat })
    }

    pub fn get_budgets(&self) -> anyhow::Result<BudgetSummaryResponse> {
        let response = self
            .inner
            .get(format!("{}/budgets", BASE_URL))
            .bearer_auth(&self.pat)
            .send()
            .with_context(|| "Failed to perform get budgets request")?;
        response
            .json::<BudgetSummaryResponse>()
            .with_context(|| "Failed to parse response as json")
    }
}
