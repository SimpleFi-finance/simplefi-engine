use interfaces::db::DatabaseError;
use rocksdb::{DBRawIteratorWithThreadMode, TransactionDB, ReadOptions};

use crate::{table::Table, common::PairResult};


pub trait DbTx {
    /// Get value
    fn get<T: Table>(&self, key: T::Key) -> Result<Option<T::Value>, DatabaseError>;
    /// Get latest value saved in table
    fn get_last<T: Table>(&self) -> PairResult<T>;
    /// Get earlisest value saved in table
    fn get_first<T: Table>(&self) -> PairResult<T>;
    /// Drops transaction
    fn drop(&self);
    /// Iterate over read only values in table.
    fn entries<T: Table>(&self) -> Result<usize, DatabaseError>;
    /// Put value to database
    fn put<T: Table>(&self, key: T::Key, value: T::Value) -> Result<(), DatabaseError>;
    /// Delete value from database
    fn delete<T: Table>(&self, key: T::Key)
        -> Result<bool, DatabaseError>;
    /// Clears database.
    fn clear<T: Table>(&self) -> Result<(), DatabaseError>;
    /// Create db Cursor
    fn new_cursor<T: Table>(&self, opts: ReadOptions) 
        -> Result<DBRawIteratorWithThreadMode<TransactionDB>, DatabaseError>;
}


// // pub trait DbCursor {
//         /// Positions the cursor at the first entry in the table, returning it.
//         fn first<T: Table>(&mut self, iter: &mut DBRawIterator) -> PairResult<T>;

//         /// Seeks to the KV pair exactly at `key`.
//         fn seek_exact<T: Table>(&mut self, key: T::Key, iter: &mut DBRawIterator) -> PairResult<T>;
    
//         /// Seeks to the KV pair whose key is greater than or equal to `key`.
//         fn seek<T: Table>(&mut self, key: T::Key, iter: &mut DBRawIterator) -> PairResult<T>;
    
//         /// Position the cursor at the next KV pair, returning it.
//         #[allow(clippy::should_implement_trait)]
//         fn next<T: Table>(&mut self, iter: &mut DBRawIterator) -> PairResult<T>;
    
//         /// Position the cursor at the previous KV pair, returning it.
//         fn prev<T: Table>(&mut self, iter: &mut DBRawIterator) -> PairResult<T>;
    
//         /// Positions the cursor at the last entry in the table, returning it.
//         fn last<T: Table>(&mut self, iter: &mut DBRawIterator) -> PairResult<T>;
    
//         /// Get the KV pair at the cursor's current position.
//         fn current<T: Table>(&mut self, iter: &mut DBRawIterator) -> PairResult<T>;
    
//         // /// Get an iterator that walks through the table.
//         // ///
//         // /// If `start_key` is `None`, then the walker will start from the first entry of the table,
//         // /// otherwise it starts at the entry greater than or equal to the provided key.
//         // fn walk<T: Table>(
//         //     &mut self,
//         //     start_key: Option<T::Key>,
//         // ) -> Result<Walker<'cursor, 'tx, T, Self>, DatabaseError>
//         // where
//         //     Self: Sized;
    
//         // /// Get an iterator that walks over a range of keys in the table.
//         // fn walk_range<'cursor>(
//         //     &'cursor mut self,
//         //     range: impl RangeBounds<T::Key>,
//         // ) -> Result<RangeWalker<'cursor, 'tx, T, Self>, DatabaseError>
//         // where
//         //     Self: Sized;
    
//         // /// Get an iterator that walks through the table in reverse order.
//         // ///
//         // /// If `start_key` is `None`, then the walker will start from the last entry of the table,
//         // /// otherwise it starts at the entry greater than or equal to the provided key.
//         // fn walk_back<'cursor>(
//         //     &'cursor mut self,
//         //     start_key: Option<T::Key>,
//         // ) -> Result<ReverseWalker<'cursor, 'tx, T, Self>, DatabaseError>
//         // where
//         //     Self: Sized;
// }