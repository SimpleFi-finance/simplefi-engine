use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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
}
