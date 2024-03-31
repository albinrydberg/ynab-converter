use std::num::ParseFloatError;

pub fn parse_swedish_float(string: String) -> Result<f32, ParseFloatError> {
    string //
        .replace(',', ".")
        .replace(' ', "")
        .parse::<f32>()
}
