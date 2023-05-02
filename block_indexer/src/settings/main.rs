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
        MyChainIndexerSettings {
        }
    };


    println!("*** Saving Settings...");

    // store settings to a local file using confy
    store_settings(&my_local_settings).expect("Failed to store settings");

    println!("*** Settings saved successfully...");
}


