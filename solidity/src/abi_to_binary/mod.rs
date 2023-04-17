use serde_json::Value;
use std::io::Cursor;

pub fn abi_to_binary(abi: &Value) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut binary_buffer = Vec::new();
    let mut cursor = Cursor::new(&mut binary_buffer);
    serde_json::to_writer(&mut cursor, abi)?;

    Ok(binary_buffer)
}

pub fn binary_to_abi(binary: &[u8]) -> Result<Value, Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(binary);
    let abi = serde_json::from_reader(&mut cursor)?;

    Ok(abi)
}
