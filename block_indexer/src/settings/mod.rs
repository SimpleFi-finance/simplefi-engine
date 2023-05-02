use clap::{ Parser };
use confy::{load, store, ConfyError};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author = "SimpleFi Finance")]
#[command(version)]
#[command(about = "Chain Tracker Settings Generator")]
#[command(
    long_about = "Chain Tracker Settings Generator enhaces the running experience generating a settings files with required and default properties."
)]
#[command(next_line_help = true)]
pub struct ChainIndexerSettings {
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MyChainIndexerSettings {
}



impl MyChainIndexerSettings {
    pub fn new(
    ) -> Self {
        MyChainIndexerSettings {
        }
    }
}


pub fn load_settings() -> Result<MyChainIndexerSettings, ConfyError> {
    let default_settings = MyChainIndexerSettings {
    };

    let settings: MyChainIndexerSettings = load("simplefi_engine", Some("chain_tracker_settings")).unwrap_or(default_settings);

    Ok(settings)
}

pub fn store_settings(settings: &MyChainIndexerSettings) -> Result<(), ConfyError> {
    store("simplefi_engine", Some("chain_tracker_settings"), &settings).expect("Failed to store abi discovery settings");

    Ok(())
}
