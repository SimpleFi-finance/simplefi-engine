use eyre::Result;
use rocksdb::{TransactionDB, MultiThreaded, Options};

use crate::tables::Tables;

pub fn create_tables(db: &mut TransactionDB::<MultiThreaded>) -> Result<()> {

    let opts = Options::default();

    for table in Tables::ALL {
        db.create_cf(table.name(), &opts).unwrap();
    }

    Ok(())
}

pub mod dae_rocksdb;