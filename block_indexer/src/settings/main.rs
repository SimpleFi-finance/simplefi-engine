use clap::Parser;
use block_indexer::settings::{ store_settings, ChainIndexerSettings, MyChainIndexerSettings};

fn main() {
    println!("**** Running Abi Discovery Settings Generator ****");
    println!("****");
    println!(
        "**** Settings Path: {:?} ****",
        confy::get_configuration_file_path("simplefi_engine", Some("chain_tracker_settings"))
    );
    println!("****");
    println!("**** Parsing Data...");

    let settings = ChainIndexerSettings::parse();

    println!("*** {:#?}", settings);

    // load settings from a local file using confy
    let my_local_settings: MyChainIndexerSettings = {
        let mongodb_uri = settings.mongodb_uri;
        let mongodb_engine_db = settings.mongodb_engine_db;
        let mongodb_database_name = settings.mongodb_database_name;
        let logs_bronze_collection_name = settings.logs_bronze_collection_name;
        let txs_bronze_collection_name = settings.txs_bronze_collection_name;
        let rabbit_mq_url = settings.rabbit_mq_url;
        let new_blocks_queue_name = settings.new_blocks_queue_name;
        let new_block_exchange_name = settings.new_block_exchange_name;
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
    };


    println!("*** Saving Settings...");

    // store settings to a local file using confy
    store_settings(&my_local_settings).expect("Failed to store settings");

    println!("*** Settings saved successfully...");
}


