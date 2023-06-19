use chains_drivers::{chains::SupportedChains, types::chain::IndexFullBlocks};
use settings::load_settings;

#[tokio::main]
async fn main() {
    // load chain using settings name
    let glob_settings = load_settings().unwrap();

    let chain_id = "1"; //todo switch to env

    match chain_id {
        "1" => {
            let uri = glob_settings.redis_uri.clone();

            // connect to redis stream or pubsub, get blocks to index, pass blocks to chain

            // can be range of blocks or single block
            loop {
                SupportedChains::EthereumMainnet.index_full_blocks(true, 0, None);
                // receives values back, convert to mongodb documents, insert into mongodb
            }
        },
        _ => panic!("Chain not implemented to index blocks"),
    };
}
