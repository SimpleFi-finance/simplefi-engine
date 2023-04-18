use clap::Parser;
use settings::{Settings, MySettings, store_settings};

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
    let my_local_settings: MySettings =  {
        let rabbit_mq_url = settings.rabbit_mq_url;
        let abi_discovery_exchange_name = settings.abi_discovery_exchange_name;
        let gooogle_service_account_file = settings.gooogle_service_account_file;
        let infura_token = settings.infura_token;
        let etherscan_api_keys = settings.etherscan_api_keys;
        let cloud_bucket = settings.cloud_bucket;
        let local_storage = settings.local_storage;
        let infura_mainnet_rpc = settings.infura_mainnet_rpc;
        let infura_mainnet_ws = settings.infura_mainnet_ws;
        let local_mainnet_rpc = settings.local_mainnet_rpc;
        let local_mainnet_ws = settings.local_mainnet_ws;
        let mongodb_uri = settings.mongodb_uri;
        let mongodb_engine_db = settings.mongodb_engine_db;
        let redis_uri = settings.redis_uri;
        let log_level = settings.log_level;
        let log_file = settings.log_file;

        MySettings {
            rabbit_mq_url,
            abi_discovery_exchange_name,
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
            mongodb_engine_db,
            redis_uri,
            log_level,
            log_file,
        }
    };

    println!("*** Saving Settings...");

    // store settings to a local file using confy
    store_settings(&my_local_settings).expect("Failed to store settings");

    println!("*** Settings saved successfully...");
}


