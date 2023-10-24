use interfaces::db::DatabaseError;

use crate::{table::Table, common::PairResult};

pub trait DbCursor<'tx, T: Table> {
    /// Positions the cursor at the first entry in the table, returning it.
    fn first(&mut self) -> PairResult<T>;

    /// Seeks to the KV pair exactly at `key`.
    fn seek_exact(&mut self, key: T::Key) -> PairResult<T>;

    /// Seeks to the KV pair whose key is greater than or equal to `key`.
    fn seek(&mut self, key: T::Key) -> PairResult<T>;

    /// Position the cursor at the next KV pair, returning it.
    #[allow(clippy::should_implement_trait)]
    fn next(&mut self) -> PairResult<T>;

    /// Position the cursor at the previous KV pair, returning it.
    fn prev(&mut self) -> PairResult<T>;

    /// Positions the cursor at the last entry in the table, returning it.
    fn last(&mut self) -> PairResult<T>;

    /// Get the KV pair at the cursor's current position.
    fn current(&mut self) -> PairResult<T>;

    /// Get an iterator that walks through the table.
    ///
    /// If `start_key` is `None`, then the walker will start from the first entry of the table,
    /// otherwise it starts at the entry greater than or equal to the provided key.
    // fn walk<'cursor>(
    //     &'cursor mut self,
    //     start_key: Option<T::Key>,
    // ) -> Result<Walker<'cursor, 'tx, T, Self>, DatabaseError>
    // where
    //     Self: Sized;

    /// Get an iterator that walks over a range of keys in the table.
    // fn walk_range<'cursor>(
    //     &'cursor mut self,
    //     range: impl RangeBounds<T::Key>,
    // ) -> Result<RangeWalker<'cursor, 'tx, T, Self>, DatabaseError>
    // where
    //     Self: Sized;

    /// Get an iterator that walks through the table in reverse order.
    ///
    /// If `start_key` is `None`, then the walker will start from the last entry of the table,
    /// otherwise it starts at the entry greater than or equal to the provided key.
    // fn walk_back<'cursor>(
    //     &'cursor mut self,
    //     start_key: Option<T::Key>,
    // ) -> Result<ReverseWalker<'cursor, 'tx, T, Self>, DatabaseError>
    // where
    //     Self: Sized;

    /// Database operation that will update an existing row if a specified value already
    /// exists in a table, and insert a new row if the specified value doesn't already exist
    fn upsert(&mut self, key: T::Key, value: T::Value) -> Result<(), DatabaseError>;

    /// Database operation that will insert a row at a given key. If the key is already
    /// present, the operation will result in an error.
    fn insert(&mut self, key: T::Key, value: T::Value) -> Result<(), DatabaseError>;

    /// Append value to next cursor item.
    ///
    /// This is efficient for pre-sorted data. If the data is not pre-sorted, use
    /// [`DbCursorRW::insert`].
    fn append(&mut self, key: T::Key, value: T::Value) -> Result<(), DatabaseError>;

    /// Delete current value that cursor points to
    fn delete_current(&mut self) -> Result<(), DatabaseError>;
}