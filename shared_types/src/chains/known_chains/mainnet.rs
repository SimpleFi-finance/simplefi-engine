// use settings::load_settings;
// use crate::{chains::{NativeCurrency, ConnectionType, Chain}, providers::SupportedProviders};


// pub fn get_data() -> Chain {
//     let settings = load_settings().unwrap_or_default();

//     let native_currency = NativeCurrency {
//         name: "Ether".to_string(),
//         symbol: "ETH".to_string(),
//         decimals: 18,
//         address: "0x0000000000000000000000000000000000000000".to_string(),
//     };

//     let infura_token = settings.infura_token.as_str();
//     let infura_https = format!("https://polygon-mainnet.infura.io/v3/{}", &infura_token);
//     let infura_wss = format!("wss://polygon-mainnet.infura.io/ws/v3/{}", &infura_token);

//     let nodes = vec![
//         (SupportedProviders::Infura, ConnectionType::RPC, infura_https.as_str()),
//         (SupportedProviders::Infura, ConnectionType::WSS, infura_wss.as_str()),
//         (SupportedProviders::Local, ConnectionType::RPC, "http://localhost:8545"),
//         (SupportedProviders::Local, ConnectionType::WSS, "http://localhost:8545"),
//     ];

//     let chain = Chain::new(
//         "Ethereum Mainnet",
//         "ETH",
//         1,
//         "mainnet",
//         "https://etherscan.io",
//         native_currency,
//         nodes
//     );

//     chain
// }
