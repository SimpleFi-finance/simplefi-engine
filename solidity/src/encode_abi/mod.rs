use std::io::Read;
use std::io::Write;
use std::io::Cursor;

use serde_json::Value;
use flate2::Compression;
use flate2::write::{ZlibEncoder, ZlibDecoder};

pub fn encode_abi(abi: &Value) -> Result<String, Box<dyn std::error::Error>> {
// pub fn encode_abi(abi: &Value) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    // encoder.write_all(abi.as_)
    encoder.write_all(abi.to_string().as_bytes())?;

    let compressed_data = encoder.finish()?;

    /* Ok(compressed_data) */

    let mut decoder = ZlibDecoder::new(Cursor::new(compressed_data));

    let mut decompressed_data = String::new();

    decoder.read_to_string(&mut decompressed_data)?;



    println!("decompressed data: {}", decompressed_data);

    Ok(decompressed_data)
}
