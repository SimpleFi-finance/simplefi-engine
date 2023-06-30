use std::panic;

use crate::{
    mongo::protocol_status::{
        basic::protocol_status_db,
        getters::get_protocol_status,
        setters::{
            create_protocol_status, updated_protocol_snapshot_last_block,
            updated_protocol_snapshot_synced_status, updated_protocol_status,
            updated_protocol_volumetric_last_block, updated_protocol_volumetric_synced_status,
        },
        types::ProtocolStatus,
    },
    protocols::uniswap::uniswap_v2_mainnet,
    types::{ProtocolInfo, Row},
};
use async_trait::async_trait;
use chains_drivers::common::base_chain::SupportedChains;
use polars::prelude::DataFrame;
use shared_types::{
    data_lake::SupportedDataTypes,
    gold::{shared::Timeframe, volumetrics::Volumes},
    mongo::bronze::evm::logs::Log,
};

pub enum SupportedProtocolDrivers {
    UniswapV2Mainnet,
}

pub fn match_protocol_from_name(name: &str) -> SupportedProtocolDrivers {
    match name {
        "Uniswap_V2_mainnet" => SupportedProtocolDrivers::UniswapV2Mainnet,
        _ => panic!("Driver not supported"),
    }
}

impl SupportedProtocolDrivers {
    pub fn get_driver_name(&self) -> String {
        match self {
            SupportedProtocolDrivers::UniswapV2Mainnet => String::from("Uniswap_V2_mainnet"),
            _ => panic!("Driver not supported"),
        }
    }

    fn resolve_collection_name(
        data_type: SupportedDataTypes,
        timeframe: Timeframe,
        chain: SupportedChains,
    ) -> String {
        // TODO CHAIN
        format!(
            "{}_gold_{}_{}",
            chain.to_string(),
            data_type.to_string(),
            timeframe.timeframe_in_text()
        )
    }

    // pub async fn get_chain(&self) -> EvmChain {
    //     match self.chain {
    //         SupportedChains::Mainnet => SupportedChains::Mainnet,
    //         _ => panic!("Chain doesn't exist"),
    //     }
    // }
}
// pub trait GetProtocolInfo {
//     fn get_protocol_info(&self) -> ProtocolInfo;
// }

// impl GetProtocolInfo for SupportedProtocolDrivers {
//     fn get_protocol_info(&self) -> ProtocolInfo {
//         match self {
//             SupportedProtocolDrivers::UniswapV2Mainnet => uniswap_v2_mainnet::get_protocol_info(),
//         }
//     }
// }

/*
  Protocal status
  Interacting with the status collection,
  keeping track of each protocols current sync status + last active block
*/
#[async_trait]
trait ProtocolStatusMethods {
    async fn get_protocol_status(&self) -> Result<ProtocolStatus, Box<dyn std::error::Error>>;
    async fn create_protocol_status(&self) -> Result<ProtocolStatus, Box<dyn std::error::Error>>;
    async fn updated_protocol_status(
        &self,
        update: ProtocolStatus,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn updated_protocol_volumetric_synced_status(
        &self,
        status: bool,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn updated_protocol_snapshot_synced_status(
        &self,
        status: bool,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn updated_protocol_volumetric_last_block(
        &self,
        block: u64,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn updated_protocol_snapshot_last_block(
        &self,
        block: u64,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
impl ProtocolStatusMethods for SupportedProtocolDrivers {
    async fn get_protocol_status(&self) -> Result<ProtocolStatus, Box<dyn std::error::Error>> {
        let db = protocol_status_db().await?;
        get_protocol_status(&self.get_driver_name(), &db).await
    }

    async fn create_protocol_status(&self) -> Result<ProtocolStatus, Box<dyn std::error::Error>> {
        let db = protocol_status_db().await?;
        let info = self.get_protocol_info();
        create_protocol_status(info.name, info.factory_address, &db).await
    }

    async fn updated_protocol_status(
        &self,
        update: ProtocolStatus,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db = protocol_status_db().await?;
        let info = self.get_protocol_info();
        updated_protocol_status(info.name, update, &db).await
    }

    async fn updated_protocol_volumetric_synced_status(
        &self,
        status: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db = protocol_status_db().await?;
        let info = self.get_protocol_info();
        updated_protocol_volumetric_synced_status(info.name, status, &db).await
    }

    async fn updated_protocol_snapshot_synced_status(
        &self,
        status: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db = protocol_status_db().await?;
        let info = self.get_protocol_info();
        updated_protocol_snapshot_synced_status(info.name, status, &db).await
    }

    async fn updated_protocol_volumetric_last_block(
        &self,
        block: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db = protocol_status_db().await?;
        let info = self.get_protocol_info();
        updated_protocol_volumetric_last_block(&info.name, block, &db).await
    }

    async fn updated_protocol_snapshot_last_block(
        &self,
        block: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db = protocol_status_db().await?;
        let info = self.get_protocol_info();
        updated_protocol_snapshot_last_block(&info.name, block, &db).await
    }
}
