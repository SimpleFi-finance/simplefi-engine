use clap::Parser;
use log::info;

use abi_discovery::settings::{ AbiDiscoverySettings, MyAbiDiscoverySettings, store_settings};
use shared_utils::logger::init_logging;

fn main() {
    // initialize logging
    init_logging();

    info!("**** Running Abi Discovery Settings Generator ****");
    info!("****");
    info!(
        "**** Settings Path: {:?} ****",
        confy::get_configuration_file_path("simplefi_engine", Some("abi_discovery_settings"))
    );
    info!("****");
    info!("**** Parsing Data...");

    let settings = AbiDiscoverySettings::parse();

    info!("*** {:#?}", settings);

    // load settings from a local file using confy
    let my_local_settings: MyAbiDiscoverySettings = {
        let redis_uri = settings.redis_uri;
        let redis_abi_key_prefix = settings.redis_abi_key_prefix;
        let redis_key_ttl_expire_ms = settings.redis_key_ttl_expire_ms;
        let etherscan_api_keys = settings.etherscan_api_keys;
        let redis_tracked_addresses_set = settings.redis_tracked_addresses_set;
        let mongodb_uri = settings.mongodb_uri;
        let mongodb_database_name = settings.mongodb_database_name;
        let mongodb_abi_collection = settings.mongodb_abi_collection;
        let mongodb_contract_abi_collection = settings.mongodb_contract_abi_collection;
        let mongodb_abi_events_collection = settings.mongodb_abi_events_collection;
        let mongodb_factory_contracts_collection = settings.mongodb_factory_contracts_collection;
        let rabbit_mq_url = settings.rabbit_mq_url;
        let rabbit_exchange_name = settings.rabbit_exchange_name;
        MyAbiDiscoverySettings {
            redis_uri,
            redis_abi_key_prefix,
            redis_key_ttl_expire_ms,
            redis_tracked_addresses_set,
            etherscan_api_keys,
            mongodb_uri,
            mongodb_database_name,
            mongodb_abi_collection,
            mongodb_contract_abi_collection,
            mongodb_abi_events_collection,
            mongodb_factory_contracts_collection,
            rabbit_mq_url,
            rabbit_exchange_name,
        }
    };


    info!("*** Saving Settings...");

    // store settings to a local file using confy
    store_settings(&my_local_settings).expect("Failed to store settings");

    info!("*** Settings saved successfully...");
}


