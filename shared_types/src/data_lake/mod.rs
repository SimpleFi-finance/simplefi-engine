use std::fmt;
use clap::{ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum SupportedDataTypes {
    Blocks,
    Transactions,
    Logs,
    DecodingError,
    UniqueAddresses
}

impl fmt::Display for SupportedDataTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SupportedDataTypes::Logs => write!(f, "logs"),
            SupportedDataTypes::Blocks => write!(f, "blocks"),
            SupportedDataTypes::Transactions => write!(f, "transactions"),
            SupportedDataTypes::DecodingError => write!(f, "decoding_error"),
            SupportedDataTypes::UniqueAddresses => write!(f, "unique_ddresses"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum SupportedDataLevels {
    Bronze,
    Silver,
    Gold,
}

impl fmt::Display for SupportedDataLevels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SupportedDataLevels::Bronze => write!(f, "bronze"),
            SupportedDataLevels::Silver => write!(f, "silver"),
            SupportedDataLevels::Gold => write!(f, "gold"),
        }
    }
}
