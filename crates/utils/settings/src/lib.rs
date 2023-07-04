use clap::{Parser, command};
use confy::{load, store, ConfyError};
use serde::{Deserialize, Serialize};
use simp_primitives::ChainSpec;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RpcProvider {
    Infura,
    Local,
    Alchemy,
}

impl From<String> for RpcProvider {
    fn from(s: String) -> Self {
        match s.as_str() {
            "infura" => RpcProvider::Infura,
            "local" => RpcProvider::Local,
            "alchemy" => RpcProvider::Alchemy,
            _ => RpcProvider::Infura,
        }
    }
}

impl From<&str> for RpcProvider {
    fn from(s: &str) -> Self {
        match s {
            "infura" => RpcProvider::Infura,
            "local" => RpcProvider::Local,
            "alchemy" => RpcProvider::Alchemy,
            _ => RpcProvider::Infura,
        }
    }
}

impl ToString for RpcProvider {
    fn to_string(&self) -> String {
        match self {
            RpcProvider::Infura => "infura".to_string(),
            RpcProvider::Local => "local".to_string(),
            RpcProvider::Alchemy => "alchemy".to_string(),
        }
    }
}

impl Default for RpcProvider {
    fn default() -> Self {
        RpcProvider::Local
    }
}

impl RpcProvider {
    pub fn rpc_endpoint(&self, chain: ChainSpec) -> String {
        let chain_id = chain.chain().id();
        let rpc_key = load_settings().unwrap().rpc_key;

        let uri = match chain_id {
            1 => {
                match self {
                    RpcProvider::Infura => {
                        "https://mainnet.infura.io/v3/__key__".to_string()
                    },
                    RpcProvider::Local => {
                        "http://localhost:8545".to_string()
                    },
                    RpcProvider::Alchemy => {
                        "http://eth-mainnet.g.alchemy.com/v2/__key__".to_string()
                    },
                }
            },
            10 => {
                match self {
                    RpcProvider::Infura => {
                        "https://optimism-mainnet.infura.io/v3__key__".to_string()
                    },
                    RpcProvider::Local => {
                        "http://localhost:8545".to_string()
                    },
                    RpcProvider::Alchemy => {
                        "https://opt-mainnet.g.alchemy.com/v2/__key__".to_string()
                    },
                }
            },
            _ => panic!("Chain not supported")
        };

        uri.replace("__key__", rpc_key.as_str())
    }

    pub fn ws_endpoint(&self, chain: ChainSpec) -> String {
        let chain_id = chain.chain().id();
        let rpc_key = load_settings().unwrap().rpc_key;

        let uri = match chain_id {
            1 => {
                match self {
                    RpcProvider::Infura => {
                        "wss://mainnet.infura.io/ws/v3/__key__".to_string()
                    },
                    RpcProvider::Local => {
                        "wss://localhost:8545".to_string()
                    },
                    RpcProvider::Alchemy => {
                        "wss://eth-mainnet.g.alchemy.com/v2/__key__".to_string()
                    },
                }
            },
            10 => {
                match self {
                    RpcProvider::Infura => {
                        panic!("Optimism wss not supported in Infura")
                    },
                    RpcProvider::Local => {
                        "wss://localhost:8545".to_string()
                    },
                    RpcProvider::Alchemy => {
                        "wss://opt-mainnet.g.alchemy.com/v2/__key__".to_string()
                    },
                }
            },
            _ => panic!("Chain not supported")
        };

        uri.replace("__key__", rpc_key.as_str())
    }
}

#[derive(Parser, Debug)]
#[command(author = "SimpleFi Finance")]
#[command(version)]
#[command(about = "Settings Generator")]
#[command(
    long_about = "Settings Generator enhaces the running experience generating a settings files with required and default properties."
)]
#[command(next_line_help = true)]
pub struct Settings {

