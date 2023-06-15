use bronze::mongo::evm::data_sets::blocks::Block;
use chains_drivers::{
    ethereum::mainnet::ethereum_mainnet, 
    types::base::chain::SubscribeBlocks,
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
            let uri = glob_settings.redis_uri.clone();
            loop {
                let bn = chain.subscribe_blocks::<Block>(uri.clone()).unwrap();

                println!("Block number: {}", bn);
            }
        },
        _ => panic!("Chain not implemented to subscribe to blocks"),
    };
}
