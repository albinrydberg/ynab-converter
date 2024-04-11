use anyhow::Context;
use reqwest;

use schema::*;

mod schema;

pub struct Client {
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
            .get("https://api.ynab.com/v1/budgets")
            .bearer_auth(&self.pat)
            .send()
            .with_context(|| "Failed to perform get budgets request")?;

        println!("{:?}", response);
        response
            .json::<BudgetSummaryResponse>()
            .with_context(|| "Failed to parse response as json")
    }
}
