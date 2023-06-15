use chains_drivers::{
    ethereum::mainnet::ethereum_mainnet, common::base_chain::SubscribeLogs,
};

use bronze::mongo::evm::data_sets::{
    logs::Log as MongoLog, 
};

use settings::load_settings;

#[tokio::main]
async fn main() {
    // load chain using settings name
    let glob_settings = load_settings().unwrap();

    let chain_id = "1"; // todo switch to env variables

    match chain_id {
        "1" => {
            let chain = ethereum_mainnet().await.unwrap();

            chain.subscribe_logs::<MongoLog>();
        },
        _ => panic!("Chain not implemented to subscribe to logs"),
    };
}
