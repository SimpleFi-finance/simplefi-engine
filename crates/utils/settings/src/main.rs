use clap::Parser;
use simplefi_engine_settings::{MySettings, Settings, store_settings};

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
        let rpc_key = settings.rpc_key;
        let rpc_provider = settings.rpc_provider;
        let local_storage = settings.local_storage;
        let log_level = settings.log_level;
        let log_file = settings.log_file;

        MySettings {
            rpc_provider, 
            rpc_key,
            local_storage,
            log_level,
            log_file,
        }
    };

    println!("*** Saving Settings...");

    // store settings to a local file using confy
    store_settings(&my_local_settings).expect("Failed to store settings");

    println!("*** Settings saved successfully...");
}


