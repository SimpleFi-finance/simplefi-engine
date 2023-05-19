use common::types::chain::{
    ConnectionType, Processor, SupportedChains, SupportedMethods, WSSLogProcessor,
};
use ethereum::{
    chain::ethereum_mainnet, utils::decode_logs::decode_logs as ethereum_mainnet_decode_callback,
    utils::subscribe_logs::subscribe_logs as ethereum_mainnet_subscribe_callback,
};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use settings::load_settings;
use std::collections::HashMap;
use tungstenite::{connect, Message};

#[tokio::main]
async fn main() {
    // load chain using settings name
    let glob_settings = load_settings().unwrap();

    let chain_name = "ethereum_mainnet"; //todo switch to settings

    let chain = match chain_name.clone() {
        "ethereum_mainnet" => ethereum_mainnet(),
        _ => panic!("Chain not found"),
    };

    let cb = match chain_name.clone() {
        "ethereum_mainnet" => ethereum_mainnet_subscribe_callback,
        _ => panic!("Chain not found"),
    };
    chain.subscribe_logs(cb).await;
}
