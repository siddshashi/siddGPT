use std::error::Error;

mod helpers;

pub struct Data {
    pub train_data: Vec<u8>,
    pub val_data: Vec<u8>,
}

impl Data {
    pub fn parse(file: &str) -> Result<Data, Box<dyn Error>> {
        println!("parsing data...");

        let (train_data, val_data) = helpers::read_text(file)?;

        Ok(Data { train_data, val_data })
    }
}
