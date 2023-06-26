use log::{error, debug, warn};
use redis::{aio::Connection, RedisError, Script};
use redis::AsyncCommands;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use simplefi_redis::check_set_exists;
use abi_discovery_types::AbiDiscoveryError;

pub struct RateLimit {
    pub second: Option<u32>,
    pub minute: Option<u32>,
    pub day: Option<u32>,
    pub month: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provider {
    pub chain: String,
    pub name: String,
    pub api_key: String,
    pub provider_type: String,
    pub rate_limits: HashMap<String, u32>,
}

///
/// Get provider rate key name
///
/// # Arguments
///
/// * `chain` - Chain name
/// * `provider_name` - Provider name
/// * `rate` - Rate name
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::providers::get_provider_rate_key_name;
///
/// let chain = "ethereum";
/// let provider_name = "infura";
/// let rate = "seconds";
///
/// let provider_rate_key_name = get_provider_rate_key_name(&chain, &provider_name, &rate);
///
/// assert_eq!(provider_rate_key_name, "abi:ethereum:provider:infura:rate:seconds");
/// ```
///
/// # Returns
///
/// * `String` - Provider rate key name
///
/// # Panics
///
/// * If `chain` is empty
/// * If `provider_name` is empty
/// * If `rate` is empty
///
/// # Errors
///
/// * If `chain` is empty
/// * If `provider_name` is empty
/// * If `rate` is empty
///
fn get_provider_rate_key_name(chain: &str, provider_name: &str, rate: &str) -> String {
    format!("abi:{}:provider:{}:rate:{}", chain, provider_name, rate)
}

///
/// Get provider list name
///
/// # Arguments
///
/// * `chain` - Chain name
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::providers::get_provider_list_name;
///
/// let chain = "ethereum";
///
/// let provider_list_name = get_provider_list_name(&chain);
///
/// assert_eq!(provider_list_name, "abi:ethereum:providers");
/// ```
///
/// # Returns
///
/// * `String` - Provider list name
///
/// # Panics
///
/// * If `chain` is empty
///
/// # Errors
///
/// * If `chain` is empty
///
fn get_provider_list_name(chain: &str) -> String {
    format!("abi:{}:providers", chain)
}

///
/// Get provider
///
/// # Arguments
///
/// * `connection` - Redis connection
/// * `chain` - Chain name
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::providers::get_provider;
///
/// let chain = "ethereum";
///
/// let provider = get_provider(&mut redis_connection, &chain).await;
///
/// assert_eq!(provider.is_err(), false);
/// ```
///
/// # Returns
///
/// * `Result<Provider, AbiDiscoveryError>` - Provider
///
/// # Panics
///
/// * If `chain` is empty
///
/// # Errors
///
/// * If `chain` is empty
/// * If `provider` is empty
/// * If `provider` is invalid
/// * If `provider` is not found
/// * If `provider` rate limit is invalid
/// * If `provider` rate limit is exceeded
///
pub async fn get_available_provider(
    connection: &mut Connection,
    chain: &str,
) -> Result<Provider, AbiDiscoveryError> {
    let mut provider_name = String::new();

    loop {
        let provider = get_next(connection, &chain).await;

        if provider.is_err() {
            error!("Failed to get provider");

            return Err(provider.unwrap_err());
        }

        let provider = provider.unwrap();

        if provider.name == provider_name {
            warn!("All providers are exhausted");

            return Err(AbiDiscoveryError::ProviderAllExpiredError);
        } else if provider_name.len() == 0 {
            provider_name = provider.name.clone();
        }

        let rate_limit = consume(connection, &provider).await;

        if rate_limit.is_err() {
            return Err(rate_limit.unwrap_err());
        }

        let rate_limit = rate_limit.unwrap();

        if rate_limit == false {
            debug!("Provider rate limit exceeded. Try next available provider");

            continue;
        }

        return Ok(provider);
    }
}

///
/// Get next provider
///
/// # Arguments
///
/// * `connection` - Redis connection
/// * `chain` - Chain name
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::providers::get_next;
///
/// let chain = "ethereum";
///
/// let provider = get_next(&mut redis_connection, &chain).await;
///
/// assert_eq!(provider.is_err(), false);
/// ```
///
/// # Returns
///
/// * `Result<Provider, AbiDiscoveryError>` - Provider
///
/// # Panics
///
/// * If `chain` is empty
///
async fn get_next(
    connection: &mut Connection,
    chain: &str
) -> Result<Provider, AbiDiscoveryError> {
    let abi_providers_list_name = get_provider_list_name(&chain);

    debug!("abi_providers_list_name: {:?}", &abi_providers_list_name);

    let script = r#"
        local api_keys = redis.call('LRANGE', KEYS[1], 0, -1)
        if next(api_keys) == nil then
            return redis.error_reply("No Provider available for that key name")
        end
        local key = redis.call('RPOP', KEYS[1])
        redis.call('LPUSH', KEYS[1], key)
        return key
    "#;

    let key: Result<String, RedisError> = Script::new(script)
        .key(&abi_providers_list_name)
        .invoke_async(connection)
        .await;

    if key.is_err() {
        return Err(AbiDiscoveryError::ProviderNotFoundError);
    }

    let provider = serde_json::from_str::<Provider>(&key.unwrap());

    if provider.is_err() {
        return Err(AbiDiscoveryError::ProviderNotFoundError);
    }

    Ok(provider.unwrap())
}

///
/// List providers
///
/// # Arguments
///
/// * `connection` - Redis connection
/// * `chain` - Chain name
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::providers::list;
///
/// let chain = "ethereum";
///
/// let providers = list(&mut redis_connection, &chain).await;
///
/// assert_eq!(providers.is_err(), false);
/// ```
///
/// # Returns
///
/// * `Result<Vec<Provider>, AbiDiscoveryError>` - Providers
///
/// # Panics
///
/// * If `chain` is empty
///
pub async fn list(
    connection: &mut Connection,
    chain: String
) -> Result<Vec<Provider>, AbiDiscoveryError> {
    let abi_providers_list_name = get_provider_list_name(&chain);

    debug!("abi_providers_list_name: {:?}", &abi_providers_list_name);

    let abi_providers: Vec<String> = connection.lrange(&abi_providers_list_name, 0, -1).await.unwrap();

    let mut providers: Vec<Provider> = Vec::new();

    for abi_provider in abi_providers.iter() {
        let current_provider = serde_json::from_str::<Provider>(&abi_provider).unwrap();

        providers.push(current_provider);
    }

    Ok(providers)
}

///
/// Add provider
///
/// # Arguments
///
/// * `connection` - Redis connection
/// * `provider` - Provider
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::providers::add;
///
/// let provider = Provider {
///    chain: "ethereum".to_string(),
///    name: "infura".to_string(),
/// }
///
/// let result = add(&mut redis_connection, provider).await;
///
/// assert_eq!(result.is_err(), false);
/// ```
///
/// # Returns
///
/// * `Result<bool, AbiDiscoveryError>` - True if provider was added, false if provider already exists
///
/// # Panics
///
/// * If `chain` is empty
/// * If `provider` is empty
///
pub async fn add(
    connection: &mut Connection,
    provider: Provider,
) -> Result<bool, AbiDiscoveryError> {
    let abi_providers_list_name = get_provider_list_name(&provider.chain);

    debug!("abi_providers_list_name: {:?}", &abi_providers_list_name);

    let abi_providers: Vec<String> = connection.lrange(&abi_providers_list_name, 0, -1).await.unwrap();

    for abi_provider in abi_providers.iter() {
        let current_provider = serde_json::from_str::<Provider>(&abi_provider).unwrap();

        if current_provider.name == provider.name {
            warn!("Provider already exists");

            return Ok(false);
        }
    }

    let _: () = connection.lpush(&abi_providers_list_name, serde_json::to_string(&provider).unwrap()).await.unwrap();

    debug!("Provider added");

    Ok(true)
}

///
/// Consume provider rate limit
///
/// # Arguments
///
/// * `connection` - Redis connection
/// * `provider` - Provider
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::providers::consume;
///
/// let provider = Provider {
///   chain: "ethereum".to_string(),
///  name: "infura".to_string(),
/// }
///
/// let result = consume(&mut redis_connection, provider).await;
///
/// assert_eq!(result.is_err(), false);
/// ```
///
/// # Returns
///
/// * `Result<bool, AbiDiscoveryError>` - True if provider rate limit was consumed, false if provider rate limit was exceeded
///
/// # Panics
///
/// * If `chain` is empty
/// * If `provider` is empty
/// * If `provider` is invalid
/// * If `provider` is not found
/// * If `provider` rate limit is invalid
/// * If `provider` rate limit is exceeded
///
async fn consume(
    connection: &mut Connection,
    provider: &Provider,
) -> Result<bool, AbiDiscoveryError> {

    debug!("consume_provider_rate_limit called");

    if provider.rate_limits.keys().len() == 0 {
        debug!("Provider has no rate limits");

        return Ok(true);
    }

    for rate in provider.rate_limits.keys() {
        let provider_rate_key_name = get_provider_rate_key_name(&provider.chain, &provider.name, rate);

        if check_set_exists(connection, &provider_rate_key_name).await.unwrap() == false {
            // Read limit from provider.rate_limits and covert to u32
            let limit = provider.rate_limits.get(rate).unwrap();

            let _: () = connection.set(&provider_rate_key_name, limit.to_string()).await.unwrap();

            let expiration = match rate.as_str() {
                "seconds" => 1,
                "minutes" => 60,
                "days" => 86400,
                "months" => 2592000,
                _ => 0,
            };

            let _: () = connection.expire(&provider_rate_key_name, expiration).await.unwrap();
        } else {
            let limit: Result<u32, RedisError> = connection.get(&provider_rate_key_name).await;

            if limit.is_err() {
                return Err(AbiDiscoveryError::ProviderRateLimitExceededError);
            }
        }
    }

    for rate in provider.rate_limits.keys() {
        let provider_rate_key_name = get_provider_rate_key_name(&provider.chain, &provider.name, rate);

        let _: () = connection.decr(&provider_rate_key_name, 1).await.unwrap();
    }

    Ok(true)
}
