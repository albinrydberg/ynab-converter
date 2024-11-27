use serde::de::DeserializeOwned;

pub struct Parser {
    delimiter: u8,
}

impl Parser {
    pub fn new(delimiter: u8) -> Self {
        Self { delimiter }
    }

    pub fn is_parsable<T: DeserializeOwned>(&self, file_name: &String) -> bool {
        let result = csv::ReaderBuilder::new()
            .delimiter(self.delimiter)
            .from_path(file_name);

        if let Ok(mut reader) = result {
            if let Some(Ok(_)) = reader.deserialize::<T>().next() {
                return true;
            }
        }

        false
    }

    pub fn read_csv<T: DeserializeOwned>(&self, file_name: &String) -> anyhow::Result<Vec<T>> {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(self.delimiter)
            .from_path(file_name)?;

        let mut result = Vec::new();
        for record in reader.deserialize() {
            let row: T = match record {
                Ok(record) => record,
                Err(e) => {
                    println!("Skipping record: {:?}", e);
                    continue;
                }
            };
            result.push(row)
        }
        Ok(result)
    }
}
