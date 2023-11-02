use crate::DatabaseProvider;
use crate::traits::ProtocolProvider;
use db::tables::Protocols;
use interfaces::Result;
use rocksdb::ReadOptions;
use db::tables::utils::decoder;
use db::transaction::DbTx;
// use db::table::Encode;
// use db::tables::models::VolumeKeysWithData;
use simp_primitives::{Protocol, ProtocolStatus,H256};



impl ProtocolProvider for DatabaseProvider {
    fn create_protocol(&self, factory_address:H256) -> Result<()> {
        
        // get new_id
        let mut opts = ReadOptions::default();
        opts.set_iterate_range(..);
        let mut iter = self.db.new_cursor::<Protocols>(opts).unwrap();
        iter.seek_to_last();


        // TODO: double check this works
        let new_id = if iter.valid() {
            let k = iter.key().unwrap();
            let value = iter.value().unwrap();
            let kv = decoder::<Protocols>((k.as_ref().to_vec(), value.as_ref().to_vec())).unwrap();
            kv.0 + 1
        } else {
            1 as u64
        };

        let protocol = Protocol {
            protocol_id: new_id,
            chain_id: 1,
            factory_address: factory_address,
            status: ProtocolStatus {
                last_sync_block_timestamp: 0,
                should_update: false,
                has_error: false,
            },
        };


        self.db.put::<Protocols>(new_id, protocol)?;
        Ok(())
    }

    fn delete_protocol(&self, protocol_id: u64) -> Result<()> {
        self.db.delete::<Protocols>(protocol_id)?;
        Ok(())
    }

    fn get_protocol(&self, protocol_id: u64) -> Result<Option<Protocol>> {
        let protocol = self.db.get::<Protocols>(protocol_id)?;
        Ok(protocol)
    }

    fn get_all_protocols(&self) -> Result<Vec<Protocol>> {
        let mut protocols = vec!();
        let mut opts = ReadOptions::default();
        opts.set_iterate_range(..);
        let mut iter = self.db.new_cursor::<Protocols>(opts).unwrap();

        iter.seek_to_first();

        while iter.valid() {
            let k = iter.key().unwrap();
            let value = iter.value().unwrap();
            let kv = decoder::<Protocols>((k.as_ref().to_vec(), value.as_ref().to_vec())).unwrap();
            protocols.push(kv.1);
            iter.next();
        }

        Ok(protocols)
    }

    fn get_all_synced_protocols(&self) -> Result<Vec<Protocol>> {
        let protocols = self.get_all_protocols()?;
        let synced_protocols = protocols.iter().filter(|x|x.status.should_update).copied().collect();
        Ok(synced_protocols)
    }

    fn update_protocol(&self, updated_protocol: Protocol, protocol_id: u64) -> Result<bool> {
        let matched_protocol = self.db.get::<Protocols>(protocol_id)?;

        match matched_protocol {
            Some(_) => self.db.put::<Protocols>(protocol_id,updated_protocol)?,
            _ => return Ok(false)
        }

        Ok(true)
    }
}


#[cfg(test)]
mod test {
    use crate::traits::{MarketProvider, ProtocolProvider};
    use crate::{providers::options::AccessType, DatabaseProvider};
    use db::{
        implementation::sip_rocksdb::DB, init_db, 
        test_utils::ERROR_TEMPDIR,
    };
    use simp_primitives::{H256, Market};
    use std::str::FromStr;

    fn get_provider() -> DatabaseProvider {
        let db = init_db(&tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path());

        let db = DB::new(db.unwrap());

        DatabaseProvider::new(db, AccessType::Primary)
    }


    #[test]
    fn insert_and_retrieve() {
        let provider = get_provider();
        let new_market = Market {
            protocol_id: 1,
            input_tokens: vec![H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap()],
        };
        provider.create_market(new_market, H256::zero()).expect("Expect to create market");
        let fetched_market = provider.get_market(H256::zero()).expect("Expect to retrieve market");
        assert!(fetched_market.is_some());
        match fetched_market {
            Some(m) => assert!(m.protocol_id == 1),
            _ => panic!()
        }
    }

    #[test]
    fn insert_and_retrieve_delete() {
        let provider = get_provider();

        let _ = provider.create_protocol( H256::default());
        let _ = provider.create_protocol( H256::from_str("0x0000000000000000000000000000000000000000000000000000000000000001").unwrap());
        let retrieved_protocols = provider.get_all_protocols().expect("Expect to fetch protocols");
    
        assert!(retrieved_protocols.len() == 2);

        let specific_proto = provider.get_protocol(retrieved_protocols[0].protocol_id).expect("Expect to retrieve protocol");
        match specific_proto {
            Some(p) => {
                assert!(p.factory_address == retrieved_protocols[0].factory_address);
            },
            _ => panic!("Expected to retrieve protocol")
        }

        let _ = provider.delete_protocol(retrieved_protocols[0].protocol_id);

        let specific_proto2 = provider.get_protocol(retrieved_protocols[0].protocol_id).expect("Expect to retrieve protocol");
        match specific_proto2 {
            None => {
                assert!(true);
            }
            _ => panic!("Expected to delete protocol")
        }

        let retrieved_protocols2 = provider.get_all_protocols().expect("Expect to fetch protocols");
    
        assert!(retrieved_protocols2.len() == 1);

    }

    #[test]
    fn update_and_get_synced() {
        let provider = get_provider();

        let _ = provider.create_protocol( H256::default());
        let protos = provider.get_all_protocols().expect("Expect to get protocols");
        let mut proto = protos[0];
        proto.status.should_update = true;
        
        let _ = provider.update_protocol(proto, proto.protocol_id);
        let protos2 = provider.get_all_protocols().expect("Expect to get protocols");

        assert!(protos2[0].status.should_update)
        

    }
    
    
}