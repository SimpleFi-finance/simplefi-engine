use chains_types::{SupportedChains, common::chain::SubscribeBlocks};
use simp_settings::load_settings;

#[tokio::main]
async fn main() {
    // load chain using settings name
    let glob_settings = load_settings().unwrap();

    let chain_id = "1"; //TODO: switch to env

    match chain_id {
        "1" => {
            let uri = "".to_string().clone();
            SupportedChains::EthereumMainnet.subscribe_blocks(uri).await.unwrap();
            // TODO: in future convert to stream returned from subscribe_blocks and save new block to db
        },
        _ => panic!("Chain not implemented to subscribe to blocks"),
    };
}
