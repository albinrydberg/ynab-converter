use crate::handelsbanken;
use crate::handelsbanken::{xlsparser, xlsxparser};

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read(&self, input_file: String) -> anyhow::Result<Vec<handelsbanken::Transaction>> {
        let output = xlsxparser::Parser::new().read_xls(input_file.clone());

        match output {
            Ok(x) => Ok(x),
            Err(err) => {
                println!("Failed to parse file: {:?} with err {:?}, trying V1 parser", input_file, err);
                xlsparser::Parser::new().read_xls(input_file)
            }
        }
    }
}