use std::{str::from_utf8, path::PathBuf};
use std::process::Command;
use serde_json::{Value, from_str};
use tracing::info;

pub fn compile_and_extract_abi(file_path: &PathBuf) -> Result<Value, Box<dyn std::error::Error>> {
    let output = Command::new("solc")
        .arg("--combined-json")
        .arg("abi")
        .arg(file_path)
        .output()?;

    println!("{:?}", output);

    info!(status=?output, "COMPILE STATUS");

    let output_str = from_utf8(&output.stdout)?;
    info!(output_str=?output_str, "OUTPUT STR");

    println!("");
    let output_json: Value = from_str(output_str)?;
    info!(output_json=?output_json, "OUTPUT JSON");

    let abi = output_json["contracts"]["tests/fixtures/token/ERC20/IERC20.sol:IERC20"]["abi"].clone();

    println!("");
    info!(abi=?abi, "ABI");

    Ok(abi)
}
