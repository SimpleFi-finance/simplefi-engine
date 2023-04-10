use http::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, Error, Response};
use std::collections::HashMap;

/// Fetch data from a URL
///
/// # Arguments
///
/// * `url` - The URL to fetch data from
/// * `query_params` - The query parameters to add to the URL
/// * `headers` - The headers to add to the request
/// * `bearer_token` - The bearer token to add to the request
///
/// # Returns
///
/// * `Result<T, Error>` - The data returned from the URL
pub async fn fetch(
    url: &str,
    query_params: HashMap<String, String>,
    headers: Option<HeaderMap<HeaderValue>>,
    bearer_token: Option<&str>,
) -> Result<Response, Error> {
    // Create client
    let client = Client::new();

    // Build request
    let mut request_builder = client.get(url).query(&query_params);

    // Add custom headers
    if let Some(header_map) = headers {
        request_builder = request_builder.headers(header_map);
    }

    // Add bearer token if provided
    if let Some(token) = bearer_token {
        let bearer = format!("Bearer {}", token);
        request_builder = request_builder.header(AUTHORIZATION, bearer);
    }

    // Send request and get response
    let response: Response = request_builder.send().await?;

    // Return data if successful
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use tracing::{info};

    #[derive(Deserialize)]
    struct ApiResponse {
        key: String,
        value: String,
    }

    #[tokio::test]
    async fn test_basic_fetch() {
        // Define mock data and mock server
        let mock_response = r#"{"key": "test_key", "value": "test_value"}"#;

        // Request a new server from the pool
        let mut server = mockito::Server::new();

        let mock = server
            .mock("GET", "/api/data")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response)
            .create();

        // Use the mock server URL and query parameters for the fetch function
        let base_url = server.url();

        info!(base_url=?base_url, "base_url");

        let url = format!("{}/api/data", base_url);
        let query_params = HashMap::new();
        let headers = None;
        let bearer_token = None;
        let result: Result<Response, Error> =
            fetch(&url, query_params, headers, bearer_token).await;

        assert!(result.is_ok());

        let response = result.unwrap();

        assert_eq!(response.status().as_u16(), 200);

        let data: ApiResponse = response.json().await.unwrap();

        assert_eq!(data.key, "test_key");
        assert_eq!(data.value, "test_value");

        info!(key=?data.key, "KEY");
        info!(value=?data.value, "VALUE");

        mock.assert();
    }
}

