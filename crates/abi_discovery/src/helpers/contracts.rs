use bson::oid::ObjectId;
use futures::StreamExt;
use log::{error, debug};
use mongodb::Collection;
use redis::{aio::Connection, AsyncCommands, RedisError};
use simplefi_redis::{is_in_set, add_to_set};
use std::{collections::{HashMap, HashSet}, vec};

use abi_discovery_types::{AbiDiscoveryError, Abi, ContractAbi, ImplementationContractAbi, AbiStandards};
use crate::{http::etherscan::{SourceCodeResponse, get_source_code}, mongo::{types::{ContractAbiCollection, AbiCollection}, getters::{get_contracts_by_addresses, get_abis_by_ids}}};

use super::providers::Provider;

/// Get the contracts tracked name
///
/// # Arguments
///
/// * `chain` - A string slice that holds the chain name
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::contracts::get_contracts_tracked_name;
///
/// let tracked_name = get_contracts_tracked_name("ethereum");
///
/// assert_eq!(tracked_name, "abi:ethereum:contracts:tracked");
/// ```
///
/// # Returns
///
/// * `String` - The tracked name
///
/// # Panics
///
/// This function will panic if the chain name is empty
///
/// # Errors
///
/// This function will return an error if the chain name is empty
///
pub fn get_contracts_tracked_name(chain: &str) -> String {
    format!("abi:{}:contracts:tracked", chain)
}

///
/// Get the contracts queue name
///
/// # Arguments
///
/// * `chain` - A string slice that holds the chain name
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::contracts::get_contracts_queue_name;
///
/// let queue_name = get_contracts_queue_name("ethereum");
///
/// assert_eq!(queue_name, "test:abi:ethereum:contracts:queue");
/// ```
///
/// # Returns
///
/// * `String` - The queue name
///
/// # Panics
///
/// This function will panic if the chain name is empty
///
/// # Errors
///
/// This function will return an error if the chain name is empty
///
pub fn get_contracts_queue_name(chain: &str) -> String {
    format!("abi:{}:contracts:queue", chain)
}

pub fn get_contracts_cache_name(chain: &str, address: &str) -> String {
    format!("abi:{}:contract:{}", chain, address)
}

///
/// Get the contract abi
///
/// # Arguments
///
/// * `provider` - A reference to the provider
/// * `address` - A string slice that holds the contract address
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::contracts::get_contract_abi;
/// use abi_discovery::helpers::providers::Provider;
///
/// let provider = Provider {
///    ...
/// }
///
/// let abi = get_contract_abi(&provider, "0x0000000000");
///
/// assert_eq!(abi.is_ok(), true);
/// ```
///
/// # Returns
///
/// * `Result<SourceCodeResponse, AbiDiscoveryError>` - The contract abi
///
/// # Panics
///
/// This function will panic if the provider is empty
///
/// # Errors
///
/// This function will return an error if the provider is empty
/// If the provider type is not supported
/// If the provider returns an error
///
pub async fn get_contract_abi(
    provider: &Provider,
    address: &str,
) -> Result<SourceCodeResponse, AbiDiscoveryError> {
    match provider.provider_type.as_str() {
        "etherscan" => {
            let reponse = get_source_code(address, &provider.api_key).await;

            if reponse.is_err() {
                error!("Failed to get source code");

                return Err(AbiDiscoveryError::ProviderError);
            }

            let source_code: SourceCodeResponse = reponse.unwrap();

            Ok(source_code)
        }
        _ => {
            error!("Provider type not supported");

            Err(AbiDiscoveryError::ProviderError)
        }
    }
}

