use log::{ debug, error };
use http::header::{ HeaderMap, HeaderValue, CONTENT_TYPE };
use reqwest::{ StatusCode, Error };
use serde::Deserialize;
use std::collections::HashMap;

use crate::http::fetch::fetch;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct EtherscanResponse {
    status: String,
    message: String,
    result:  String,
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

    // Add custom headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let result = fetch(etherscan_api_url, query_params, Some(headers), None).await?;

    match result.status() {
        StatusCode::OK => {
            let results: EtherscanResponse = result.json().await?;

            debug!("result: {:?}", results.result.len());

            Ok(results.result)
        },
        _ => {
            error!("Error: {}", result.status());

            Ok("".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;
    use serde_json::Value;
    use settings::load_settings;

    #[tokio::test]
    async fn test_get_abi() {
        let local_settings = load_settings().unwrap();
        let api_key = local_settings.etherscan_api_keys;

        let contract_address = "0x6b175474e89094c44da98b954eedeac495271d0f";

        let result = get_abi(contract_address, &api_key).await.unwrap();

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

                },
                None => {
                    assert!(false);
                }
            }
        }
    }
}

