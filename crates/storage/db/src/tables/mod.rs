use crate::abstraction::table::Table;
use std::{fmt::Display, str::FromStr};

pub mod codecs;
pub mod models;
use simp_primitives::{
    Address, BlockHash, BlockNumber, Header, StoredDecodedData, StoredLog, TransactionSigned,
    TxHash, TxNumber, VolumeKey, Volumetric, MarketAddress, Protocol, Market, H256, TokenMarkets, PeriodVolumes
};
pub mod utils;
pub use models::{
    sharded_key::ShardedKey, AbiData, BlockBodyIndices, ContractData, LogIndices, StoredContract,
    TxIndices, TxLogs,
};

use self::models::{VolumeKeysWithData, VolumeKeys};
/// Enum for the types of tables present in rocksdb.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TableType {
    /// key value table
    Table,
    /// Duplicate key value table
    DupSort,
}

/// Number of tables that should be present inside database.
pub const NUM_TABLES: usize = 32;

pub trait TableViewer<R> {
    /// type of error to return
    type Error;

    /// operate on table in generic way
    fn view<T: Table>(&self) -> Result<R, Self::Error>;
}

macro_rules! tables {
    ([$(($table:ident, $type:expr)),*]) => {
        #[derive(Debug, PartialEq, Copy, Clone)]
        /// Default tables that should be present inside database.
        pub enum Tables {
            $(
                #[doc = concat!("Represents a ", stringify!($table), " table")]
                $table,
            )*
        }

        impl Tables {
            /// Array of all tables in database
            pub const ALL: [Tables; NUM_TABLES] = [$(Tables::$table,)*];

            /// The name of the given table in database
            pub const fn name(&self) -> &str {
                match self {
                    $(Tables::$table => {
                        $table::NAME
                    },)*
                }
            }

            /// The type of the given table in database
            pub const fn table_type(&self) -> TableType {
                match self {
                    $(Tables::$table => {
                        $type
                    },)*
                }
            }

            /// Allows to operate on specific table type
            pub fn view<T, R>(&self, visitor: &T) -> Result<R, T::Error>
            where
                T: TableViewer<R>,
            {
                match self {
                    $(Tables::$table => {
                        visitor.view::<$table>()
                    },)*
                }
            }
        }

        impl Display for Tables {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name())
            }
        }

        impl FromStr for Tables {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($table::NAME => {
                        return Ok(Tables::$table)
                    },)*
                    _ => {
                        return Err("Unknown table".to_string())
                    }
                }
            }
        }
    };
}

tables!([
    (CanonicalHeaders, TableType::Table),
    (Headers, TableType::Table),
    (HeaderNumbers, TableType::Table),
    (BlockIndices, TableType::Table),
    (Transactions, TableType::Table),
    (TransactionBlock, TableType::Table),
    (TxHashNumber, TableType::Table),
    (TransactionLogs, TableType::Table),
    (BlockLogs, TableType::Table),
    (ContractLogs, TableType::Table),
    (Logs, TableType::Table),
    (DecodedLogs, TableType::Table),
    (ContractProxy, TableType::Table),
    (ContractsData, TableType::Table),
    (MarketToProxy, TableType::Table),
    (Abi, TableType::Table),
    (UnknownContracts, TableType::Table),
    (TrackedContracts, TableType::Table),
    (VolumetricsFiveMin, TableType::Table),
    (VolumetricsHour, TableType::Table),
    (VolumetricsDay, TableType::Table),
    (MarketVolumetricsIndicesFiveMin, TableType::Table),
    (MarketVolumetricsIndicesHour, TableType::Table),
    (MarketVolumetricsIndicesDay, TableType::Table),
    (TimestampVolumetricsIndicesFiveMin, TableType::Table),
    (TimestampVolumetricsIndicesHour, TableType::Table),
    (TimestampVolumetricsIndicesDay, TableType::Table),
    (Protocols, TableType::Table),
    (MarketProtocol, TableType::Table),
    (TokensMarkets, TableType::Table),
    (TempPeriodVolumesFive, TableType::Table),
    (TempPeriodVolumesHour, TableType::Table)
]);

