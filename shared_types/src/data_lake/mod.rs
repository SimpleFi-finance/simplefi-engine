use std::fmt;
use clap::{ValueEnum};

use std::fs::File;

use parquet::{
    schema::{
        types::Type
    }, file::{writer::SerializedFileWriter, properties::WriterProperties}, 
};

pub trait GetSchema {
    fn get_schema() -> Type;
}

pub trait WriteDFToFile {
    fn write_to_file(&self, writer: &mut SerializedFileWriter<File>)  -> Result<(), Box<dyn std::error::Error>>;
}

pub trait FileProperties {
    fn file_properties() -> WriterProperties;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum SupportedPartitionIntervals {
    Day,
    Week,
    Month,
}
impl fmt::Display for SupportedPartitionIntervals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SupportedPartitionIntervals::Day => write!(f, "day"),
            SupportedPartitionIntervals::Week => write!(f, "week"),
            SupportedPartitionIntervals::Month => write!(f, "month"),
        }
    }
}

impl SupportedPartitionIntervals {
    fn get_seconds(&self) -> u64 {
        match self {
            SupportedPartitionIntervals::Day => 86400,
            SupportedPartitionIntervals::Week => 604800,
            SupportedPartitionIntervals::Month => 2592000,
        }
    }
    fn get_ms(&self) -> u64 {
        match self {
            SupportedPartitionIntervals::Day => 86400000,
            SupportedPartitionIntervals::Week => 604800000,
            SupportedPartitionIntervals::Month => 2592000000,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum SupportedDataTypes {
    Blocks,
    Transactions,
    Logs,
    DecodingError
}

impl fmt::Display for SupportedDataTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SupportedDataTypes::Logs => write!(f, "logs"),
            SupportedDataTypes::Blocks => write!(f, "blocks"),
            SupportedDataTypes::Transactions => write!(f, "transactions"),
            SupportedDataTypes::DecodingError => write!(f, "decoding_error"),
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
