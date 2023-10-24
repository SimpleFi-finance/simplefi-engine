
use interfaces::db::DatabaseError;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    marker::{Send, Sync}
};

/// Trait that will transform the data to be saved in the DB in a (ideally) compressed format
pub trait Compress: Send + Sync + Sized + Debug {
    /// Compressed type.
    type Compressed: bytes::BufMut + AsMut<[u8]> + Default + AsRef<[u8]> + Send + Sync;

    /// If the type cannot be compressed, return its inner reference as `Some(self.as_ref())`
    fn uncompressable_ref(&self) -> Option<&[u8]> {
        None
    }

    /// Compresses data going into the database.
    fn compress(self) -> Self::Compressed {
        let mut buf = Self::Compressed::default();
        self.compress_to_buf(&mut buf);
        buf
    }

    /// Compresses data to a given buffer.
    fn compress_to_buf<B: bytes::BufMut + AsMut<[u8]>>(self, buf: &mut B);
}

/// Trait that will transform the data to be read from the DB.
pub trait Decompress: Send + Sync + Sized + Debug {
    /// Decompresses data coming from the database.
    fn decompress<B: AsRef<[u8]>>(value: B) -> Result<Self, DatabaseError>;
}

/// Trait that will transform the data to be saved in the DB.
pub trait Encode: Send + Sync + Sized + Debug {
    /// Encoded type.
    type Encoded: AsRef<[u8]> + Send + Sync;

    /// Encodes data going into the database.
    fn encode(self) -> Self::Encoded;
}

/// Trait that will transform the data to be read from the DB.
pub trait Decode: Send + Sync + Sized + Debug {
    /// Decodes data coming from the database.
    fn decode<B: AsRef<[u8]>>(value: B) -> Result<Self, DatabaseError>;
}

/// Generic trait that enforces the database key to implement [`Encode`] and [`Decode`].
pub trait Key: Encode + Decode + Ord + Clone + Serialize + Debug + for<'a> Deserialize<'a> {}

impl<T> Key for T where T: Encode + Decode + Ord + Clone + Serialize + Debug + for<'a> Deserialize<'a> {}

/// Generic trait that enforces the database value to implement [`Compress`] and [`Decompress`].
pub trait Value: Compress + Decompress + Serialize + Debug + Clone {}

impl<T> Value for T where T: Compress + Decompress + Serialize + Debug + Clone {}

pub trait Table: Send + Sync + Debug + 'static {
    /// Return table name as it is present inside the RocksDB.
    const NAME: &'static str;
    /// Key element of `Table`.
    ///
    /// Sorting should be taken into account when encoding this.
    type Key: Key;
    /// Value element of `Table`.
    type Value: Value;
}

/// Tuple with `T::Key` and `T::Value`.
pub type TableRow<T> = (<T as Table>::Key, <T as Table>::Value);

pub trait ShardedTable: Send + Sync + Debug + 'static {
        /// Return table name as it is present inside the RocksDB.
        const NAME: &'static str;
        /// Key element of `Table`.
        /// Sorting should be taken into account when encoding this.
        type Key: Key;
        /// Value element of `Table`.
        type Value: Value;
        
        const SHARDED_KEY: &'static u64;
}


/// Tuple with `T::Key` and `T::Value`.
pub type ShardedTableRow<T> = (<T as ShardedTable>::Key, <T as ShardedTable>::Value);