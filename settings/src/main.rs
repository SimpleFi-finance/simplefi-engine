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
        let new_blocks_queue_name = settings.new_blocks_queue_name;
        let new_block_exchange_name = settings.new_block_exchange_name;
        let gooogle_service_account_file = settings.gooogle_service_account_file;
        let infura_token = settings.infura_token;
        let cloud_bucket = settings.cloud_bucket;
        let local_storage = settings.local_storage;
        let infura_mainnet_rpc = settings.infura_mainnet_rpc;
        let infura_mainnet_ws = settings.infura_mainnet_ws;
        let local_mainnet_rpc = settings.local_mainnet_rpc;
        let local_mainnet_ws = settings.local_mainnet_ws;
        let mongodb_uri = settings.mongodb_uri;
        let mongodb_database_name = settings.mongodb_database_name;
        let logs_bronze_collection_name = settings.logs_bronze_collection_name;
        let txs_bronze_collection_name = settings.txs_bronze_collection_name;
        let blocks_bronze_collection_name = settings.blocks_bronze_collection_name;
        let decoding_error_bronze_collection_name = settings.decoding_error_bronze_collection_name;
        let abi_collection_name = settings.abi_collection_name;
        let contract_abi_collection_name = settings.contract_abi_collection_name;
        let redis_uri = settings.redis_uri;
        let log_level = settings.log_level;
        let log_file = settings.log_file;

        MySettings {
            new_blocks_queue_name,
            new_block_exchange_name,
            gooogle_service_account_file,
            infura_token,
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
            abi_collection_name,
            contract_abi_collection_name,
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


