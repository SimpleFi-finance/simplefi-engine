// get dbi

// 
impl<'tx, K: TransactionKind, E: EnvironmentKind> DbTx<'tx> for Tx<'tx, K, E> {
    fn get<T: Table>(&self, key: T::Key) -> Result<Option<<T as Table>::Value>, DatabaseError> {
        self.inner
            .get(self.get_dbi::<T>()?, key.encode().as_ref())
            .map_err(|e| DatabaseError::Read(e.into()))?
            .map(decode_one::<T>)
            .transpose()
    }

    fn commit(self) -> Result<bool, DatabaseError> {
        let start = Instant::now();
        let result = self.inner.commit().map_err(|e| DatabaseError::Commit(e.into()));
        histogram!("tx.commit", start.elapsed());
        result
    }

    fn drop(self) {
        drop(self.inner)
    }

    // Iterate over read only values in database.
    fn cursor_read<T: Table>(&self) -> Result<<Self as DbTxGAT<'_>>::Cursor<T>, DatabaseError> {
        self.new_cursor()
    }

    /// Iterate over read only values in database.
    fn cursor_dup_read<T: DupSort>(
        &self,
    ) -> Result<<Self as DbTxGAT<'_>>::DupCursor<T>, DatabaseError> {
        self.new_cursor()
    }

    /// Returns number of entries in the table using cheap DB stats invocation.
    fn entries<T: Table>(&self) -> Result<usize, DatabaseError> {
        Ok(self
            .inner
            .db_stat_with_dbi(self.get_dbi::<T>()?)
            .map_err(|e| DatabaseError::Stats(e.into()))?
            .entries())
    }
}

impl<E: EnvironmentKind> DbTxMut<'_> for Tx<'_, RW, E> {
    fn put<T: Table>(&self, key: T::Key, value: T::Value) -> Result<(), DatabaseError> {
        let key = key.encode();
        self.inner
            .put(self.get_dbi::<T>()?, key.as_ref(), &value.compress(), WriteFlags::UPSERT)
            .map_err(|e| DatabaseError::Write {
                code: e.into(),
                operation: DatabaseWriteOperation::Put,
                table_name: T::NAME,
                key: Box::from(key.as_ref()),
            })
    }

    fn delete<T: Table>(
        &self,
        key: T::Key,
        value: Option<T::Value>,
    ) -> Result<bool, DatabaseError> {
        let mut data = None;

        let value = value.map(Compress::compress);
        if let Some(value) = &value {
            data = Some(value.as_ref());
        };

        self.inner
            .del(self.get_dbi::<T>()?, key.encode(), data)
            .map_err(|e| DatabaseError::Delete(e.into()))
    }

    fn clear<T: Table>(&self) -> Result<(), DatabaseError> {
        self.inner.clear_db(self.get_dbi::<T>()?).map_err(|e| DatabaseError::Delete(e.into()))?;

        Ok(())
    }

    fn cursor_write<T: Table>(
        &self,
    ) -> Result<<Self as DbTxMutGAT<'_>>::CursorMut<T>, DatabaseError> {
        self.new_cursor()
    }

    fn cursor_dup_write<T: DupSort>(
        &self,
    ) -> Result<<Self as DbTxMutGAT<'_>>::DupCursorMut<T>, DatabaseError> {
        self.new_cursor()
    }
}
