use chains_drivers::{
    ethereum::mainnet::ethereum_mainnet, common::base_chain::SubscribeLogs,
};
use third_parties::mongo::lib::bronze::logs::types::Log;
use settings::load_settings;

#[tokio::main]
async fn main() {
    // load chain using settings name
    let glob_settings = load_settings().unwrap();

    let chain_id = "1"; //todo switch to settings

    let chain = match chain_id {
        "1" => ethereum_mainnet().await.unwrap(),
        _ => panic!("Chain not implemented to subscribe to logs"),
    };

    // todo pass types dynamically to methods 
    chain.subscribe_logs::<Log, Log>();
}
