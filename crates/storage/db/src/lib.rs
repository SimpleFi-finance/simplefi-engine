mod utils;
use eyre::Context;
pub use utils::is_database_empty;
use std::{
    path::Path,
    fs::create_dir_all
};

pub mod version;
pub mod tables;
pub mod implementation;
/// Traits defining the database abstractions, such as cursors and transactions.
pub mod abstraction;
pub use abstraction::*;
use rocksdb::{Options, TransactionDB, TransactionDBOptions, SingleThreaded, MultiThreaded };

use crate::implementation::create_tables;

pub fn get_all_cfs<P: AsRef<Path>>(path: P) -> eyre::Result<Vec<String>> {

    let opts: Options = Options::default();

    let cfs = TransactionDB::<SingleThreaded>::list_cf(&opts, &path);
    #[allow(unused_variables)]
    let cfs = match cfs {
        Ok(cfs) => cfs,
        Err(err) => {
            vec![]
        }
    };
    Ok(cfs)
}

pub fn init_db<P: AsRef<Path>>(path: P) -> eyre::Result<TransactionDB::<MultiThreaded>> {
    use crate::version::{check_db_version_file, create_db_version_file, DatabaseVersionError};

    let path = path.as_ref();
    if is_database_empty(path) {
        create_dir_all(path)
            .wrap_err_with(|| format!("Could not create database directory {}", path.display()))?;
        create_db_version_file(path)?;
    } else {
        match check_db_version_file(path) {
            Ok(_) => (),
            Err(DatabaseVersionError::MissingFile) => create_db_version_file(path)?,
            Err(err) => return Err(err.into()),
        }
    }

    let mut opts: Options = Options::default();
    opts.create_if_missing(true);
    opts.create_missing_column_families(true);

    let tx_opts = TransactionDBOptions::default();

    let cfs = get_all_cfs(&path).unwrap();

    if cfs.len() > 0 {
        let db = TransactionDB::<MultiThreaded>::open_cf(&opts, &tx_opts, path, &cfs)?;

        Ok(db)
    } else {
        let mut db = TransactionDB::<MultiThreaded>::open_cf(&opts, &tx_opts, path, &cfs)?;
        create_tables(&mut db).unwrap();

        Ok(db)
    }
}

/// Opens up an existing database. Read/Write mode.

pub fn open_db<P: AsRef<Path>>(path: P) -> eyre::Result<TransactionDB::<MultiThreaded>> {
    let path = path.as_ref();
    let mut opts: Options = Options::default();
    opts.create_if_missing(true);
    opts.create_missing_column_families(true);

    let tx_opts = TransactionDBOptions::default();

    let cfs = get_all_cfs(&path).unwrap();

    TransactionDB::<MultiThreaded>::open_cf(&opts, &tx_opts, &path, &cfs)
        .with_context(|| format!("Could not open database at path: {}", path.display()))
}


// TODO: Once supported, create ReadOnly DB methods

pub mod test_utils {
    use super::*;

    /// Error during database open
    pub const ERROR_DB_OPEN: &str = "Not able to open the database file.";
    /// Error during database creation
    pub const ERROR_DB_CREATION: &str = "Not able to create the database file.";
    /// Error during table creation
    pub const ERROR_TABLE_CREATION: &str = "Not able to create tables in the database.";
    /// Error during tempdir creation
    pub const ERROR_TEMPDIR: &str = "Not able to create a temporary directory.";

    /// Create read/write database for testing
    pub fn create_test_rw_db() -> TransactionDB {
        init_db(tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path())
            .expect(ERROR_DB_CREATION)
    }

    /// Create read/write database for testing
    pub fn create_test_rw_db_with_path<P: AsRef<Path>>(path: P) -> TransactionDB {
        init_db(path.as_ref()).expect(ERROR_DB_CREATION)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        init_db,
        version::db_version_file_path, get_all_cfs,
    };
    use tempfile::tempdir;

    #[test]
    fn db_version() {
        let path = tempdir().unwrap();

        {
            let db = Arc::new(init_db(&path));
            assert!(db.is_ok());
        }

        {
            let cfs = get_all_cfs(&path).unwrap();

            assert_ne!(cfs.len(), 0);
        }

        // Database is not empty, version file is malformed
        {
            std::fs::write(path.path().join(db_version_file_path(&path)), "invalid-version")
                .unwrap();
            let db = init_db(&path);
            assert!(db.is_err());
        }

        // Database is not empty, version file contains not matching version
        {
            std::fs::write(path.path().join(db_version_file_path(&path)), "0").unwrap();
            let db = init_db(&path);
            assert!(db.is_err());
        }

    }
}