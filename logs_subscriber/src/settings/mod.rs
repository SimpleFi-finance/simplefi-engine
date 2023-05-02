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

    pub chain: String,
}



impl MyLogsSubscriberSettings {
    pub fn new(
        chain: String,
    ) -> Self {
        MyLogsSubscriberSettings {
            chain
        }
    }
}


pub fn load_settings() -> Result<MyLogsSubscriberSettings, ConfyError> {
    let default_settings = MyLogsSubscriberSettings {
        chain: String::from("ethereum"),
    };

    let settings: MyLogsSubscriberSettings = load("simplefi_engine", Some("logs_tracker_settings")).unwrap_or(default_settings);

    Ok(settings)
}

pub fn store_settings(settings: &MyLogsSubscriberSettings) -> Result<(), ConfyError> {
    store("simplefi_engine", Some("logs_tracker_settings"), &settings).expect("Failed to store abi discovery settings");

    Ok(())
}
