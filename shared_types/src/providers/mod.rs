use clap::{ValueEnum};
use std::{fmt};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Hash)]
pub enum SupportedProviders {
    Infura,
    Simplefi,
    Local,
    Alchemy,
}

impl fmt::Display for SupportedProviders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SupportedProviders::Infura => write!(f, "infura"),
            SupportedProviders::Simplefi => write!(f, "simplefi"),
            SupportedProviders::Local => write!(f, "local"),
            SupportedProviders::Alchemy => write!(f, "alchemy"),
        }
    }
}



