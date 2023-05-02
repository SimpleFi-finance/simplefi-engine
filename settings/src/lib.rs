use clap::Parser;
use confy::{load, store, ConfyError};
use serde::{Deserialize, Serialize};

pub mod helpers;

#[derive(Parser, Debug)]
#[command(author = "SimpleFi Finance")]
#[command(version)]
#[command(about = "Settings Generator")]
#[command(
    long_about = "Settings Generator enhaces the running experience generating a settings files with required and default properties."
)]
#[command(next_line_help = true)]
pub struct Settings {
    #[arg(
        short = 'M',
        long = "mq_url",
        default_value = "amqp://guest:guest@localhost:5672",
        help = "RabbitMQ URL"
    )]
    pub rabbit_mq_url: String,

    #[arg(
        long = "abi_discovery_exchange_name",
        default_value = "abi_discovery_exchange",
        help = "RabbitMQ exchange name for ABI Discovery"
    )]
    pub abi_discovery_exchange_name: String,

    #[arg(
        long = "new_blocks_queue_name",
        help = "Rabbit MQ new Block queue name",
        default_value = "new_block_queue"
    )]
    pub new_blocks_queue_name: String,
    
    #[arg(
        long = "new_block_exchange_name",
        help = "Rabbit MQ new Block exchange name",
        default_value = "new_block_exchange"
    )]
    pub new_block_exchange_name: String,

    #[arg(
        short = 'G',
        long = "google_service_account",
        help = "Google Service Account JSON file"
    )]
    pub gooogle_service_account_file: std::path::PathBuf,

    #[arg(short = 'I', long = "infura_token", help = "Infura Auth Token")]
    pub infura_token: String,

    #[arg(
        short = 'C',
        long = "cloud_bucket",
        help = "Cloud Bucket to store the data",
        required = false
    )]
    pub cloud_bucket: String,

    #[arg(
        short = 'L',
        long = "local_storage",
        help = "Path to store the data locally",
        required = false
    )]
    pub local_storage: std::path::PathBuf,

    #[arg(
        short = 'R',
        long = "redis_uri",
        help = "Redis DB URI",
        required = false
    )]
    pub redis_uri: String,

    // Nodes
    #[arg(
        long = "infura_mainnet_rpc",
        help = "Infura Mainnet RPC Node",
        default_value = "https://mainnet.infura.io/v3/"
    )]
    pub infura_mainnet_rpc: String,

    // Etherscan API Key
    #[arg(short = 'E', long = "etherscan_api_keys", help = "Etherscan API key")]
    pub etherscan_api_keys: String,

    #[arg(
        long = "infura_mainnet_ws",
        help = "Infura Mainnet WS Node",
        default_value = "https://mainnet.infura.io/ws/v3/"
    )]
    pub infura_mainnet_ws: String,

    #[arg(
        long = "local_mainnet_rpc",
        help = "Local Mainnet RPC Node",
        default_value = "http://localhost:8545"
    )]
    pub local_mainnet_rpc: String,

    #[arg(
        long = "local_mainnet_ws",
        help = "Local Mainnet WS Node",
        default_value = "wss://localhost:8545"
    )]
    pub local_mainnet_ws: String,

    #[arg(
        long = "mongodb_uri",
        help = "MongoDB URI",
        default_value = "mongodb://localhost:27017/"
    )]
    pub mongodb_uri: String,

    #[arg(
        long = "mongodb_database_name",
        help = "MongoDB Engine DB",
        default_value = "engine"
    )]
    pub mongodb_database_name: String,

    #[arg(
        long = "logs_bronze_collection_name",
        help = "MongoDB bronze logs collection name in mongo DB",
        default_value = "logs_bronze"
    )]
    pub logs_bronze_collection_name: String,

    #[arg(
        long = "txs_bronze_collection_name",
        help = "MongoDB bronze txs collection name in mongo DB",
        default_value = "txs_bronze"
    )]
    pub txs_bronze_collection_name: String,

    #[arg(
        long = "blocks_bronze_collection_name",
        help = "MongoDB bronze blocks collection name in mongo DB",
        default_value = "blocks_bronze"
    )]
    pub blocks_bronze_collection_name: String,

    #[arg(
        long = "decoding_error_bronze_collection_name",
        help = "MongoDB bronze decoding error collection name in mongo DB",
        default_value = "decoding_error_bronze"
    )]
    pub decoding_error_bronze_collection_name: String,

    #[arg(
        long = "log_level",
        help = "Log level to filter the logs. Default is INFO",
        default_value = "INFO"
    )]
    pub log_level: String,

    #[arg(
        long = "log_file",
        help = "File name to store the logs. Keep it empty to disable file logging and show all logs in console.",
        default_value = ""
    )]
    pub log_file: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MySettings {
    pub rabbit_mq_url: String,
    pub abi_discovery_exchange_name: String,
    pub new_blocks_queue_name: String,
    pub new_block_exchange_name: String,
    pub gooogle_service_account_file: std::path::PathBuf,
    pub infura_token: String,
    pub etherscan_api_keys: String,
    pub cloud_bucket: String,
    pub local_storage: std::path::PathBuf,
    pub infura_mainnet_rpc: String,
    pub infura_mainnet_ws: String,
    pub local_mainnet_rpc: String,
    pub local_mainnet_ws: String,
    pub mongodb_uri: String,
    pub mongodb_database_name: String,
    pub logs_bronze_collection_name: String,
    pub txs_bronze_collection_name: String,
    pub blocks_bronze_collection_name: String,
    pub decoding_error_bronze_collection_name: String,
    pub redis_uri: String,
    pub log_level: String,
    pub log_file: String,
}

