use db::tables;
use simp_primitives::Address;
use time::OffsetDateTime;
use db::transaction::DbTx;
use crate::traits::{TrackingWriter, TrackingProvider};
use crate::DatabaseProvider;
use interfaces::Result;

impl TrackingProvider for DatabaseProvider {
    fn is_contract_tracked(&self, address: Address) -> Result<bool> {
        let tracked = self.db.get::<tables::TrackedContracts>(address)?;
        Ok(tracked.is_some())
    }
}

impl TrackingWriter for DatabaseProvider {
    fn insert_tracked_contract(&self, address: Address) -> Result<()> {
        let ts = OffsetDateTime::now_utc().microsecond();
        self.db.put::<tables::TrackedContracts>(address, ts)?;
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use simp_primitives::Address;
    use db::{init_db, test_utils::ERROR_TEMPDIR, implementation::sip_rocksdb::DB};
    use crate::{DatabaseProvider, providers::options::AccessType, traits::{TrackingWriter, TrackingProvider}};

    fn get_provider() -> DatabaseProvider {
        let db = init_db(&tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path());

        let db = DB::new(db.unwrap());

        DatabaseProvider::new(db, AccessType::Primary)
    }

    #[test]
    fn test_insert_and_retrieve_unknown_contract() {
        let address = Address::from(1);

        let provider = get_provider();

        let address_2 = Address::from(2);

        provider.insert_tracked_contract(address).unwrap();

        let tracked_contract = provider.is_contract_tracked(address).unwrap();

        assert!(tracked_contract == true);

        let untracked_contract = provider.is_contract_tracked(address_2).unwrap();

        assert!(untracked_contract == false );
    }
}