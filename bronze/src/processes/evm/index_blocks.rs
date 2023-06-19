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

            // connect to redis pub sub, get blocks to index, pass blocks to chain
            SupportedChains::EthereumMainnet.index_blocks(uri);
        },
        _ => panic!("Chain not implemented to index blocks"),
    };
}
