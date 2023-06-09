use clap::{ Parser };
use confy::{load, store, ConfyError};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author = "SimpleFi Finance")]
#[command(version)]
#[command(about = "Logs listener Settings Generator")]
#[command(
    long_about = "Logs Listener Settings Generator enhaces the running experience generating a settings files with required and default properties."
)]
#[command(next_line_help = true)]
pub struct ChainSyncDbSettings {
    // MongoDB Settings
    #[arg(
        long = "mongodb_uri",
        help = "MongoDB URI",
        default_value = "mongodb://localhost:27017/"
    )]
    pub mongodb_uri: String,

    #[arg(
        long = "chain",
        help = "chain",
        default_value = "ethereum"
    )]
    pub chain: String,
    
    #[arg(
        long = "mongodb_database_name",
        help = "MongoDB database name DB",
        default_value = "simplefi_staged_data"
    )]
    pub mongodb_database_name: String,

    #[arg(
        long = "logs_collection_name",
        help = "MongoDB logs collection name in mongo DB",
        default_value = "logs"
    )]
    pub logs_collection_name: String,

    #[arg(
        long = "txs_collection_name",
        help = "MongoDB txs collection name in mongo DB",
        default_value = "txs"
    )]
    pub txs_collection_name: String,

    #[arg(
        long = "blocks_collection_name",
        help = "MongoDB blocks collection name in mongo DB",
        default_value = "blocks"
    )]
    pub blocks_collection_name: String,
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MyChainSyncDbSettings {
    pub mongodb_uri: String,
    pub mongodb_database_name: String,
    pub logs_collection_name: String,
    pub txs_collection_name: String,
    pub blocks_collection_name: String,
    pub chain: String,
}



impl MyChainSyncDbSettings {
    pub fn new(
        chain: String,
        mongodb_uri:String,
        mongodb_database_name:String,
        logs_collection_name:String,
        txs_collection_name:String,
        blocks_collection_name:String,
    ) -> Self {
        MyChainSyncDbSettings {
            chain,
            mongodb_uri,
            mongodb_database_name,
            logs_collection_name,
            txs_collection_name,
            blocks_collection_name,
        }
    }
}


pub fn load_settings() -> Result<MyChainSyncDbSettings, ConfyError> {
    let default_settings = MyChainSyncDbSettings {
        mongodb_uri: String::from("mongodb://localhost:27017/"),
        mongodb_database_name: String::from("simplefi_staged_data"),
        logs_collection_name: String::from("logs"),
        txs_collection_name: String::from("txs"),
        blocks_collection_name: String::from("blocks"),
        chain: String::from("ethereum"),
    };

    let settings: MyChainSyncDbSettings = load("simplefi_engine", Some("chain_sync_to_db_settings")).unwrap_or(default_settings);

    Ok(settings)
}

pub fn store_settings(settings: &MyChainSyncDbSettings) -> Result<(), ConfyError> {
    store("simplefi_engine", Some("chain_sync_to_db_settings"), &settings).expect("Failed to store abi discovery settings");

    Ok(())
}
