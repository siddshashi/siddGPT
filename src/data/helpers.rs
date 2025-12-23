use std::fs;
use std::error::Error;

pub(super) fn read_text(file: &str) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let bytes = fs::read(file)?;

    let split = (bytes.len() * 9) / 10;

    let train = bytes[..split].to_vec();
    let val = bytes[split..].to_vec();

    Ok((train, val))
}
