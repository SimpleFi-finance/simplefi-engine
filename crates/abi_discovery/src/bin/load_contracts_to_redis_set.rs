use log::{ debug, error };
use std::env;
use tokio;

use simplefi_engine_settings::load_settings;

use simplefi_logger::init_logging;
use simplefi_redis::connect;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let chain = env::var("ENV_CHAIN");

    let chain = match chain {
        Ok(chain) => {
            debug!("Loading contracts from chain: {}", chain);

            chain
        },
        Err(error) => {
            error!("Failed to get chain name from environment variable ENV_CHAIN: {:?}", error);

            return Err("Missing ENV_CHAIN environment variable".into());
        }
    };

    debug!("Loading contracts into redis set for chain: {}", chain);

    let mysettings = load_settings().expect("Failed to load settings");
    let redis_uri = mysettings.redis_uri.to_string();

    let redis_connection = connect(redis_uri.as_str()).await;

    if redis_connection.is_err() {
        error!("Failed to connect to redis: {:?}", redis_connection.err().unwrap());

        return Err("Failed to connect to redis".into());
    }

    let mut redis_connection = redis_connection.unwrap();

    let mongodb_uri = mysettings.mongodb_uri.to_string();
    let mongodb_database_name = mysettings.mongodb_database_name.to_string();

    // TODO: replace with rocksDB

    // let mongo_config = MongoConfig {
    //     uri: mongodb_uri,
    //     database: mongodb_database_name,
    // };

    // println!("Mongo config: {:?}", mongo_config);

    // let mongo = Mongo::new(&mongo_config).await;

    // if mongo.is_err() {
    //     error!("Failed to connect to MongoDB: {:?}", mongo.err().unwrap());

    //     return Err("Failed to connect to MongoDB".into());
    // }

    // let mongo = mongo.unwrap();

    let contract_abi_collection_name = format!("{}_{}", chain, &mysettings.contract_abi_collection_name);

    debug!("Contract ABI collection name: {}", contract_abi_collection_name);

    // let contracts_collection = mongo.database.collection::<ContractAbiCollection>(&contract_abi_collection_name);
        // TODO:

    // let is_copied = copy_contracts_to_redis_set(
    //     &mut redis_connection,
    //     &contracts_collection,
    //     &chain,
    // ).await;

    // if is_copied.is_err() {
    //     error!("Failed to copy contracts to redis set: {:?}", is_copied.err().unwrap());

    //     return Err("Failed to copy contracts to redis set".into());
    // }

    debug!("Contracts copied to redis set");

    Ok(())
}