///
/// Get full contracts info
///
/// # Arguments
///
/// * `contract_collection` - A reference to the contract collection
/// * `abi_collection` - A reference to the abi collection
/// * `addresses` - A vector of strings that holds the contract addresses
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::contracts::get_full_contracts_info;
///
/// let contracts = get_full_contracts_info(&contract_collection, &abi_collection, &abi_collection, vec!["0x0000000000".to_string()]);
///
/// assert_eq!(contracts.is_ok(), true);
/// ```
///
/// # Returns
///
/// * `Result<Vec<ContractAbi>, AbiDiscoveryError>` - The contracts
///
/// # Panics
///
/// This function will panic if the contract collection is empty
///
/// # Errors
///
/// This function will return an error if the contract collection is empty
/// This function will return an error if the abi collection is empty
/// If any call to the database fails
///
pub async fn get_full_contracts_info(
    redis_connection: &mut Connection,
    contract_collection: &Collection<ContractAbiCollection>,
    abi_collection: &Collection<AbiCollection>,
    chain: &str,
    addresses: Vec<String>,
) -> Result<Vec<ContractAbi>, AbiDiscoveryError> {
    let mut contracts: Vec<ContractAbi> = Vec::new();
    let mut addresses_missing: Vec<String> = vec![];

    for address in &addresses {
        let contract = get_contract_cached(redis_connection, chain, &address).await;

        if contract.is_err() {
            error!("Failed to get contract from redis: {}", contract.unwrap_err());

            return Err(AbiDiscoveryError::RedisError);
        }

        let contract = contract.unwrap();

        if contract.is_none() {
            addresses_missing.push(address.clone());
        } else {
            contracts.push(contract.unwrap());
        }
    }

    if addresses_missing.len() == 0 {
        return Ok(contracts);
    }

    let contracts_results = get_contracts_by_addresses(contract_collection, addresses_missing).await;

    if contracts_results.is_err() {
        error!("Failed to get contracts: {}", contracts_results.unwrap_err());

        return Err(AbiDiscoveryError::MongoDBError);
    }

    let contracts_results = contracts_results.unwrap();

    let mut abi_ids: HashSet<ObjectId> = HashSet::new();

    for contract in &contracts_results {
        let contract = contract;

        match contract.abi_id {
            Some(abi_id) => abi_ids.insert(abi_id),
            None => false,
        };

        for implementation in &contract.implementations {
            match implementation.abi_id {
                Some(abi_id) => abi_ids.insert(abi_id),
                None => false,
            };
        }
    }

    let abi_ids: Vec<ObjectId> = abi_ids.into_iter().collect();

    let abis_results = get_abis_by_ids(abi_collection, abi_ids).await;

    if abis_results.is_err() {
        error!("Failed to get abis: {}", abis_results.unwrap_err());

        return Err(AbiDiscoveryError::MongoDBError);
    }

    let abis_results = abis_results.unwrap();

    let mut abi_map: HashMap<ObjectId, AbiCollection> = HashMap::new();

    for abi in abis_results {
        if let Some(abi_id) = abi.id.clone() {
            abi_map.insert(abi_id, abi);
        }
    }

    for contract in contracts_results {
        let contract = contract;

        let mut output_contract = ContractAbi {
            id: contract.id.clone().unwrap(),
            name: contract.name.clone(),
            address: contract.address.clone(),
            abi: None,
            creation_block: contract.creation_block.clone(),
            is_proxy: contract.is_proxy.clone(),
            implementations: vec![],
            verified: contract.verified.clone(),
        };

        if let Some(abi_id) = contract.abi_id.clone() {


            let abi_found = abi_map
                .get(&abi_id)
                .unwrap()
                .clone();

            let abi_contract = Abi {
                id: abi_found.id.clone().unwrap(),
                abi: abi_found.abi.clone(),
                abi_hash: abi_found.abi_hash.clone(),
                is_proxy: abi_found.is_proxy.clone(),
                standard: AbiStandards::from_u32(abi_found.standard.clone()),
            };

            output_contract.abi = Some(abi_contract);
        }

        for implementation in contract.implementations {
            if let Some(abi_id) = implementation.abi_id.clone() {
                let abi_found = abi_map
                    .get(&abi_id)
                    .unwrap()
                    .clone();

                let abi_contract = Abi {
                    id: abi_found.id.clone().unwrap(),
                    abi: abi_found.abi.clone(),
                    abi_hash: abi_found.abi_hash.clone(),
                    is_proxy: abi_found.is_proxy.clone(),
                    standard: AbiStandards::from_u32(abi_found.standard.clone()),
                };

                let implementation = ImplementationContractAbi {
                    abi: Some(abi_contract),
                    order: implementation.order.clone(),
                    name: implementation.name.clone(),
                    address: implementation.address.clone(),
                    creation_block: implementation.creation_block.clone(),
                    verified: implementation.verified.clone(),
                };

                output_contract.implementations.push(implementation);
            }
        }

        let cache_result = cache_contract_to_redis(redis_connection, &chain, &output_contract).await;

        if cache_result.is_err() {
            error!("Failed to cache contract: {}", cache_result.unwrap_err());

            return Err(AbiDiscoveryError::RedisError);
        }

        contracts.push(output_contract);
    }

    Ok(contracts)
}