    #[arg(short = 'K', long = "rpc_key", help = "RPC Auth Token")]
    pub rpc_key: String,

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
        default_value = "simplefi-data-lake"
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
        short = 'C',
        long = "chain",
        help = "Chain ID of the system",
        required = true
    )]
    pub chain_id: u64,

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

    #[arg(
        long = "protocol_status_gold_collection_name",
        help = "MongoDB protocol status gold collection name in mongo DB",
        default_value = "protocol_status_gold"
    )]
    pub protocol_status_gold_collection_name: String,

    #[arg(
        long = "volumetrics_five_minute_collection_name",
        help = "MongoDB volumetrics five minute gold collection name in mongo DB",
        default_value = "volumetrics_five_minute_gold"
    )]
    pub volumetrics_five_minute_gold_collection_name: String,
    #[arg(
        long = "volumetrics_hourly_collection_name",
        help = "MongoDB volumetrics hourly gold collection name in mongo DB",
        default_value = "volumetrics_hourly_gold"
    )]
    pub volumetrics_hourly_gold_collection_name: String,

    #[arg(
        long = "volumetrics_daily_collection_name",
        help = "MongoDB volumetrics daily collection name in mongo DB",
        default_value = "volumetrics_daily_gold"
    )]
    pub volumetrics_daily_gold_collection_name: String,

    #[arg(
        long = "snapshots_five_minute_gold_collection_name",
        help = "MongoDB snapshots five minute gold collection name in mongo DB",
        default_value = "snapshots_five_minute_gold"
    )]
    pub snapshots_five_minute_gold_collection_name: String,

    #[arg(
        long = "snapshots_hourly_gold_collection_name",
        help = "MongoDB snapshots hourly gold collection name in mongo DB",
        default_value = "snapshots_hourly_gold"
    )]
    pub snapshots_hourly_gold_collection_name: String,

    #[arg(
        long = "snapshots_daily_gold_collection_name",
        help = "MongoDB snapshots daily gold collection name in mongo DB",
        default_value = "snapshots_daily_gold"
    )]
    pub snapshots_daily_gold_collection_name: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MySettings {
    pub rpc_key: String,
    pub chain_id: u64,
    pub rpc_provider: RpcProvider,
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
    pub protocol_status_gold_collection_name: String,
    pub snapshots_five_minute_gold_collection_name: String,
    pub snapshots_hourly_gold_collection_name: String,
    pub snapshots_daily_gold_collection_name: String,
    pub volumetrics_five_minute_gold_collection_name: String,
    pub volumetrics_hourly_gold_collection_name: String,
    pub volumetrics_daily_gold_collection_name: String,
    pub abi_collection_name: String,
    pub contract_abi_collection_name: String,
    pub redis_uri: String,
    pub log_level: String,
    pub log_file: String,
}

impl MySettings {
    pub fn new(
        rpc_key: String,
        rpc_provider: RpcProvider,
        chain_id: u64,
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
        abi_collection_name: String,
        contract_abi_collection_name: String,
        protocol_status_gold_collection_name: String,
        snapshots_five_minute_gold_collection_name: String,
        snapshots_hourly_gold_collection_name: String,
        snapshots_daily_gold_collection_name: String,
        volumetrics_five_minute_gold_collection_name: String,
        volumetrics_hourly_gold_collection_name: String,
        volumetrics_daily_gold_collection_name: String,
        redis_uri: String,
        log_level: String,
        log_file: String,
    ) -> Self {
        MySettings {
            rpc_key,
            chain_id,
            rpc_provider,
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
            protocol_status_gold_collection_name,
            snapshots_five_minute_gold_collection_name,
            snapshots_hourly_gold_collection_name,
            snapshots_daily_gold_collection_name,
            volumetrics_five_minute_gold_collection_name,
            volumetrics_hourly_gold_collection_name,
            volumetrics_daily_gold_collection_name,
            abi_collection_name,
            contract_abi_collection_name,
            redis_uri,
            log_level,
            log_file,
        }
    }
}

pub fn load_settings() -> Result<MySettings, ConfyError> {
    let default_settings = MySettings {
        rpc_key: String::from("default_infura_token"),
        rpc_provider: RpcProvider::default(),
        chain_id: 1,
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
        abi_collection_name: String::from("abis"),
        contract_abi_collection_name: String::from("contract_abis"),
        protocol_status_gold_collection_name: String::from("protocol_status_gold"),
        snapshots_five_minute_gold_collection_name: String::from("snapshots_five_minute_gold"),
        snapshots_hourly_gold_collection_name: String::from("snapshots_hourly_gold"),
        snapshots_daily_gold_collection_name: String::from("snapshots_daily_gold"),
        volumetrics_five_minute_gold_collection_name: String::from("snapshots_five_minute_gold"),
        volumetrics_hourly_gold_collection_name: String::from("snapshots_hourly_gold"),
        volumetrics_daily_gold_collection_name: String::from("snapshots_daily_gold"),

        // redis
        redis_uri: String::from("redis://localhost:6379/"),
        // logging
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
