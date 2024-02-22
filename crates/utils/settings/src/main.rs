use clap::Parser;
use simp_settings::{MySettings, Settings, store_settings};
use tracing::info;
fn main() {
    info!(" Running Settings Generator ");
    info!("");
    info!(
        " Settings Path: {:?} ",
        confy::get_configuration_file_path("simplefi_engine", Some("settings"))
    );
    info!("");
    info!("** Parsing Data...");

    let settings = Settings::parse();

    info!("* {:#?}", settings);

    // load settings from a local file using confy
    let my_local_settings: MySettings =  {
        let rpc_key = settings.rpc_key;
        let rpc_provider = settings.rpc_provider;
        let chain_id = settings.chain_id;
        let local_storage = settings.local_storage;
        let log_level = settings.log_level;
        let log_file = settings.log_file;

        MySettings {
            rpc_provider, 
            chain_id,
            rpc_key,
            local_storage,
            log_level,
            log_file,
        }
    };

    info!(" Saving Settings...");

    // store settings to a local file using confy
    store_settings(&my_local_settings).expect("Failed to store settings");
}