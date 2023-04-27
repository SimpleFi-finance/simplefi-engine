use clap::{ Parser };
use confy::{load, store, ConfyError};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author = "SimpleFi Finance")]
#[command(version)]
#[command(about = "Chain Tracker Settings Generator")]
#[command(
    long_about = "Chain Tracker Settings Generator enhaces the running experience generating a settings files with required and default properties."
)]
#[command(next_line_help = true)]
pub struct ChainIndexerSettings {
    // MongoDB Settings
    #[arg(
        long = "mongodb_uri",
        help = "MongoDB URI",
        default_value = "mongodb://localhost:27017/"
    )]
    pub mongodb_uri: String,

    #[arg(
        long = "mongodb_engine_db",
        help = "MongoDB Engine DB",
        default_value = "simplefi_engine"
    )]
    pub mongodb_engine_db: String,
    
    #[arg(
        long = "mongodb_database_name",
        help = "MongoDB database name DB",
        default_value = "simplefi_data"
    )]
    pub mongodb_database_name: String,

    #[arg(
        long = "txs_bronze_collection_name",
        help = "MongoDB bronze txs collection name in mongo DB",
        default_value = "txs_bronze"
    )]
    pub txs_bronze_collection_name: String,

    #[arg(
        long = "logs_bronze_collection_name",
        help = "MongoDB bronze logs collection name in mongo DB",
        default_value = "logs_bronze"
    )]
    pub logs_bronze_collection_name: String,

    #[arg(
        long = "rabbit_mq_url",
        help = "Rabbit MQ Url",
        default_value = "amqp://guest:guest@localhost:5672"
    )]
    pub rabbit_mq_url: String,

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
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MyChainIndexerSettings {
    // MongoDB Settings
    pub mongodb_uri: String,
    pub mongodb_engine_db: String,
    pub mongodb_database_name: String,
    pub logs_bronze_collection_name: String,
    pub txs_bronze_collection_name: String,
    // RabbitMQ Settings
    pub rabbit_mq_url: String,
    pub new_blocks_queue_name: String,
    pub new_block_exchange_name: String,
}



impl MyChainIndexerSettings {
    pub fn new(
        mongodb_uri: String,
        mongodb_engine_db: String,
        mongodb_database_name: String,
        logs_bronze_collection_name: String,
        txs_bronze_collection_name: String,
        rabbit_mq_url: String,
        new_blocks_queue_name: String,
        new_block_exchange_name: String,
    ) -> Self {
        MyChainIndexerSettings {
            mongodb_uri,
            mongodb_engine_db,
            mongodb_database_name,
            logs_bronze_collection_name,
            txs_bronze_collection_name,
            rabbit_mq_url,
            new_blocks_queue_name,
            new_block_exchange_name
        }
    }
}


pub fn load_settings() -> Result<MyChainIndexerSettings, ConfyError> {
    let default_settings = MyChainIndexerSettings {
        mongodb_uri: String::from("mongodb://localhost:27017/"),
        mongodb_engine_db: String::from("simplefi_engine"),
        mongodb_database_name: String::from("simplefi_data"),
        logs_bronze_collection_name: String::from("logs_bronze"),
        txs_bronze_collection_name: String::from("txs_bronze"),
        rabbit_mq_url: String::from("amqp://guest:guest@localhost:5672"),
        new_blocks_queue_name: String::from("new_block_queue"),
        new_block_exchange_name: String::from("new_block_exchange"),
    };

    let settings: MyChainIndexerSettings = load("simplefi_engine", Some("chain_tracker_settings")).unwrap_or(default_settings);

    Ok(settings)
}

pub fn store_settings(settings: &MyChainIndexerSettings) -> Result<(), ConfyError> {
    store("simplefi_engine", Some("chain_tracker_settings"), &settings).expect("Failed to store abi discovery settings");

    Ok(())
}
