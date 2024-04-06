use chrono::NaiveDate;

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Transaction {
    #[serde(rename = "Bokföringsdag", with = "nordea_date")]
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

mod nordea_date {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer};
    use serde::de::Error;

    const DATE_FMT: &str = "%Y-%m-%d";
    const DATE_FMT_2: &str = "%Y/%m/%d";
    const RESERVED_KEYWORD: &str = "Reserverat";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_string = String::deserialize(deserializer)?;
        
        if date_string.eq(RESERVED_KEYWORD) {
            return Err(Error::custom("Ledger date was 'reserved'"));
        }
        
        NaiveDate::parse_from_str(&date_string, DATE_FMT)
            .or_else(|_| NaiveDate::parse_from_str(&date_string, DATE_FMT_2).map_err(Error::custom))
    }
}
