mod ethereum;
mod common;

use crate::ethereum::mainnet::ethereum_mainnet;
use crate::common::base_chain::{ConnectionType, SubscribeLogs, NativeCurrency, Engine};
use shared_types::chains::evm::log::Log;

#[tokio::main]
async fn main() {
    let chain = ethereum_mainnet().await.unwrap();

    chain.subscribe_logs::<Log, Log>();
}