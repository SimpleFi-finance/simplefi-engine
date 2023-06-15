mod known_chains;

use std::{fmt, collections::HashMap};
use super::providers::SupportedProviders;

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
// pub enum SupportedChains {
//     Mainnet,
//     Polygon,
//     Arbitrum,
//     Bnb,
//     Fantom,
//     Avalanche,
// }

// impl fmt::Display for SupportedChains {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             SupportedChains::Mainnet => write!(f, "mainnet"),
//             SupportedChains::Polygon => write!(f, "polygon"),
//             SupportedChains::Arbitrum => write!(f, "arbitrum"),
//             SupportedChains::Bnb => write!(f, "bnb"),
//             SupportedChains::Fantom => write!(f, "fantom"),
//             SupportedChains::Avalanche => write!(f, "avalanche"),
//         }
//     }
// }

// #[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
// pub enum ConnectionType {
//     RPC,
//     WSS,
// }

// impl fmt::Display for ConnectionType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             ConnectionType::RPC => write!(f, "rpc"),
//             ConnectionType::WSS => write!(f, "wss"),
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct NativeCurrency {
//     pub name: String,
//     pub symbol: String,
//     pub decimals: u64,
//     pub address: String,
// }

// pub struct Chain {
//     pub name: String,
//     pub symbol: String,
//     pub chain_id: u32,
//     pub network: String,
//     pub block_explorer: String,
//     pub native_currency: NativeCurrency,
//     nodes: HashMap<(SupportedProviders, ConnectionType), String>,
// }

// impl Chain {
//     /// Creates a new [`Chain`].
//     pub fn new(name: &str, symbol: &str, chain_id: u32, network: &str, block_explorer: &str, native_currency: NativeCurrency, node_tuples: Vec<(SupportedProviders, ConnectionType, &str)>) -> Self {
//         let nodes = node_tuples.into_iter().map(|(provider, connection, url)| {
//             let provider = SupportedProviders::from(provider);
//             let connection = ConnectionType::from(connection); // SupportedChains::from_str(chain).unwrap();
//             let url = url.to_string();
//             ((provider, connection), url)
//         }).collect();

//         Chain {
//             name: name.to_string(),
//             symbol: symbol.to_string(),
//             chain_id,
//             network: network.to_string(),
//             block_explorer: block_explorer.to_string(),
//             native_currency,
//             nodes,
//         }
//     }

//     pub fn get_node(&self, provider: SupportedProviders, connection: ConnectionType) -> Option<&String> {
//         self.nodes.get(&(provider, connection))
//     }

//     // A factory method to create a Chain based on a known chain
//     pub fn from_chain(chain: SupportedChains) -> Self {
//         match chain {
//             SupportedChains::Mainnet => known_chains::mainnet::get_data(),
//             SupportedChains::Polygon => known_chains::polygon::get_data(),
//             _ => panic!("Failed to find chain data for {:?}", chain),
//             // Add more known chains here
//         }
//     }

// }

