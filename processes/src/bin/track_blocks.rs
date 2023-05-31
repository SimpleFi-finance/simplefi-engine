use chains_drivers::{
    ethereum::mainnet::ethereum_mainnet, common::{base_chain::{SubscribeBlocks}, types::evm::transaction::Tx},
};

use settings::load_settings;

#[tokio::main]
async fn main() {
    // load chain using settings name
    let glob_settings = load_settings().unwrap();

    let chain_id = "1"; //todo switch to settings

    match chain_id {
        "1" => {
            let chain = ethereum_mainnet().await.unwrap();
            chain.subscribe_blocks::<Tx>(glob_settings.redis_uri);
        },
        _ => panic!("Chain not implemented to subscribe to blocks"),
    };
}
