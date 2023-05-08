use log::{ debug, error, warn, info };
use http::header::{ HeaderMap, HeaderValue, CONTENT_TYPE };
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;

use crate::http::fetch::fetch;

#[derive(Debug, Clone, Deserialize)]
pub struct FourByteEventSignature {
    pub id: u32,
    pub created_at: String,
    pub text_signature: String,
    pub hex_signature: String,
    pub bytes_signature: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FourByteResponse {
    pub count: u64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<FourByteEventSignature>,
}

/* Example
{

    "count": 185760,
	"next": "https://www.4byte.directory/api/v1/event-signatures/?count=2&page=2",
	"previous": null,
	"results": [
		{
			"id": 185984,
			"created_at": "2023-05-05T08:37:55.201159Z",
			"text_signature": "BuyLimitOrderEnqueued(uint256,uint112,uint112,bool,bool,uint32,uint32,uint256,uint32,uint32,address,address,uint256)",
			"hex_signature": "0xad8aad49ce4054e19ffae260cfa0f4cf3aa7e9a59d9837e2cb1fd88746e9cfda",
			"bytes_signature": "­­IÎ@Táúâ`Ï ôÏ:§é¥7âËØFéÏÚ"
		},
} */

pub async fn get_event_signatures(
    next_page: Option<String>,
) -> FourByteResponse {
    // starting page
    let four_byte_event_signatures = "https://www.4byte.directory/api/v1/event-signatures/";

    let url = match next_page {
        Some(next_page) => next_page,
        None => four_byte_event_signatures.to_string(),
    };

    let query_params = HashMap::new();

    info!("url: {:?}", url);

    // Add custom headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let result = fetch(&url, query_params, Some(headers), None).await.expect("Error fetching data from 4byte.directory");

    warn!("status: {:?}", result.status());

    match result.status() {
        StatusCode::OK => {
            let response = result.json::<FourByteResponse>().await;

            let body = match response {
                Ok(response) => {
                    debug!("response: {:?}", response);

                    response
                },
                Err(e) => {
                    error!("error: {:?}", e);

                    FourByteResponse {
                        count: 0,
                        next: None,
                        previous: None,
                        results: Vec::new(),
                    }
                }
            };

            debug!("body: {:?}", body);

            body
        },
        _ => {
            error!("Error returning: {}", result.status());

            FourByteResponse {
                count: 0,
                next: None,
                previous: None,
                results: Vec::new(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;
    use shared_utils::logger::init_logging;

    #[tokio::test]
    async fn test_event_signatures_count_good() {
        init_logging();

        let result = get_event_signatures(None).await;

        info!("result: {:?}", result.count);

        assert!(result.count > 0);
    }

    #[tokio::test]
    async fn test_event_signatures_count_0() {
        init_logging();

        let result = get_event_signatures(Some("https://www.4byte.directory/api/v1/event-signatures/&page=10000000".to_string())).await;

        info!("result: {:?}", result.count);

        assert!(result.count == 0);
    }
}

