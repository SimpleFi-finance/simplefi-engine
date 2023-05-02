use clap::Parser;
use logs_subscriber::settings::{LogsSubscriberSettings, MyLogsSubscriberSettings, store_settings};

fn main() {
    println!("**** Running Abi Discovery Settings Generator ****");
    println!("****");
    println!(
        "**** Settings Path: {:?} ****",
        confy::get_configuration_file_path("simplefi_engine", Some("chain_tracker_settings"))
    );
    println!("****");
    println!("**** Parsing Data...");

    let settings = LogsSubscriberSettings::parse();

    println!("*** {:#?}", settings);

    // load settings from a local file using confy
    let my_local_settings: MyLogsSubscriberSettings = {
        let chain = settings.chain;
        MyLogsSubscriberSettings {
            chain,
        }
    };


    println!("*** Saving Settings...");

    // store settings to a local file using confy
    store_settings(&my_local_settings).expect("Failed to store settings");

    println!("*** Settings saved successfully...");
}