impl MySettings {
    pub fn new(
        rabbit_mq_url: String,
        abi_discovery_exchange_name: String,
        new_blocks_queue_name: String,
        new_block_exchange_name: String,
        gooogle_service_account_file: std::path::PathBuf,
        infura_token: String,
        etherscan_api_keys: String,
        cloud_bucket: String,
        local_storage: std::path::PathBuf,
        infura_mainnet_rpc: String,
        infura_mainnet_ws: String,
        local_mainnet_rpc: String,
        local_mainnet_ws: String,
        mongodb_uri: String,
        mongodb_database_name: String,
        logs_bronze_collection_name: String,
        txs_bronze_collection_name: String,
        blocks_bronze_collection_name: String,
        decoding_error_bronze_collection_name: String,
        redis_uri: String,
        log_level: String,
        log_file: String,
    ) -> Self {
        MySettings {
            rabbit_mq_url,
            abi_discovery_exchange_name,
            new_blocks_queue_name,
            new_block_exchange_name,
            gooogle_service_account_file,
            infura_token,
            etherscan_api_keys,
            cloud_bucket,
            local_storage,
            infura_mainnet_rpc,
            infura_mainnet_ws,
            local_mainnet_rpc,
            local_mainnet_ws,
            mongodb_uri,
            mongodb_database_name,
            logs_bronze_collection_name,
            txs_bronze_collection_name,
            blocks_bronze_collection_name,
            decoding_error_bronze_collection_name,
            redis_uri,
            log_level,
            log_file,
        }
    }
}

pub fn load_settings() -> Result<MySettings, ConfyError> {
    let default_settings = MySettings {
        rabbit_mq_url: String::from("amqp://guest:guest@localhost:5672"),
        abi_discovery_exchange_name: String::from("abi_discovery"),
        new_blocks_queue_name: String::from("new_blocks"),
        new_block_exchange_name: String::from("new_block"),
        gooogle_service_account_file: std::path::PathBuf::from(
            "default_google_service_account.json",
        ),
        infura_token: String::from("default_infura_token"),
        etherscan_api_keys: String::from("change_etherscan_api_key_1,change_etherscan_api_key_2"),
        cloud_bucket: String::from("default_cloud_bucket"),
        local_storage: std::path::PathBuf::from("default_local_storage"),
        infura_mainnet_rpc: String::from("https://mainnet.infura.io/v3/"),
        infura_mainnet_ws: String::from("wss://mainnet.infura.io/ws/v3/"),
        local_mainnet_rpc: String::from("http://localhost:8545"),
        local_mainnet_ws: String::from("wss://localhost:8545"),
        // mongo 
        mongodb_uri: String::from("mongodb://localhost:27017/"),
        mongodb_database_name: String::from("engine"),
        logs_bronze_collection_name: String::from("logs_bronze"),
        txs_bronze_collection_name: String::from("txs_bronze"),
        blocks_bronze_collection_name: String::from("blocks_bronze"),
        decoding_error_bronze_collection_name: String::from("decoding_error_bronze"),
        redis_uri: String::from("redis://localhost:6379/"),
        log_level: String::from("INFO"),
        log_file: String::from(""),
    };

    let settings: MySettings =
        load("simplefi_engine", Some("settings")).unwrap_or(default_settings);

    Ok(settings)
}

pub fn store_settings(settings: &MySettings) -> Result<(), ConfyError> {
    store("simplefi_engine", Some("settings"), &settings).expect("Failed to store settings");

    Ok(())
}
