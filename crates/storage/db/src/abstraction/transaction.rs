use interfaces::db::DatabaseError;
use rocksdb::{DBRawIteratorWithThreadMode, TransactionDB, ReadOptions};

use crate::{table::Table, common::PairResult};


pub trait DbTx: Send + Sync {
    /// Get value
    fn dae_get<T: Table>(&self, key: T::Key) -> Result<Option<T::Value>, DatabaseError>;
    /// Get latest value saved in table
    fn dae_get_last<T: Table>(&self) -> PairResult<T>;
    /// Get earlisest value saved in table
    fn dae_get_first<T: Table>(&self) -> PairResult<T>;
    /// Drops transaction
    fn dae_drop(&self);
    /// Iterate over read only values in table.
    fn dae_entries<T: Table>(&self) -> Result<usize, DatabaseError>;
    /// Put value to database
    fn dae_put<T: Table>(&self, key: T::Key, value: T::Value) -> Result<(), DatabaseError>;
    /// Delete value from database
    fn dae_delete<T: Table>(&self, key: T::Key)
        -> Result<bool, DatabaseError>;
    /// Clears database.
    fn dae_clear<T: Table>(&self) -> Result<(), DatabaseError>;
    /// Create db Cursor
    fn dae_new_cursor<T: Table>(&self, opts: ReadOptions) 
        -> Result<DBRawIteratorWithThreadMode<TransactionDB>, DatabaseError>;
}