use simp_settings::load_settings;

#[tokio::main]
async fn main() {
    // load chain using settings name
    let chain_id = load_settings().unwrap().chain_id.clone().to_string();
    let chain_id = chain_id.as_str();

    match chain_id {
        "1" => {
            let uri = "".to_string().clone();
            // SupportedChains::EthereumMainnet.subscribe_blocks(uri).await.unwrap();
            // TODO: in future convert to stream returned from subscribe_blocks and save new block to db
        },
        _ => panic!("Chain not implemented to subscribe to blocks"),
    };
}
