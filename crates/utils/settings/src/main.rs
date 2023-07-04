use clap::Parser;
use simplefi_engine_settings::{store_settings, MySettings, Settings};

fn main() {
    info!("**** Running Settings Generator ****");
    info!("****");
    info!(
        "**** Settings Path: {:?} ****",
        confy::get_configuration_file_path("simplefi_engine", Some("settings"))
    );
    info!("****");
    info!("**** Parsing Data...");

    let settings = Settings::parse();

    info!("*** {:#?}", settings);

    // load settings from a local file using confy
    let my_local_settings: MySettings = {
        let new_blocks_queue_name = settings.new_blocks_queue_name;
        let new_block_exchange_name = settings.new_block_exchange_name;
        let gooogle_service_account_file = settings.gooogle_service_account_file;
        let infura_token = settings.infura_token;
        let cloud_bucket = settings.cloud_bucket;
        let local_storage = settings.local_storage;
        let log_level = settings.log_level;
        let log_file = settings.log_file;
        let protocol_status_gold_collection_name = settings.protocol_status_gold_collection_name;
        let snapshots_five_minute_gold_collection_name =
            settings.snapshots_five_minute_gold_collection_name;
        let snapshots_hourly_gold_collection_name = settings.snapshots_hourly_gold_collection_name;
        let snapshots_daily_gold_collection_name = settings.snapshots_daily_gold_collection_name;
        let volumetrics_five_minute_gold_collection_name =
            settings.volumetrics_five_minute_gold_collection_name;
        let volumetrics_hourly_gold_collection_name =
            settings.volumetrics_hourly_gold_collection_name;
        let volumetrics_daily_gold_collection_name =
            settings.volumetrics_daily_gold_collection_name;

        MySettings {
            rpc_provider, 
            chain_id,
            rpc_key,
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
            protocol_status_gold_collection_name,
            snapshots_five_minute_gold_collection_name,
            snapshots_hourly_gold_collection_name,
            snapshots_daily_gold_collection_name,
            volumetrics_five_minute_gold_collection_name,
            volumetrics_hourly_gold_collection_name,
            volumetrics_daily_gold_collection_name,
            abi_collection_name,
            contract_abi_collection_name,
            redis_uri,
            log_level,
            log_file,
        }
    };

    info!("*** Saving Settings...");

    // store settings to a local file using confy
    store_settings(&my_local_settings).expect("Failed to store settings");

    info!("*** Settings saved successfully...");
}
