use chains_drivers::{
    ethereum::mainnet::ethereum_mainnet, common::base_chain::{SubscribeBlocks},
};
use third_parties::mongo::lib::bronze::logs::types::Log;
use settings::load_settings;

#[tokio::main]
async fn main() {
    // load chain using settings name
    let glob_settings = load_settings().unwrap();

    let chain_name = "ethereum_mainnet"; //todo switch to settings

    let chain = match chain_name.clone() {
        "ethereum_mainnet" => ethereum_mainnet().await.unwrap(),
        _ => panic!("Chain not found"),
    };

    // todo pass types dynamically to methods 
    chain.subscribe_blocks::<Log, Log>();
}
