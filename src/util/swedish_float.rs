use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::num::ParseFloatError;

pub fn deserialize<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let float_string = String::deserialize(deserializer)?;
    parse_swedish_float(float_string).map_err(Error::custom)
}

pub fn parse_swedish_float(string: String) -> Result<f32, ParseFloatError> {
    string //
        .replace(',', ".")
        .replace(' ', "")
        .parse::<f32>()
}
