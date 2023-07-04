use crate::types::volumetrics::Volumetric;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumetricPeriod {
    pub address: String,
    pub period_start_time: u64,
    pub period_start_block: u64,
    pub volumetrics: Vec<Volumetric>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DailyMappingItem {
    pub day: u32,
    pub volume: Volumetric,
    pub latest: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumetricPeriodDaily {
    pub address: String,
    pub year: u32,
    pub month: u32,
    pub mapping: Vec<DailyMappingItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HourlyMappingItem {
    pub day: u32,
    pub mapping: Vec<Volumetric>,
    pub latest: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumetricPeriodHourly {
    pub address: String,
    pub year: u32,
    pub month: u32,
    pub mapping: Vec<HourlyMappingItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FiveMinMappingItem {
    pub volume: Volumetric,
    pub latest: bool,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumetricPeriodFiveMin {
    pub address: String,
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub mapping: Vec<FiveMinMappingItem>,
}
