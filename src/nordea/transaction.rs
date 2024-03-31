use chrono::NaiveDate;

#[derive(serde::Deserialize, Debug)]
pub struct Transaction {
    #[serde(rename = "Bokföringsdag")]
    pub timestamp: NaiveDate,
    #[serde(rename = "Belopp", with = "swedish_float")]
    pub amount: f32,
    #[serde(rename = "Avsändare")]
    pub sender: String,
    #[serde(rename = "Mottagare")]
    pub receiver: String,
    #[serde(rename = "Namn")]
    pub name: String,
    #[serde(rename = "Rubrik")]
    pub title: String,
    #[serde(rename = "Saldo", with = "swedish_float")]
    pub saldo: f32,
    #[serde(rename = "Valuta")]
    pub currency: String,
}

mod swedish_float {
    use serde::{Deserialize, Deserializer};
    use serde::de::Error;

    use crate::util;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let float_string = String::deserialize(deserializer)?;
        util::parse_swedish_float(float_string).map_err(Error::custom)
    }
}