#[macro_export]
/// Macro to declare key value table.
macro_rules! table {
    ($(#[$docs:meta])+ ( $table_name:ident ) $key:ty | $value:ty) => {
        $(#[$docs])+
        ///
        #[doc = concat!("Takes [`", stringify!($key), "`] as a key and returns [`", stringify!($value), "`]")]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $table_name;

        impl $crate::table::Table for $table_name {
            const NAME: &'static str = $table_name::const_name();
            type Key = $key;
            type Value = $value;
        }

        impl $table_name {
            #[doc=concat!("Return ", stringify!($table_name), " as it is present inside the database.")]
            pub const fn const_name() -> &'static str {
                stringify!($table_name)
            }
        }

        impl std::fmt::Display for $table_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", stringify!($table_name))
            }
        }
    };
}

#[macro_export]
/// Macro to declare duplicate key value table.
macro_rules! dupsort {
    ($(#[$docs:meta])+ ( $table_name:ident ) $key:ty | [$subkey:ty] $value:ty) => {
        table!(
            $(#[$docs])+
            ///
            #[doc = concat!("`DUPSORT` table with subkey being: [`", stringify!($subkey), "`].")]
            ( $table_name ) $key | $value
        );
        impl DupSort for $table_name {
            type SubKey = $subkey;
        }
    };
}

// Blocks tables
table!(
    /// Stores header bodies.
    ( CanonicalHeaders ) BlockNumber | BlockHash
);
table!(
    /// Stores header bodies.
    ( Headers ) BlockNumber | Header
);

table!(
    /// Stores the block number corresponding to a header.
    ( HeaderNumbers ) BlockHash | BlockNumber
);

table!(
    /// Stores the block body indices.
    ( BlockIndices ) BlockNumber | BlockBodyIndices
);

// Transactions Tables
table!(
    /// (Canonical only) Stores the transaction body for canonical transactions.
    ( Transactions ) TxNumber | TransactionSigned
);
table!(
    /// Stores the mapping of transaction number to the blocks number.
    ///
    /// The key is the highest transaction ID in the block.
    ( TransactionBlock ) TxNumber | BlockNumber
);
table!(
    /// Stores the mapping of the transaction hash to the transaction number.
    ( TxHashNumber ) TxHash | TxNumber
);

// Logs tables
table!(
    /// stores the hash of tx to its logs ids
    ( TransactionLogs ) TxNumber | TxLogs
);

table!(
    /// stores the blocknumber to its logs ids
    ( BlockLogs ) BlockNumber | TxLogs
);

table!(
    /// stores the hash of address to its logs ids
    ( ContractLogs ) ShardedKey<Address> | TxLogs
);

table!(
    /// stores the id to its logs
    ( Logs ) String | StoredLog
);

table!(
    /// stores the id to its decoded log data
    ( DecodedLogs ) String | StoredDecodedData
);

// ABI tables
table!(
    /// Stores the hash of contract to its proxy hash
    ( ContractProxy ) Address | StoredContract
);

table!(
    /// Stores the hash of contract to its proxy hash
    ( ContractsData ) Address | ContractData
);

table!(
    /// Stores the hash of contract to its proxy hash
    ( MarketToProxy ) Address | Address
);

table!(
    /// stores index to abi
    ( Abi ) u64 | AbiData
);

table!(
    /// stored the address of contracts without abi
    ( UnknownContracts ) Address | u32
);

// Tracking tables

table!(
    /// Tracks the required contracts to be decoded and timestamp started
    ( TrackedContracts ) Address | u32
);

table!(
    /// stores volumetric 5m
    ( VolumetricsFiveMin ) VolumeKey | Volumetric
);
table!(
    /// stores volumetric 1 hour
    ( VolumetricsHour ) VolumeKey | Volumetric
);
table!(
    /// stores volumetric 1 day
    ( VolumetricsDay ) VolumeKey | Volumetric
);


table!(
    /// stores marketAddress - volume  keys 5 min
    ( MarketVolumetricsIndicesFiveMin ) ShardedKey<MarketAddress> | VolumeKeysWithData
);
table!(
    /// stores marketAddress - volume  keys 1 hour
    ( MarketVolumetricsIndicesHour ) ShardedKey<MarketAddress> | VolumeKeysWithData
);
table!(
    /// stores marketAddress - volume  keys day
    ( MarketVolumetricsIndicesDay ) ShardedKey<MarketAddress> | VolumeKeysWithData
);

table!(
    /// stores timestamp - volume  keys 5 min
    ( TimestampVolumetricsIndicesFiveMin ) u64 | VolumeKeys
);
table!(
    /// stores timestamp - volume  keys 1 hour
    ( TimestampVolumetricsIndicesHour ) u64 | VolumeKeys
);
table!(
    /// stores timestamp - volume  keys day
    ( TimestampVolumetricsIndicesDay ) u64 | VolumeKeys
);

table!(
    /// Stores basic protocol and current status (syncing, error etc)
    ( Protocols ) u64 | Protocol
);

table!(
    /// Stores the protocol id that the market address belongs to.
    ( MarketProtocol ) MarketAddress | Market
);

table!(
    /// Stores each market addresses the token is present in. (TokenAddress | Vec<MarketAddress>
    ( TokensMarkets ) H256 | TokenMarkets
);

table!(
    /// Stores the five minute volumes into short term storage
        /// key = "periodTimestamp-marketAddress"
    ( TempPeriodVolumesFive ) ShardedKey<MarketAddress> | PeriodVolumes
);

table!(
    /// Stores the hourly volumes into short term storage
    /// key = "periodTimestamp-marketAddress"
    ( TempPeriodVolumesHour )  ShardedKey<MarketAddress> | PeriodVolumes
);



#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tables::{MarketProtocol, MarketVolumetricsIndicesDay, MarketVolumetricsIndicesFiveMin, MarketVolumetricsIndicesHour, Protocols, Tables, TimestampVolumetricsIndicesDay, TimestampVolumetricsIndicesFiveMin, TimestampVolumetricsIndicesHour, VolumetricsDay, VolumetricsFiveMin, VolumetricsHour};

    use super::{
        Abi, BlockIndices, BlockLogs, CanonicalHeaders, ContractLogs, ContractProxy, ContractsData,
        DecodedLogs, HeaderNumbers, Headers, Logs, MarketToProxy, TableType, TrackedContracts,
        TransactionBlock, TransactionLogs, Transactions, TxHashNumber, UnknownContracts,
        NUM_TABLES, TokensMarkets, TempPeriodVolumesFive, TempPeriodVolumesHour,
    };

    const TABLES: [(TableType, &str); NUM_TABLES] = [
        (TableType::Table, CanonicalHeaders::const_name()),
        (TableType::Table, Headers::const_name()),
        (TableType::Table, HeaderNumbers::const_name()),
        (TableType::Table, BlockIndices::const_name()),
        (TableType::Table, Transactions::const_name()),
        (TableType::Table, TransactionBlock::const_name()),
        (TableType::Table, TxHashNumber::const_name()),
        (TableType::Table, TransactionLogs::const_name()),
        (TableType::Table, BlockLogs::const_name()),
        (TableType::Table, ContractLogs::const_name()),
        (TableType::Table, Logs::const_name()),
        (TableType::Table, DecodedLogs::const_name()),
        (TableType::Table, ContractProxy::const_name()),
        (TableType::Table, ContractsData::const_name()),
        (TableType::Table, MarketToProxy::const_name()),
        (TableType::Table, Abi::const_name()),
        (TableType::Table, UnknownContracts::const_name()),
        (TableType::Table, TrackedContracts::const_name()),
        (TableType::Table, VolumetricsFiveMin::const_name()),
        (TableType::Table, VolumetricsHour::const_name()),
        (TableType::Table, VolumetricsDay::const_name()),
        (TableType::Table, MarketVolumetricsIndicesFiveMin::const_name()),
        (TableType::Table, MarketVolumetricsIndicesHour::const_name()),
        (TableType::Table, MarketVolumetricsIndicesDay::const_name()),
        (TableType::Table, TimestampVolumetricsIndicesFiveMin::const_name()),
        (TableType::Table, TimestampVolumetricsIndicesHour::const_name()),
        (TableType::Table, TimestampVolumetricsIndicesDay::const_name()),
        (TableType::Table, Protocols::const_name()),
        (TableType::Table, MarketProtocol::const_name()),
        (TableType::Table, TokensMarkets::const_name()),
        (TableType::Table, TempPeriodVolumesFive::const_name()),
        (TableType::Table, TempPeriodVolumesHour::const_name())
    ];

    #[test]
    fn parse_table_from_str() {
        for (table_index, &(table_type, table_name)) in TABLES.iter().enumerate() {
            let table = Tables::from_str(table_name).unwrap();
            assert_eq!(table as usize, table_index);
            assert_eq!(table.table_type(), table_type);
            assert_eq!(table.name(), table_name);
        }
    }
}
