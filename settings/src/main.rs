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
    let my_local_settings: MySettings =  MySettings::new(
        settings.rabbit_mq_url,
        settings.gooogle_service_account_file,
        settings.infura_token,
        settings.cloud_bucket,
        settings.local_storage,
        settings.infura_mainnet_rpc,
        settings.infura_mainnet_ws,
        settings.local_mainnet_rpc,
        settings.local_mainnet_ws,
    );

    println!("*** Saving Settings...");

    // store settings to a local file using confy
    store_settings(&my_local_settings).expect("Failed to store settings");

    println!("*** Settings saved successfully...");
}


