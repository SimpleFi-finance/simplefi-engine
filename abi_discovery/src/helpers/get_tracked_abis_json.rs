use log::{ info, debug };
use redis::{ AsyncCommands, Expiry::PX};
use std::collections::HashMap;

use crate::{ settings::load_settings, helpers::{ get_tracked_abi_json_from_mongo} };
use shared_types::redis::abi::{ ContractWithAbiJSONRedis};
use third_parties::redis::{ connect, is_in_set };

///
/// Function to get tracked abis from redis
///
/// # Arguments
///
/// * `addresses` - Vec<String> of addresses to get abi for
///
/// # Returns
///
/// * `Result<HashMap<String, String>, Box<dyn std::error::Error>>` - Result of HashMap<String, String> or Error
///
/// # Example
///
/// ```
/// use abi_discovery::get_tracked_abis;
///
/// let addresses = vec!["0x.."];
///
/// let result = get_tracked_abis(addresses).await;
///
/// match result {
///   Ok(abis) => info!("abis: {:?}", abis),
///  Err(e) => error!("error: {:?}", e),
/// }
///
/// ```
///
/// # Panics
///
/// This function will panic if the redis_uri is not set in the settings file
///
/// # Notes
///
/// This function will check if the address is in the redis set called tracked_addresses
/// If the address is in the set, it will check if the abi is in the redis hash called address_abi
/// If the abi is not in the hash, it will get the abi from mongo and store it in the hash
/// If the address is not in the set, it will add the address to the set and send the address to the queue to be scraped
///
/// # TODO
///
/// * Add the address to the queue to be scraped
/// * change logger to use env_logger
///
pub async fn get_tracked_abis_json(
    addresses: Vec<String>,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    debug!("get_tracked_abis: {:?}", addresses);

    // get settings
    let mysettings = load_settings()?;

    // get redis_url from settings
    let redis_uri = mysettings.redis_uri;

    info!("redis url: {:?}", redis_uri);

    // connect to redis with third_parties::redis::connect
    let mut connection = connect(&redis_uri).await?;

    let mut tracked_addresses: Vec<String> = vec![];
    let mut untracked_addresses: Vec<String> = vec![];

    // for each tracked address, we are going to check if we already have the abi cached in redis collection called address_abi
    // each abi is a binary string
    let mut tracked_abis = HashMap::new();

    // check if addresses are in redis
    for address in addresses {
        let is_in_set = is_in_set(&mut connection, "tracked_addresses", &address).await?;
        debug!("address: {:?} is_in_set: {:?}", address, is_in_set);

        if is_in_set {
            tracked_addresses.push(address);
        } else {
            untracked_addresses.push(address);
        }
    }

    // if tracked_addresses is empty, return empty vec
    if tracked_addresses.is_empty() {
        // TODO: for untracked addresses, send to queue to be scrapped
        return Ok(tracked_abis);
    }


    // Vec<String> = vec![];
    debug!("tracked_addresses: {:?}", tracked_addresses);

    // create vector of strings named addresses_mongo
    let mut addresses_mongo: Vec<String> = vec![];

    let abi_prefix = mysettings.redis_abi_key_prefix;
    let abi_ttl = mysettings.redis_key_ttl_expire_ms;

    for address in tracked_addresses {
        // Check if the key exists in the hash
        let key = format!("{}{}", &abi_prefix, &address);

        debug!("key: {:?} ", &key);

        // check if the key exists in redis
        let exists: bool = connection.exists(&key).await?;

        if exists {
            debug!("exists: {:?}", &address);

            // get abi from redis
            let abi: String = connection.get_ex(&key, PX(abi_ttl)).await?;

            debug!("abi: {:?}", abi);

            tracked_abis.insert(address, abi);
        } else {
            debug!("does not exist: {:?}. We get it from mongo", &address);

            // get abi from mongo
            addresses_mongo.push(address);
        }
    }

    // if addresses_mongo is empty, return tracked_abis
    if addresses_mongo.is_empty() {
        return Ok(tracked_abis);
    }

    debug!("addresses_mongo: {:?}", addresses_mongo);

    // if abi is not in redis, get it from mongo
    let tracked_abis_mongo = get_tracked_abi_json_from_mongo(addresses_mongo).await?;

    debug!("tracked_abis_mongo: {:?}", tracked_abis_mongo.len());

    // Add to the redis cache all tracked_abis_mongo
    for tracked_abi_mongo in tracked_abis_mongo {
        let address = tracked_abi_mongo.address.clone();

        let contract_abi_redis = ContractWithAbiJSONRedis {
            timestamp: tracked_abi_mongo.timestamp,
            abi: tracked_abi_mongo.abi,
        };

        let serialized_value = serde_json::to_string(&contract_abi_redis)?;

        let key = format!("{}{}", abi_prefix, address);

        let _: () = connection.pset_ex(&key, &serialized_value, abi_ttl).await?;

        debug!("key: {:?} added to redis with expiration", &key);

        tracked_abis.insert(address, serialized_value);
    }

    Ok(tracked_abis)
}


#[cfg(test)]
mod tests {
    use super::*;
    use log::info;

    #[tokio::test]
    async fn test_get_tracked_abis() {
        let addresses = vec![
            "empty_return_data".to_string(),
        ];

        let abis = get_tracked_abis_json(addresses).await.unwrap();

        assert!(abis.len() == 0);

        // TODO: implement tear up and down for these fixtures
        let addresses = vec![
            "0x6b175474e89094c44da98b954eedeac495271d0f".to_string(),
            "0x0000000000003f5e74c1ba8a66b48e6f3d71ae82".to_string(),
            "0x00000000009726632680fb29d3f7a9734e3010e2".to_string(),
            "0x0000871c95bb027c90089f4926fd1ba82cdd9a8b".to_string(),
            "0x00000000b65e5ae3c80a89f73fbcfdeabc3c2c06".to_string(),
        ];

        let abis = get_tracked_abis_json(addresses).await.unwrap();

        info!("abis len: {:?}", abis.len());
        info!("abis: {:?}", abis);

        assert!(abis.len() == 2);

    }
}
