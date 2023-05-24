use chains_drivers::ethereum::mainnet::ethereum_mainnet;
use settings::load_settings as load_global_settings;
use third_parties::{
    broker::{
        bind_queue_to_exchange, create_rmq_channel, declare_exchange, declare_rmq_queue,
        publish_rmq_message,
    },
    mongo::{lib::bronze::blocks::{setters::save_blocks, types::Block}, Mongo, MongoConfig},
};

use shared_utils::logger::init_logging;

#[tokio::main]
async fn main() {
    // connects to node wss endpoint and listens to new blocks (can store block data as it comes in)
    // todo load settings and select chain
    let chain_name = "ethereum_mainnet"; //todo switch to settings

    let chain = match chain_name {
        "ethereum_mainnet" => ethereum_mainnet(),
        _ => panic!("Chain not found")
    };

    let glob_settings = load_global_settings().unwrap();

    init_logging();

    // load method directly
    
}
