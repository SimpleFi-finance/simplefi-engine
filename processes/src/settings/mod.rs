use clap::{ Parser };
use confy::{load, store, ConfyError};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author = "SimpleFi Finance")]
#[command(version)]
#[command(about = "Chains Processes Settings Generator")]
#[command(
    long_about = "Chains Processes Settings Generator enhaces the running experience generating a settings files with required and default properties."
)]
#[command(next_line_help = true)]
pub struct ChainSettings {
    // MongoDB Settings
    #[arg(
        long = "chain",
        help = "chain",
        default_value = "ethereum"
    )]
    pub chain: String,
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MyChainSettings {
    pub chain: String,
}



impl MyChainSettings {
    pub fn new(
        chain: String,
    ) -> Self {
        MyChainSettings {
            chain,
        }
    }
}


pub fn load_settings() -> Result<MyChainSettings, ConfyError> {
    let default_settings = MyChainSettings {
        chain: String::from("ethereum"),
    };

    let settings: MyChainSettings = load("simplefi_engine", Some("processes_chains_settings")).unwrap_or(default_settings);

    Ok(settings)
}

pub fn store_settings(settings: &MyChainSettings) -> Result<(), ConfyError> {
    store("simplefi_engine", Some("chain_sync_to_db_settings"), &settings).expect("Failed to store abi discovery settings");

    Ok(())
}
