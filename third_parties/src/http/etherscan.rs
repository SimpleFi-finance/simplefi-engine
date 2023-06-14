use http::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use log::{debug, error, info};
use reqwest::{Error, StatusCode};
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;

use crate::http::fetch::fetch;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct EtherscanResponse {
    status: String,
    message: String,
    result: String,
}

/// Fetch contract ABI from Etherscan
///
/// # Arguments
///
/// * `contract_address` - The contract address to fetch the ABI from
/// * `api_key` - The Etherscan API key
///
/// # Returns
///
/// * `Result<Option<Value>, Error>` - The contract ABI
/// * `None` if the contract ABI could not be fetched
/// * `Some(Value)` if the contract ABI was fetched successfully
/// * `Error` if there was an error fetching the contract ABI
///
/// # ToDo
///
/// Change hardcoded etherescan url to a config value
///
///
pub async fn get_abi(
    contract_address: &str,
    api_key: &str,
) -> Result<String, Error> {
    // Base url
    let etherscan_api_url = "https://api.etherscan.io/api";

    // generate query params
    let mut query_params = HashMap::new();
    query_params.insert("module".to_string(), "contract".to_string());
    query_params.insert("action".to_string(), "getabi".to_string());
    query_params.insert("address".to_string(), contract_address.to_string());
    query_params.insert("apikey".to_string(), api_key.to_string());

    debug!("query_params: {:?}", query_params);

    // Add custom headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let result = fetch(etherscan_api_url, query_params, Some(headers), None).await?;

    match result.status() {
        StatusCode::OK => {
            let results: EtherscanResponse = result.json().await?;

            debug!("result: {:?}", results.result.len());

            Ok(results.result)
        }
        _ => {
            error!("Error: {}", result.status());

            Ok("".to_string())
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct SourceCodeMessage {
    #[serde(rename = "SourceCode")]
    source_code: String,
    #[serde(rename = "ABI")]
    abi: String,
    #[serde(rename = "ContractName")]
    contract_name: String,
    #[serde(rename = "CompilerVersion")]
    compiler_version: String,
    #[serde(rename = "OptimizationUsed")]
    optimization_used: String,
    #[serde(rename = "Runs")]
    runs: String,
    #[serde(rename = "ConstructorArguments")]
    constructor_arguments: String,
    #[serde(rename = "EVMVersion")]
    evm_version: String,
    #[serde(rename = "Library")]
    library: String,
    #[serde(rename = "LicenseType")]
    license_type: String,
    #[serde(rename = "Proxy")]
    proxy: String,
    #[serde(rename = "Implementation")]
    implementation: String,
    #[serde(rename = "SwarmSource")]
    swarm_source: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SourceCodeEtherscanResponse {
    status: String,
    message: String,
    result: Vec<SourceCodeMessage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SourceCodeResponse {
    pub abi: String,
    pub contract_name: String,
    pub implementation: String,
    pub proxy: String,
}

pub async fn get_source_code(
    contract_address: &str,
    api_key: &str,
) -> Result<SourceCodeResponse, Error> {
    // Base url
    let etherscan_api_url = "https://api.etherscan.io/api";

    // generate query params
    let mut query_params = HashMap::new();
    query_params.insert("module".to_string(), "contract".to_string());
    query_params.insert("action".to_string(), "getsourcecode".to_string());
    query_params.insert("address".to_string(), contract_address.to_string());
    query_params.insert("apikey".to_string(), api_key.to_string());

    debug!("query_params: {:?}", query_params);

    // Add custom headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let response = fetch(etherscan_api_url, query_params, Some(headers), None).await?;

    match response.status() {
        StatusCode::OK => {
            let etherscan_response = response.json::<SourceCodeEtherscanResponse>().await?;

            info!("result: {:?}", etherscan_response.result.len());

            if etherscan_response.result.len() > 0 {
                let source_code = etherscan_response.result[0].clone();

                return Ok(SourceCodeResponse {
                    abi: source_code.abi,
                    contract_name: source_code.contract_name,
                    implementation: source_code.implementation,
                    proxy: source_code.proxy,
                });
            }

            Ok(SourceCodeResponse {
                abi: "".to_string(),
                contract_name: "".to_string(),
                implementation: "".to_string(),
                proxy: "".to_string(),
            })
        }
        _ => {
            error!("Error: {}", response.status());

            Ok(SourceCodeResponse {
                abi: "".to_string(),
                contract_name: "".to_string(),
                implementation: "".to_string(),
                proxy: "".to_string(),
            })
        }
    }
}

fn extract_info(input: &str) -> (Vec<&str>, &str) {
    let re_addr = Regex::new(r"0x[a-fA-F0-9]{40}").unwrap();
    let re_standard = Regex::new(r"EIP-\d+").unwrap();
    let mut addresses = vec![];

    for cap in re_addr.captures_iter(input) {
        let address = cap.get(0).map_or("", |m| m.as_str());



        if addresses.contains(&address) {
            continue;
        }

        addresses.push(cap.get(0).map_or("", |m| m.as_str()));
    }

    let standard = re_standard.find(input).map_or("", |m| m.as_str());

    (addresses, standard)
}

pub async fn get_previous_implementation(
    contract_address: &str,
) -> Result<(), Error> {
    let etherscan_api_url = format!("https://etherscan.io/address/{}#readProxyContract", contract_address);

    let response = reqwest::get(etherscan_api_url).await?.text().await?    ;

    let re = Regex::new(r#"<span id="ContentPlaceHolder1_readProxyMessage">(.*)</span>"#).unwrap();

    let mat = re.captures_iter(&response).next();

    if let Some(mat) = mat {
        println!("mat: {}", &mat[1]);

        let (addresses, standard) = extract_info(&mat[1]);

        println!("addresses: {:?}", addresses);
        println!("standard: {}", standard);

    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use log::info;
    use serde_json::Value;

    #[tokio::test]
    async fn test_get_abi() {
        let api_key = env::var("TESTING_ETHERSCAN_API_KEY").expect("TESTING_ETHERSCAN_API_KEY must be set");
        let contract_address = env::var("TESTING_CONTRACT").expect("TESTING_CONTRACT must be set");

        let result = get_abi(contract_address.as_str(), &api_key).await.unwrap();

        info!("result: {:?}", result);

        // if result is empty string, then assert false otherwise use serde_json to convert into a Value and do assertions
        if result == "" {
            assert!(false);
        } else {
            let abi = result.parse::<Value>().ok();

            info!("ABI VALUE: {:?}", &abi);

            match abi {
                Some(abi) => {
                    assert!(abi.is_array());

                    let abi = abi.as_array().unwrap();

                    assert!(abi.len() > 0);
                }
                None => {
                    assert!(false);
                }
            }
        }
    }

    #[tokio::test]
    async fn test_source_code() {
        let api_key = env::var("TESTING_ETHERSCAN_API_KEY").expect("TESTING_ETHERSCAN_API_KEY must be set");
        let contract_address = env::var("TESTING_CONTRACT").expect("TESTING_CONTRACT must be set");

        let result = get_source_code(contract_address.as_str(), &api_key).await.expect("Failed to get source code");

        let abi = result.abi;
        let implementation = result.implementation;

        assert!(abi.len() > 0);
        assert!(implementation.len() > 0);
    }

    #[tokio::test]
    async fn test_get_previous_implementation() {
        let contract_address = env::var("TESTING_CONTRACT").expect("TESTING_CONTRACT must be set");

        let result = get_previous_implementation(contract_address.as_str()).await.expect("Failed to get previous implementation");

        println!("result: {:?}", result);

        assert!(true);
    }
}