///
/// Discover addresses
///
/// # Arguments
///
/// * `redis_connection` - A reference to the redis connection
/// * `chain` - A string slice that holds the chain name
/// * `addresses` - A vector of strings that holds the contract addresses
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::contracts::discover;
///
/// let result = discover(&mut redis_connection, "ethereum", vec!["0x0000000000".to_string()]);
///
/// assert_eq!(result.is_ok(), true);
/// ```
///
/// # Returns
///
/// * `Result<bool, AbiDiscoveryError>` - The result
///
/// # Panics
///
/// This function will panic if the redis connection is empty
///
/// # Errors
///
/// This function will return an error if the redis connection is empty
/// If any call to the database fails
///
pub async fn discover(
    mut redis_connection: &mut Connection,
    chain: &str,
    addresses: Vec<String>,
) -> Result<bool, AbiDiscoveryError> {
    let contracts_tracked_name = get_contracts_tracked_name(chain);
    let contracts_queue_name = get_contracts_queue_name(chain);

    for address in addresses.iter() {
        let check_address = is_in_set(&mut redis_connection, &contracts_tracked_name, &address)
            .await;

        if check_address.is_err() {
            error!("Failed to check if address is in set: {}", check_address.unwrap_err());

            return Err(AbiDiscoveryError::RedisError);
        }

        if check_address.unwrap() == false {
            let add_result = add_to_set(&mut redis_connection, &contracts_queue_name, &address)
                .await;

            if add_result.is_err() {
                error!("Failed to add address to set: {}", add_result.unwrap_err());

                return Err(AbiDiscoveryError::RedisError);

            }

            println!("Address {} added to queue", address);
        } else {
            println!("Address {} already tracked", address);
        }
    }

    Ok(true)
}

///
/// is tracked function to check if an address is tracked
///
/// # Arguments
///
/// * `redis_connection` - A reference to the redis connection
/// * `chain` - A string slice that holds the chain name
/// * `address` - A string slice that holds the contract address
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::contracts::is_tracked;
///
/// let result = is_tracked(&mut redis_connection, "ethereum", "0x0000000000");
///
/// assert_eq!(result.is_ok(), true);
/// ```
///
/// # Returns
///
/// * `Result<bool, AbiDiscoveryError>` - The result
///
/// # Panics
///
/// This function will panic if the redis connection is empty
///
/// # Errors
///
/// This function will return an error if the redis connection is empty
/// If any call to the database fails
///
pub async fn is_tracked(
    mut redis_connection: &mut Connection,
    chain: &str,
    address: &str,
) -> Result<bool, AbiDiscoveryError> {
    let redis_tracked_addresses_set = get_contracts_tracked_name(chain);

    let is_in_set = is_in_set(
        &mut redis_connection,
        &redis_tracked_addresses_set,
        &address
    ).await;

    if is_in_set.is_err() {
        return Err(AbiDiscoveryError::RedisError);
    }

    Ok(is_in_set.unwrap())
}

///
/// Add tracked function to add an address to the tracked set
/// This function will not check if the address is already tracked
/// Use is_tracked function to check if the address is already tracked
/// Use discover function to add an address to the queue if it is not already tracked
///
/// # Arguments
///
/// * `redis_connection` - A reference to the redis connection
/// * `chain` - A string slice that holds the chain name
/// * `address` - A string slice that holds the contract address
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::contracts::add_tracked;
///
/// let result = add_tracked(&mut redis_connection, "ethereum", "0x0000000000");
///
/// assert_eq!(result.is_ok(), true);
/// ```
///
/// # Returns
///
/// * `Result<bool, AbiDiscoveryError>` - The result
///
/// # Panics
///
/// This function will panic if the redis connection is empty
///
/// # Errors
///
/// This function will return an error if the redis connection is empty
/// If any call to the database fails
///
pub async fn add_tracked(
    mut redis_connection: &mut Connection,
    chain: &str,
    address: &str,
) -> Result<bool, AbiDiscoveryError> {
    let redis_tracked_addresses_set = get_contracts_tracked_name(chain);

    let added_result = add_to_set(
        &mut redis_connection,
        &redis_tracked_addresses_set,
        &address
    ).await;

    if added_result.is_err() {
        return Err(AbiDiscoveryError::RedisError);
    }

    Ok(true)
}

