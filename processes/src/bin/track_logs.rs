use chains_drivers::{
    ethereum::mainnet::ethereum_mainnet, common::{base_chain::SubscribeLogs, types::evm::log::Log},
};

use settings::load_settings as glob_settings;
use processes::settings::load_settings;
#[tokio::main]
async fn main() {
    let glob_settings = glob_settings().unwrap();
    let local_settings = load_settings().unwrap();

    let chain_id = &local_settings.chain_id;

    match chain_id.as_str() {
        "1" => {
            let chain = ethereum_mainnet().await.unwrap();

            chain.subscribe_logs::<Log>();
        },
        _ => panic!("Chain not implemented to subscribe to logs"),
    };
}
