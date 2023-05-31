use clap::Parser;
use processes::settings::{ChainSettings, MyChainSettings, store_settings};

fn main() {
    println!("**** Running Processes Settings Generator ****");
    println!("****");
    println!(
        "**** Settings Path: {:?} ****",
        confy::get_configuration_file_path("simplefi_engine", Some("processes_chains_settings"))
    );
    println!("****");
    println!("**** Parsing Data...");

    let settings = ChainSettings::parse();

    println!("*** {:#?}", settings);

    // load settings from a local file using confy
    let my_local_settings: MyChainSettings = {

        MyChainSettings {
            chain: settings.chain,
        }
    };


    println!("*** Saving Settings...");

    // store settings to a local file using confy
    store_settings(&my_local_settings).expect("Failed to store settings");

    println!("*** Settings saved successfully...");
}


