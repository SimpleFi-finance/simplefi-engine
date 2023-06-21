// use settings::load_settings;
// use crate::{chains::{Chain, NativeCurrency, ConnectionType}, providers::SupportedProviders};


// pub fn get_data() -> Chain {
//     let settings = load_settings().unwrap_or_default();

//     let native_currency = NativeCurrency {
//         name: "Matic".to_string(),
//         symbol: "MATIC".to_string(),
//         decimals: 18,
//         address: "0x0000000000000000000000000000000000001010".to_string(),
//     };

//     let infura_token = settings.infura_token.as_str();
//     let infura_https = format!("https://polygon-mainnet.infura.io/v3/{}", &infura_token);

//     let nodes = vec![
//         (SupportedProviders::Infura, ConnectionType::RPC, infura_https.as_str()),
//         (SupportedProviders::Local, ConnectionType::RPC, "http://localhost:8545"),
//         (SupportedProviders::Local, ConnectionType::WSS, "http://localhost:8545"),
//     ];

//     let chain = Chain::new(
//         "Polygon Mainnet",
//         "MATIC",
//         137,
//         "mainnet",
//         "https://polygonscan.com/",
//         native_currency,
//         nodes
//     );

//     chain
// }
