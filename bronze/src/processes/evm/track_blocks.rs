use chains_drivers::{chains::SupportedChains, types::chain::SubscribeBlocks};
use settings::load_settings;

#[tokio::main]
async fn main() {
    // load chain using settings name
    let glob_settings = load_settings().unwrap();

    let chain_id = "1"; //todo switch to env

    match chain_id {
        "1" => {
            let uri = glob_settings.redis_uri.clone();
            SupportedChains::EthereumMainnet.subscribe_blocks(uri).await;
            // todo in future convert to stream returned from subscribe_blocks and save new block to db
        },
        _ => panic!("Chain not implemented to subscribe to blocks"),
    };
}
