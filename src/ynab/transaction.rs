use chrono::NaiveDate;

/// The format of a YNAB transaction CSV row
#[derive(serde::Serialize, Default)]
pub struct Transaction {
    #[serde(with = "ynab_date_format")]
    pub date: NaiveDate,
    pub payee: String,
    pub category: String,
    pub memo: String,
    pub outflow: f32,
    pub inflow: f32,
}

mod ynab_date_format {
    use chrono::NaiveDate;
    use serde::Serializer;

    const FORMAT: &str = "%Y/%m/%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }
}