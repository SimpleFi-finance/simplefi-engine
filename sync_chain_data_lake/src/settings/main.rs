use clap::Parser;
use sync_chain_data_lake::settings::{ChainSyncDbSettings, MyChainSyncDbSettings, store_settings};

fn main() {
    println!("**** Running Abi Discovery Settings Generator ****");
    println!("****");
    println!(
        "**** Settings Path: {:?} ****",
        confy::get_configuration_file_path("simplefi_engine", Some("chain_tracker_settings"))
    );
    println!("****");
    println!("**** Parsing Data...");

    let settings = ChainSyncDbSettings::parse();

    println!("*** {:#?}", settings);

    // load settings from a local file using confy
    let my_local_settings: MyChainSyncDbSettings = {
        let chain = settings.chain;
        let mongodb_uri = settings.mongodb_uri;
        let mongodb_database_name = settings.mongodb_database_name;
        let logs_collection_name = settings.logs_collection_name;
        let txs_collection_name = settings.txs_collection_name;
        let blocks_collection_name = settings.blocks_collection_name;

        MyChainSyncDbSettings {
            chain,
            mongodb_uri,
            mongodb_database_name,
            logs_collection_name,
            txs_collection_name,
            blocks_collection_name,
        }
    };


    println!("*** Saving Settings...");

    // store settings to a local file using confy
    store_settings(&my_local_settings).expect("Failed to store settings");

    println!("*** Settings saved successfully...");
}