///
/// Copy contracts to redis set
/// This function will not check if the address is already tracked
///
/// # Arguments
///
/// * `redis_connection` - A reference to the redis connection
/// * `contracts_abi_collection` - A reference to the contracts abi collection
/// * `chain` - A string slice that holds the chain name
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::contracts::copy_contracts_to_redis_set;
///
/// let result = copy_contracts_to_redis_set(&mut redis_connection, &contracts_abi_collection, "ethereum");
///
/// assert_eq!(result.is_ok(), true);
/// ```
///
/// # Returns
///
/// * `Result<bool, AbiDiscoveryError>` - The result
///
/// # Panics
///
/// This function will panic if the redis connection is empty
///
/// # Errors
///
/// This function will return an error if the redis connection is empty
/// If any call to the database fails
///
pub async fn copy_contracts_to_redis_set(
    mut redis_connection: &mut Connection,
    contracts_abi_collection: &Collection<ContractAbiCollection>,
    chain: &str,
) -> Result<bool, AbiDiscoveryError> {
    let contracts_tracked_name = get_contracts_tracked_name(chain);

    let cursor = contracts_abi_collection
        .find(None, None)
        .await;

    if cursor.is_err() {
        error!("Failed to get contracts: {}", cursor.unwrap_err());

        return Err(AbiDiscoveryError::MongoDBError);
    }

    let mut cursor = cursor.unwrap();

    while let Some(contract) = cursor.next().await {
        if contract.is_err() {
            error!("Failed to get contract: {}", contract.unwrap_err());

            continue;
        }

        let contract = contract.unwrap();

        debug!("Adding contract to redis set: {}", contract.address);

        let add_result = add_to_set(
            &mut redis_connection,
            &contracts_tracked_name,
            &contract.address,
        ).await;

        if add_result.is_err() {
            error!("Failed to add address to set: {}", add_result.unwrap_err());

            return Err(AbiDiscoveryError::RedisError);
        }
    }

    Ok(true)
}

///
/// Get contract abi from redis
///
/// # Arguments
///
/// * `redis_connection` - A reference to the redis connection
/// * `chain` - A string slice that holds the chain name
/// * `address` - A string slice that holds the contract address
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::contracts::get_contract_abi_from_redis;
///
/// let result = get_contract_abi_from_redis(&mut redis_connection, "ethereum", "0x0000000000");
///
/// assert_eq!(result.is_ok(), true);
/// ```
///
/// # Returns
///
/// * `Result<Option<ContractAbi>, AbiDiscoveryError>` - The contract abi
///
/// # Panics
///
/// This function will panic if the redis connection is empty
///
/// # Errors
///
/// This function will return an error if the redis connection is empty
/// If any call to the database fails
///
pub async fn cache_contract_to_redis(
    redis_connection: &mut Connection,
    chain: &str,
    contract: &ContractAbi,
) -> Result<bool, AbiDiscoveryError> {
    let contract_json = serde_json::to_string(&contract);

    if contract_json.is_err() {
        error!("Failed to serialize contract: {}", contract_json.unwrap_err());

        return Err(AbiDiscoveryError::SerializationError);
    }

    let contract_json = contract_json.unwrap();

    let contract_key_name = get_contracts_cache_name(chain, &contract.address);

    // TODO: Set expiration time in global settings
    let _: () = redis_connection.set(&contract_key_name, &contract_json).await.unwrap();

    let _: () = redis_connection.expire(&contract_key_name, 3600).await.unwrap();

    Ok(true)
}

///
/// Get contract abi from redis
///
/// # Arguments
///
/// * `redis_connection` - A reference to the redis connection
/// * `chain` - A string slice that holds the chain name
/// * `address` - A string slice that holds the contract address
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::contracts::get_contract_abi_from_redis;
///
/// let result = get_contract_abi_from_redis(&mut redis_connection, "ethereum", "0x0000000000");
///
/// assert_eq!(result.is_ok(), true);
/// ```
///
/// # Returns
///
/// * `Result<Option<ContractAbi>, AbiDiscoveryError>` - The contract abi
///
/// # Panics
///
/// This function will panic if the redis connection is empty
///
/// # Errors
///
/// This function will return an error if the redis connection is empty
/// If any call to the database fails
///
pub async fn get_contract_cached(
    redis_connection: &mut Connection,
    chain: &str,
    address: &str,
) -> Result<Option<ContractAbi>, AbiDiscoveryError> {
    let contract_key_name = get_contracts_cache_name(chain, address);

    let contract_json: Result<Option<String>, RedisError> = redis_connection.get(&contract_key_name).await;

    if contract_json.is_err() {
        return Err(AbiDiscoveryError::RedisError);
    }

    // unwrap contract_json and check if it's none
    let contract_json = contract_json.unwrap();

    if contract_json.is_none() {
        return Ok(None);
    }

    let contract_json = contract_json.unwrap();

    let contract = serde_json::from_str::<ContractAbi>(&contract_json);

    if contract.is_err() {
        error!("Failed to deserialize contract: {}", contract.unwrap_err());

        return Err(AbiDiscoveryError::SerializationError);
    }

    let contract = contract.unwrap();

    // TODO: Set expiration time in global settings
    let _: () = redis_connection.expire(&contract_key_name, 3600).await.unwrap();

    Ok(Some(contract))
}
