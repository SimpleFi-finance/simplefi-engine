use clap::Parser;
use confy::{load, store, ConfyError};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author = "SimpleFi Finance")]
#[command(version)]
#[command(about = "Settings Generator")]
#[command(
    long_about = "Settings Generator enhaces the running experience generating a settings files with required and default properties."
)]
#[command(next_line_help = true)]
struct Settings {
    #[arg(
        short = 'M',
        long = "mq_url",
        default_value = "amqp://guest:guest@localhost:5672",
        help = "RabbitMQ URL"
    )]
    rabbit_mq_url: String,

    #[arg(
        short = 'G',
        long = "google_service_account",
        help = "Google Service Account JSON file"
    )]
    gooogle_service_account_file: std::path::PathBuf,

    #[arg(short = 'I', long = "infura_token", help = "Infura Auth Token")]
    infura_token: String,

    #[arg(
        short = 'C',
        long = "cloud_bucket",
        help = "Cloud Bucket to store the data",
        required = false
    )]
    cloud_bucket: String,

    #[arg(
        short = 'L',
        long = "local_storage",
        help = "Path to store the data locally",
        required = false
    )]
    local_storage: std::path::PathBuf,

    // Nodes
    #[arg(
        long = "infura_mainnet_rpc",
        help = "Infura Mainnet RPC Node",
        default_value = "https://mainnet.infura.io/v3/"
    )]
    infura_mainnet_rpc: String,

    #[arg(
        long = "infura_mainnet_ws",
        help = "Infura Mainnet WS Node",
        default_value = "https://mainnet.infura.io/ws/v3/"
    )]
    infura_mainnet_ws: String,

    #[arg(
        long = "local_mainnet_rpc",
        help = "Local Mainnet RPC Node",
        default_value = "http://localhost:8545"
    )]
    local_mainnet_rpc: String,

    #[arg(
        long = "local_mainnet_ws",
        help = "Local Mainnet WS Node",
        default_value = "wss://localhost:8545"
    )]
    local_mainnet_ws: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MySettings {
    rabbit_mq_url: String,
    gooogle_service_account_file: std::path::PathBuf,
    infura_token: String,
    cloud_bucket: String,
    local_storage: std::path::PathBuf,
    infura_mainnet_rpc: String,
    infura_mainnet_ws: String,
    local_mainnet_rpc: String,
    local_mainnet_ws: String,
}

fn main() {
    println!("**** Running Settings Generator ****");
    println!("****");
    println!(
        "**** Settings Path: {:?} ****",
        confy::get_configuration_file_path("simplefi_engine", Some("settings"))
    );
    println!("****");
    println!("**** Parsing Data...");

    let settings = Settings::parse();

    println!("*** {:#?}", settings);

    // load settings from a local file using confy
    let mut my_local_settings = load_settings().unwrap_or_default();

    my_local_settings.rabbit_mq_url = settings.rabbit_mq_url;
    my_local_settings.gooogle_service_account_file = settings.gooogle_service_account_file;
    my_local_settings.infura_token = settings.infura_token;
    my_local_settings.cloud_bucket = settings.cloud_bucket;
    my_local_settings.local_storage = settings.local_storage;
    my_local_settings.infura_mainnet_rpc = settings.infura_mainnet_rpc;
    my_local_settings.infura_mainnet_ws = settings.infura_mainnet_ws;
    my_local_settings.local_mainnet_rpc = settings.local_mainnet_rpc;
    my_local_settings.local_mainnet_ws = settings.local_mainnet_ws;

    println!("*** Saving Settings...");

    // store settings to a local file using confy
    store_settings(my_local_settings).expect("Failed to store settings");

    println!("*** Settings saved successfully...");
}

pub fn load_settings() -> Result<MySettings, ConfyError> {
    let default_settings = MySettings {
        rabbit_mq_url: String::from("amqp://guest:guest@localhost:5672"),
        gooogle_service_account_file: std::path::PathBuf::from(
            "default_google_service_account.json",
        ),
        infura_token: String::from("default_infura_token"),
        cloud_bucket: String::from("default_cloud_bucket"),
        local_storage: std::path::PathBuf::from("default_local_storage"),
        infura_mainnet_rpc: String::from("https://mainnet.infura.io/v3/"),
        infura_mainnet_ws: String::from("https://mainnet.infura.io/ws/v3/"),
        local_mainnet_rpc: String::from("http://localhost:8545"),
        local_mainnet_ws: String::from("wss://localhost:8545"),
    };

    let settings: MySettings =
        load("simplefi_engine", Some("settings")).unwrap_or(default_settings);

    Ok(settings)
}

fn store_settings(settings: MySettings) -> Result<(), ConfyError> {
    store("simplefi_engine", Some("settings"), &settings).expect("Failed to store settings");

    Ok(())
}
