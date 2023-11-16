use clap::{Parser, command};
use confy::{load, store, ConfyError};
use serde::{Deserialize, Serialize};
use simp_primitives::{ChainSpec, Chain};

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

    #[arg(short = 'R', long = "rpc_provider", help = "RPC Provider")]
    pub rpc_provider: RpcProvider,

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
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MySettings {
    pub rpc_key: String,
    pub chain_id: u64,
    pub rpc_provider: RpcProvider,
    pub local_storage: std::path::PathBuf,
    pub log_level: String,
    pub log_file: String,
}

impl MySettings {
    pub fn new(
        rpc_key: String,
        rpc_provider: RpcProvider,
        chain_id: u64,
        local_storage: std::path::PathBuf,
        log_level: String,
        log_file: String,
    ) -> Self {
        MySettings {
            rpc_key,
            chain_id,
            rpc_provider,
            local_storage,
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
