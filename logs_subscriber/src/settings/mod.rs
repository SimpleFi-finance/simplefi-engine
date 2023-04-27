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
pub struct LogsSubscriberSettings {
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
        long = "logs_bronze_collection_name",
        help = "MongoDB bronze logs collection name in mongo DB",
        default_value = "logs_bronze"
    )]
    pub logs_bronze_collection_name: String,
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MyLogsSubscriberSettings {
    // MongoDB Settings
    pub mongodb_uri: String,
    pub mongodb_engine_db: String,
    pub mongodb_database_name: String,
    pub logs_bronze_collection_name: String,
    pub chain: String,
}



impl MyLogsSubscriberSettings {
    pub fn new(
        mongodb_uri: String,
        mongodb_engine_db: String,
        mongodb_database_name: String,
        logs_bronze_collection_name: String,
        chain: String,
    ) -> Self {
        MyLogsSubscriberSettings {
            mongodb_uri,
            mongodb_engine_db,
            mongodb_database_name,
            logs_bronze_collection_name,
            chain
        }
    }
}


pub fn load_settings() -> Result<MyLogsSubscriberSettings, ConfyError> {
    let default_settings = MyLogsSubscriberSettings {
        mongodb_uri: String::from("mongodb://localhost:27017/"),
        mongodb_engine_db: String::from("simplefi_engine"),
        mongodb_database_name: String::from("simplefi_data"),
        logs_bronze_collection_name: String::from("logs_bronze"),
        chain: String::from("ethereum"),
    };

    let settings: MyLogsSubscriberSettings = load("simplefi_engine", Some("logs_tracker_settings")).unwrap_or(default_settings);

    Ok(settings)
}

pub fn store_settings(settings: &MyLogsSubscriberSettings) -> Result<(), ConfyError> {
    store("simplefi_engine", Some("logs_tracker_settings"), &settings).expect("Failed to store abi discovery settings");

    Ok(())
}
