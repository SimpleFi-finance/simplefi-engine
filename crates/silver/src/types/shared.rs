use serde::{Deserialize, Serialize};

use crate::utils::date::{round_down_timestamp, round_timestamp};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Timeframe {
    Daily,
    Hourly,
    FiveMinute,
}

impl Timeframe {
    pub fn is_hourly(&self) -> bool {
        match self {
            Timeframe::Hourly => true,
            _ => false,
        }
    }
    pub fn is_daily(&self) -> bool {
        match self {
            Timeframe::Daily => true,
            _ => false,
        }
    }
    pub fn is_five_minute(&self) -> bool {
        match self {
            Timeframe::FiveMinute => true,
            _ => false,
        }
    }

    pub fn timeframe_in_text(&self) -> String {
        match self {
            Timeframe::FiveMinute => String::from("five_minutes"),
            Timeframe::Hourly => String::from("hourly"),
            Timeframe::Daily => String::from("daily"),
            _ => panic!(""),
        }
    }

    pub fn round_timestamp(
        &self,
        ts: &u64,
    ) -> u64 {
        match self {
            Timeframe::FiveMinute => round_timestamp(5, ts),
            Timeframe::Hourly => round_timestamp(60, ts),
            Timeframe::Daily => round_timestamp(1440, ts),
            _ => panic!(""),
        }
    }
    pub fn round_down_timestamp(
        &self,
        ts: &u64,
    ) -> u64 {
        match self {
            Timeframe::FiveMinute => round_down_timestamp(5, ts),
            Timeframe::Hourly => round_down_timestamp(60, ts),
            Timeframe::Daily => round_down_timestamp(1440, ts),
            _ => panic!(""),
        }
    }
}
