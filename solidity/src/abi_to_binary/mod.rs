use serde_json::Value;
use std::io::Cursor;

pub fn abi_to_binary(abi: &Value) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut binary_buffer = Vec::new();
    let mut cursor = Cursor::new(&mut binary_buffer);
    serde_json::to_writer(&mut cursor, abi)?;

    Ok(binary_buffer)
}
