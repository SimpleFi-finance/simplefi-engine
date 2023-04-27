use clap::{ Parser };
use confy::{load, store, ConfyError};
use serde::{Deserialize, Serialize};
use settings::helpers::parse_usize;

#[derive(Parser, Debug)]
#[command(author = "SimpleFi Finance")]
#[command(version)]
#[command(about = "ABI Discovery Settings Generator")]
#[command(
    long_about = "ABI Discovery Settings Generator enhaces the running experience generating a settings files with required and default properties."
)]
#[command(next_line_help = true)]
pub struct AbiDiscoverySettings {
    // Redis Settings
    #[arg(
        long = "redis_uri",
        help = "Redis DB URI",
        default_value = "redis://localhost:6379/"
    )]
    pub redis_uri: String,

    #[arg(
        long = "redis_abi_key_prefix",
        help = "Redis ABI Key prefix",
        default_value = "abi:"
    )]
    pub redis_abi_key_prefix: String,

    #[arg(
        long = "redis_key_ttl_expire_ms",
        help = "Redis Key TTL expiration in milliseconds",
        default_value = "300000", // Default 5 minutes for expiration
        value_parser(parse_usize),
    )]
    pub redis_key_ttl_expire_ms: usize,

    // Etherscan API Key. For more than 1 key, separate them with a comma
    #[arg(
        long = "etherscan_api_keys",
        help = "Etherscan API key",
        default_value = "changeme"
    )]
    pub etherscan_api_keys: String,

    // MongoDB Settings
    #[arg(
        long = "mongodb_uri",
        help = "MongoDB URI",
        default_value = "mongodb://localhost:27017/"
    )]
    pub mongodb_uri: String,

    #[arg(
        long = "mongodb_database_name",
        help = "MongoDB Engine DB",
        default_value = "simplefi_engine"
    )]
    pub mongodb_database_name: String,

    #[arg(
        long = "mongodb_abi_collection",
        help = "MongoDB ABI Collection",
        default_value = "abis"
    )]
    pub mongodb_abi_collection: String,

    #[arg(
        long = "mongodb_contract_abi_collection",
        help = "MongoDB Contract ABI Collection",
        default_value = "contract-abi"
    )]
    pub mongodb_contract_abi_collection: String,

    #[arg(
        long = "mongodb_factory_contracts_collection",
        help = "MongoDB Factory Contracts Collection",
        default_value = "factory-contracts"
    )]
    pub mongodb_factory_contracts_collection: String,

    #[arg(
        long = "rabbit_mq_url",
        help = "Rabbit MQ Url",
        default_value = "amqp://guest:guest@localhost:5672"
    )]
    pub rabbit_mq_url: String,

    #[arg(
        long = "rabbit_exchange_name",
        help = "Rabbit MQ Exchange Name for Abi Discovery",
        default_value = "abi_discovery"
    )]
    pub rabbit_exchange_name: String,
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MyAbiDiscoverySettings {
    // Redis Settings
    pub redis_uri: String,
    pub redis_abi_key_prefix: String,
    pub redis_key_ttl_expire_ms: usize,
    // Etherscan API Key. For more than 1 key, separate them with a comma
    pub etherscan_api_keys: String,
    // MongoDB Settings
    pub mongodb_uri: String,
    pub mongodb_database_name: String,
    pub mongodb_abi_collection: String,
    pub mongodb_contract_abi_collection: String,
    pub mongodb_factory_contracts_collection: String,
    // RabbitMQ Settings
    pub rabbit_mq_url: String,
    pub rabbit_exchange_name: String,
}



impl MyAbiDiscoverySettings {
    pub fn new(
        redis_uri: String,
        redis_abi_key_prefix: String,
        redis_key_ttl_expire_ms: usize,
        etherscan_api_keys: String,
        mongodb_uri: String,
        mongodb_database_name: String,
        mongodb_abi_collection: String,
        mongodb_contract_abi_collection: String,
        mongodb_factory_contracts_collection: String,
        rabbit_mq_url: String,
        rabbit_exchange_name: String,
    ) -> Self {
        MyAbiDiscoverySettings {
            redis_uri,
            redis_abi_key_prefix,
            redis_key_ttl_expire_ms,
            etherscan_api_keys,
            mongodb_uri,
            mongodb_database_name,
            mongodb_abi_collection,
            mongodb_contract_abi_collection,
            mongodb_factory_contracts_collection,
            rabbit_mq_url,
            rabbit_exchange_name,
        }
    }
}


pub fn load_settings() -> Result<MyAbiDiscoverySettings, ConfyError> {
    let default_settings = MyAbiDiscoverySettings {
        redis_uri: String::from("redis://localhost:6379/"),
        redis_abi_key_prefix: String::from("abis:"),
        redis_key_ttl_expire_ms: 300000,
        etherscan_api_keys: String::from("change_etherscan_api_key_1,change_etherscan_api_key_2"),
        mongodb_uri: String::from("mongodb://localhost:27017/"),
        mongodb_database_name: String::from("simplefi_engine"),
        mongodb_abi_collection: String::from("abis"),
        mongodb_contract_abi_collection: String::from("contract-abi"),
        mongodb_factory_contracts_collection: String::from("factory-contracts"),
        rabbit_mq_url: String::from("amqp://guest:guest@localhost:5672"),
        rabbit_exchange_name: String::from("abi_discovery"),
    };

    let settings: MyAbiDiscoverySettings = load("simplefi_engine", Some("abi_discovery_settings")).unwrap_or(default_settings);

    Ok(settings)
}

pub fn store_settings(settings: &MyAbiDiscoverySettings) -> Result<(), ConfyError> {
    store("simplefi_engine", Some("abi_discovery_settings"), &settings).expect("Failed to store abi discovery settings");

    Ok(())
}
