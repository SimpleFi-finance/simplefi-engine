use ethereum::mainnet::ethereum_mainnet;
use shared_types::chains::evm::log::Log;
use crate::common::base_chain::SubscribeLogs;

mod common;
mod ethereum;

#[tokio::main]
async fn main() {
    let chain = ethereum_mainnet().await.unwrap();
    chain.subscribe_logs::<Log, Log>();
}
