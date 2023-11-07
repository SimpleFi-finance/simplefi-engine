use std::collections::HashMap;

use crate::DatabaseProvider;
use crate::traits::{ Timeframe, VolumetricReader, VolumetricWriter};
use db::tables::models::sharded_key::NUM_OF_INDICES_IN_SHARD;
use db::tables::{VolumetricsDay, VolumetricsHour, VolumetricsFiveMin, MarketVolumetricsIndicesDay, MarketVolumetricsIndicesHour, MarketVolumetricsIndicesFiveMin, TimestampVolumetricsIndicesDay, TimestampVolumetricsIndicesHour, TimestampVolumetricsIndicesFiveMin, ShardedKey};
use interfaces::Result;
use rocksdb::ReadOptions;
use db::tables::utils::decoder;
use db::transaction::DbTx;
use db::tables::models::{VolumeKeysWithData, VolumeKeyWithData, VolumeKeys};
use simp_primitives::{H256, MarketAddress, VolumeKey, Volumetric};



impl VolumetricReader for DatabaseProvider {

    fn get_volume_helper(&self, key: VolumeKey, timeframe: Timeframe) -> Result<Option<Volumetric>> {
        let volume = match timeframe {
            Timeframe::Daily => self.db.dae_get::<VolumetricsDay>(key),
            Timeframe::Hourly => self.db.dae_get::<VolumetricsHour>(key),
            Timeframe::FiveMinute => self.db.dae_get::<VolumetricsFiveMin>(key),
        }?;

        Ok(volume)

    }


    fn get_market_range(&self, market_address: H256, timeframe: crate::traits::volumetric::Timeframe, from: Option<u64>, to: Option<u64>) -> Result<Vec<Volumetric>> {
        let mut volumes = Vec::new();
        let mut options = rocksdb::ReadOptions::default();
        options.set_iterate_range(rocksdb::PrefixRange(market_address.as_bytes()));

        let mut iter = match timeframe {
            Timeframe::Daily =>  self.db.dae_new_cursor::<MarketVolumetricsIndicesDay>(options),
            Timeframe::Hourly =>self.db.dae_new_cursor::<MarketVolumetricsIndicesHour>(options),
            Timeframe::FiveMinute => self.db.dae_new_cursor::<MarketVolumetricsIndicesFiveMin>(options),
        }?;

        iter.seek_to_first();

        while iter.valid() {
            let k = iter.key().unwrap();
            let value = iter.value().unwrap();

            let location_keys = match timeframe {
                Timeframe::Daily => decoder::<MarketVolumetricsIndicesDay>((k.to_vec(), value.to_vec())).unwrap(),
                Timeframe::Hourly => decoder::<MarketVolumetricsIndicesHour>((k.to_vec(), value.to_vec())).unwrap(),
                Timeframe::FiveMinute => decoder::<MarketVolumetricsIndicesFiveMin>((k.to_vec(), value.to_vec())).unwrap(),
            };
            let final_volume = &location_keys.1.volume_keys[location_keys.1.volume_keys.len() - 1].clone();

            match from {
                Some(f) => {
                    if final_volume.timestamp < f {
                        continue
                    }
                },
                _ => ()
            }

            for key in location_keys.1.volume_keys {
                match (from,to) {
                    (Some(f),Some(t)) => {
                        if key.timestamp >= f  && key.timestamp <= t{
                            let volume = self.get_volume_helper(key.key, timeframe)?;
                            match volume {
                                Some(v) => volumes.push(v),
                                None => ()
                            }
                        }
                    },
                    (Some(f), None) => {
                        if key.timestamp >= f {
                            let volume = self.get_volume_helper(key.key, timeframe)?;
                            match volume {
                                Some(v) => volumes.push(v),
                                None => ()
                            }
                        }
                    },
                    _ => {
                        let volume = self.get_volume_helper(key.key, timeframe)?;
                        match volume {
                            Some(v) => volumes.push(v),
                            None => ()
                        }
                    }
                }

               
            }
            match to {
                Some(t) => {
                    if final_volume.timestamp > t {
                        break
                    }
                },
                _ => ()
            }
            iter.next();
        }

        Ok(volumes)

    }



    fn get_by_timestamp(&self, timeframe: crate::traits::volumetric::Timeframe, timestamp: u64) -> Result<Vec<Volumetric>> {

        let keys = match timeframe {
            Timeframe::Daily => self.db.dae_get::<TimestampVolumetricsIndicesDay>(timestamp),
            Timeframe::Hourly => self.db.dae_get::<TimestampVolumetricsIndicesHour>(timestamp),
            Timeframe::FiveMinute => self.db.dae_get::<TimestampVolumetricsIndicesFiveMin>(timestamp),
        }?;

        let mut volumes_response = Vec::new();

        let _ = match keys {
            None => (),
            Some(volume_keys) => {
                for v_key in volume_keys.volume_keys{
                    let volume = self.get_volume_helper(v_key, timeframe)?;

                    let _ = match volume {
                        Some(matched_volume) => volumes_response.push(matched_volume),
                        None => (),
                    };
                }
            }
        };

        Ok(volumes_response)
    }

    fn get_latest_market_volume (&self, market_address: H256,timeframe:Timeframe) -> Result<Option<Volumetric>> {
        let mut options = rocksdb::ReadOptions::default();
        options.set_iterate_range(rocksdb::PrefixRange(market_address.as_bytes()));

        let mut iter = match timeframe {
            Timeframe::Daily =>  self.db.dae_new_cursor::<MarketVolumetricsIndicesDay>(options),
            Timeframe::Hourly =>self.db.dae_new_cursor::<MarketVolumetricsIndicesHour>(options),
            Timeframe::FiveMinute => self.db.dae_new_cursor::<MarketVolumetricsIndicesFiveMin>(options),
        }?;

        iter.seek_to_last();

        // market not found
        if iter.valid() != true {
            ()
        }

        let k = iter.key().unwrap();
        let value = iter.value().unwrap();

        let kv = match timeframe {
            Timeframe::Daily => decoder::<MarketVolumetricsIndicesDay>((k.to_vec(), value.to_vec())).unwrap(),
            Timeframe::Hourly => decoder::<MarketVolumetricsIndicesHour>((k.to_vec(), value.to_vec())).unwrap(),
            Timeframe::FiveMinute => decoder::<MarketVolumetricsIndicesFiveMin>((k.to_vec(), value.to_vec())).unwrap(),
        };


        let key = kv.1.volume_keys.last();

        let volume = match key {
            Some(volume_key) => self.get_volume_helper(volume_key.key.clone(), timeframe)?,
            None => None
        };

        Ok(volume)
    }



    fn get_volume_by_key (&self, key: u64, timeframe:Timeframe) -> Result<Option<Volumetric>> {
        let v = self.get_volume_helper(key,timeframe)?;
        Ok(v)
    }

    fn get_market_volume_keys (&self, market_address: H256, timeframe: Timeframe) -> Result<Option<Vec<VolumeKeyWithData>>> {
        let mut keys : Vec<VolumeKeyWithData> = vec![];
        let mut options = rocksdb::ReadOptions::default();
        options.set_iterate_range(rocksdb::PrefixRange(market_address.as_bytes()));
        
        let mut iter = match timeframe {
            Timeframe::Daily =>  self.db.dae_new_cursor::<MarketVolumetricsIndicesDay>(options),
            Timeframe::Hourly =>self.db.dae_new_cursor::<MarketVolumetricsIndicesHour>(options),
            Timeframe::FiveMinute => self.db.dae_new_cursor::<MarketVolumetricsIndicesFiveMin>(options),
        }?;

        iter.seek_to_first();
        

        while iter.valid() {
            let k = iter.key().unwrap();
            let value = iter.value().unwrap();
            let mut kv: (ShardedKey<H256>, VolumeKeysWithData) = match timeframe {
                Timeframe::Daily => decoder::<MarketVolumetricsIndicesDay>((k.to_vec(), value.to_vec())).unwrap(),
                Timeframe::Hourly => decoder::<MarketVolumetricsIndicesHour>((k.to_vec(), value.to_vec())).unwrap(),
                Timeframe::FiveMinute => decoder::<MarketVolumetricsIndicesFiveMin>((k.to_vec(), value.to_vec())).unwrap(),
            };

            keys.append(&mut kv.1.volume_keys);
            iter.next();
        
        };


        Ok(Some(keys))
    }

  

    
}

impl VolumetricWriter for DatabaseProvider {

    fn get_last_volume_id_or_default(&self, timeframe:Timeframe) -> Result<u64> {
        let mut opts = ReadOptions::default();
        opts.set_iterate_range(..);


        let mut iter = match timeframe {
            Timeframe::Daily => self.db.dae_new_cursor::<VolumetricsDay>(opts),
            Timeframe::Hourly => self.db.dae_new_cursor::<VolumetricsHour>(opts),
            Timeframe::FiveMinute => self.db.dae_new_cursor::<VolumetricsFiveMin>(opts),
        }?;

        iter.seek_to_last();
        let new_key = if iter.valid() {
            let k = iter.key().unwrap();
            let value = iter.value().unwrap();
    
            let kv = match timeframe {
                Timeframe::Daily => decoder::<VolumetricsDay>((k.to_vec(), value.to_vec())).unwrap(),
                Timeframe::Hourly => decoder::<VolumetricsHour>((k.to_vec(), value.to_vec())).unwrap(),
                Timeframe::FiveMinute => decoder::<VolumetricsFiveMin>((k.to_vec(), value.to_vec())).unwrap(),
            };
            kv.0 + 1
        } else {
            1
        };
        Ok(new_key)
    }

    fn set_volume_helper (&self, key: VolumeKey, volume: Volumetric, timeframe: Timeframe) -> Result<()> {
        let _ = match timeframe {
            Timeframe::Daily => self.db.dae_put::<VolumetricsDay>(key, volume),
            Timeframe::Hourly => self.db.dae_put::<VolumetricsHour>(key, volume),
            Timeframe::FiveMinute => self.db.dae_put::<VolumetricsFiveMin>(key, volume),
        }?;
        Ok(())
    }

    fn set_market_index_helper(&self, key: ShardedKey<MarketAddress>, value: VolumeKeysWithData, timeframe: Timeframe) -> Result<()> {
        let _ = match timeframe {
            Timeframe::Daily =>  self.db.dae_put::<MarketVolumetricsIndicesDay>(key, value),
            Timeframe::Hourly =>self.db.dae_put::<MarketVolumetricsIndicesHour>(key, value),
            Timeframe::FiveMinute => self.db.dae_put::<MarketVolumetricsIndicesFiveMin>(key,value),
        }?;

        Ok(())
    }


    fn add_volume(&self, volume: Volumetric, timeframe: Timeframe, key: u64, add_indices: bool) -> Result<u64> {

        let volume_ts = volume.timestamp.clone();
        let market_address = volume.market_address.clone();
        let _ = self.set_volume_helper(key,volume,timeframe)?;


        if add_indices {
            self.add_indices((&market_address, &volume_ts, &key), timeframe)?;
        }

        Ok(key)
    }

    fn add_market_volumes(&self, volumes: Vec<Volumetric>, timeframe: Timeframe) -> Result<()> {
        let mut key = self.get_last_volume_id_or_default(timeframe)?;
        let mut timestamp_hash : HashMap<u64, Vec<u64>> = HashMap::new();
        let mut market_hash: HashMap<H256, Vec<VolumeKeyWithData>> = HashMap::new();
        for volume in volumes {
            let v_ts = volume.timestamp.clone();
            let v_market_address = volume.market_address.clone();
            self.add_volume(volume, timeframe, key, false)?;
            timestamp_hash.entry(v_ts).or_insert(vec![]).push(key);
            market_hash.entry(v_market_address).or_insert(vec![]).push(VolumeKeyWithData { timestamp:v_ts, key });
            key = key + 1
        }


        for (key,value) in market_hash.iter() {
           let _ = self.bulk_volume_market_indices(key, value.clone(), timeframe)?;
        }
        
        for (key,value) in timestamp_hash.iter() {
            let _ = self.bulk_volume_timestamp_indices(key.clone(), value.clone(), timeframe)?;
        }

       
        Ok(())
    }

    fn add_ts_index(&self, volume_data:(&u64,&u64), timeframe:Timeframe) -> Result<()> {
        let volume_ts = volume_data.0;

        // timestamp index table
        let _ = match timeframe {
            Timeframe::Daily => {
                let existing = self.db.dae_get::<TimestampVolumetricsIndicesDay>(volume_ts.clone())?;
                match existing {
                    Some(mut matched_existing) => {
                        matched_existing.volume_keys.push(volume_data.1.clone());
                        self.db.dae_put::<TimestampVolumetricsIndicesDay>(volume_ts.clone(), matched_existing)?;
                    },
                    None => {
                        let new_entry = VolumeKeys {
                            volume_keys: vec![*volume_data.1]
                        };
                        self.db.dae_put::<TimestampVolumetricsIndicesDay>(volume_ts.clone(), new_entry)?;
                    }
                }
            },
            Timeframe::Hourly => {
                let existing = self.db.dae_get::<TimestampVolumetricsIndicesHour>(volume_ts.clone())?;
                match existing {
                    Some(mut matched_existing) => {
                        matched_existing.volume_keys.push(volume_data.1.clone());
                        self.db.dae_put::<TimestampVolumetricsIndicesFiveMin>(volume_ts.clone(), matched_existing)?;
                    },
                    None => {
                        let new_entry = VolumeKeys {
                            volume_keys: vec![*volume_data.1]
                        };
                        self.db.dae_put::<TimestampVolumetricsIndicesHour>(volume_ts.clone(), new_entry)?;
                    }
                }
            },
            Timeframe::FiveMinute => {
                let existing = self.db.dae_get::<TimestampVolumetricsIndicesFiveMin>(volume_ts.clone())?;
                match existing {
                    Some(mut matched_existing) => {
                        matched_existing.volume_keys.push(volume_data.1.clone());
                        self.db.dae_put::<TimestampVolumetricsIndicesFiveMin>(volume_ts.clone(), matched_existing)?;
                    },
                    None => {
                        let new_entry = VolumeKeys {
                            volume_keys: vec![*volume_data.1]
                        };
                        self.db.dae_put::<TimestampVolumetricsIndicesFiveMin>(volume_ts.clone(), new_entry)?;
                    }
                }
            },
        };

        Ok(())
    }

    fn add_market_index(&self, volume_data:(&H256,&u64,&u64), timeframe:Timeframe) -> Result<()> {

        // market address index table
        let mut options = rocksdb::ReadOptions::default();
        options.set_iterate_range(rocksdb::PrefixRange(volume_data.0.as_bytes()));

        let mut iter = match timeframe {
            Timeframe::Daily =>  self.db.dae_new_cursor::<MarketVolumetricsIndicesDay>(options),
            Timeframe::Hourly =>self.db.dae_new_cursor::<MarketVolumetricsIndicesHour>(options),
            Timeframe::FiveMinute => self.db.dae_new_cursor::<MarketVolumetricsIndicesFiveMin>(options),
        }?;


        iter.seek_to_last();
        while iter.valid() {
            let k = iter.key().unwrap();
            let value = iter.value().unwrap();

            let kv = match timeframe {
                Timeframe::Daily => decoder::<MarketVolumetricsIndicesDay>((k.to_vec(), value.to_vec())).unwrap(),
                Timeframe::Hourly => decoder::<MarketVolumetricsIndicesHour>((k.to_vec(), value.to_vec())).unwrap(),
                Timeframe::FiveMinute => decoder::<MarketVolumetricsIndicesFiveMin>((k.to_vec(), value.to_vec())).unwrap(),
            };


            let key = kv.0;
            let value = kv.1;

            if value.volume_keys.len() < NUM_OF_INDICES_IN_SHARD {
                let mut new_key = key.clone();
                let mut new_value = value.clone();
                new_value.volume_keys.push(VolumeKeyWithData { timestamp: volume_data.1.clone(), key : volume_data.2.clone() as VolumeKey });
                if volume_data.1.clone() > new_key.max_shard_value {
                    new_key.max_shard_value = volume_data.1.clone()
                }

                match timeframe {
                    Timeframe::Daily => self.db.dae_delete::<MarketVolumetricsIndicesDay>(key).unwrap(),
                    Timeframe::Hourly => self.db.dae_delete::<MarketVolumetricsIndicesHour>(key).unwrap(),
                    Timeframe::FiveMinute => self.db.dae_delete::<MarketVolumetricsIndicesFiveMin>(key).unwrap(),
                };
                self.set_market_index_helper(new_key,new_value,timeframe)?;
                return Ok(())
                // break
            }

            break;
        }


        // matched no shards, create new shard
        let shard_key = match timeframe {
            Timeframe::Daily => ShardedKey {key: volume_data.0.clone() as MarketAddress, max_shard_value: volume_data.1.clone()},
            Timeframe::Hourly => ShardedKey{key: volume_data.0.clone() as MarketAddress, max_shard_value: volume_data.1.clone()},
            Timeframe::FiveMinute => ShardedKey{key: volume_data.0.clone() as MarketAddress, max_shard_value: volume_data.1.clone()},
        };
        let shard_value = VolumeKeysWithData {
            volume_keys: vec![VolumeKeyWithData{key:volume_data.2.clone(), timestamp: volume_data.1.clone()}],
        };
        self.set_market_index_helper(shard_key,shard_value, timeframe)?;

        Ok(())
    }



    fn add_indices(&self, volume_data: (&H256, &u64, &u64), timeframe: Timeframe) -> Result<()> {

        self.add_ts_index((volume_data.1,volume_data.2), timeframe)?;
        self.add_market_index(volume_data,timeframe)?;

        Ok(())
    }

    fn bulk_volume_market_indices (&self, market_address: &H256, keys: Vec<VolumeKeyWithData>,timeframe: Timeframe) -> Result<()> {
        // market address index table
        let mut options = rocksdb::ReadOptions::default();
        options.set_iterate_range(rocksdb::PrefixRange(market_address.as_bytes()));

        let mut iter = match timeframe {
            Timeframe::Daily =>  self.db.dae_new_cursor::<MarketVolumetricsIndicesDay>(options),
            Timeframe::Hourly =>self.db.dae_new_cursor::<MarketVolumetricsIndicesHour>(options),
            Timeframe::FiveMinute => self.db.dae_new_cursor::<MarketVolumetricsIndicesFiveMin>(options),
        }?;



        iter.seek_to_last();
        if iter.valid() {
            let k = iter.key().unwrap();
            let value = iter.value().unwrap();

            let kv = match timeframe {
                Timeframe::Daily => decoder::<MarketVolumetricsIndicesDay>((k.to_vec(), value.to_vec())).unwrap(),
                Timeframe::Hourly => decoder::<MarketVolumetricsIndicesHour>((k.to_vec(), value.to_vec())).unwrap(),
                Timeframe::FiveMinute => decoder::<MarketVolumetricsIndicesFiveMin>((k.to_vec(), value.to_vec())).unwrap(),
            };


            let key = kv.0;
            let value = kv.1;
            let available_slots = value.volume_keys.len() - NUM_OF_INDICES_IN_SHARD;
            let mut new_shard_value = value.clone();
            let mut new_key = key.clone();
            let keys_to_save = &keys[0..available_slots as usize];
            new_shard_value.volume_keys.extend(keys_to_save);
            new_key.max_shard_value = new_shard_value.volume_keys.last().unwrap().timestamp.clone();
            self.set_market_index_helper(new_key,new_shard_value,timeframe)?;
            
            match timeframe {
                Timeframe::Daily => self.db.dae_delete::<MarketVolumetricsIndicesDay>(key).unwrap(),
                Timeframe::Hourly => self.db.dae_delete::<MarketVolumetricsIndicesHour>(key).unwrap(),
                Timeframe::FiveMinute => self.db.dae_delete::<MarketVolumetricsIndicesFiveMin>(key).unwrap(),
            };

            // new shard to be created
            if keys.len() > keys_to_save.len() {
                let volume_keys_to_save = &keys[available_slots as usize..];
                let new_shard_value_2 = VolumeKeysWithData {volume_keys: volume_keys_to_save.to_vec()};
                let new_shard_key = ShardedKey { key: market_address.clone(), max_shard_value: new_shard_value_2.volume_keys.last().unwrap().timestamp.clone()};
                self.set_market_index_helper(new_shard_key,new_shard_value_2,timeframe)?;
            }
            return Ok(())
        } 

        // first shard
        self.set_market_index_helper(ShardedKey { key: market_address.clone(), max_shard_value: keys.last().unwrap().timestamp.clone() },VolumeKeysWithData { volume_keys: keys },timeframe)?;
        Ok(())
    }

    fn bulk_volume_timestamp_indices (&self, timestamp: u64, keys: Vec<u64>, timeframe: Timeframe) -> Result<()> {
        let _ = match timeframe {
            Timeframe::Daily => {
                let existing = self.db.dae_get::<TimestampVolumetricsIndicesDay>(timestamp)?;
                match existing {
                    Some(mut matched_existing) => {
                        matched_existing.volume_keys.extend(keys);
                        self.db.dae_put::<TimestampVolumetricsIndicesDay>(timestamp, matched_existing)?;
                    },
                    None => {
                        let new_entry = VolumeKeys {
                            volume_keys: keys
                        };
                        self.db.dae_put::<TimestampVolumetricsIndicesDay>(timestamp, new_entry)?;
                    }
                }
            },
            Timeframe::Hourly => {
                let existing = self.db.dae_get::<TimestampVolumetricsIndicesHour>(timestamp)?;
                match existing {
                    Some(mut matched_existing) => {
                        matched_existing.volume_keys.extend(keys);
                        self.db.dae_put::<TimestampVolumetricsIndicesHour>(timestamp, matched_existing)?;
                    },
                    None => {
                        let new_entry = VolumeKeys {
                            volume_keys: keys
                        };
                        self.db.dae_put::<TimestampVolumetricsIndicesHour>(timestamp, new_entry)?;
                    }
                }
            },
            Timeframe::FiveMinute => {
                let existing = self.db.dae_get::<TimestampVolumetricsIndicesFiveMin>(timestamp)?;
                match existing {
                    Some(mut matched_existing) => {
                        matched_existing.volume_keys.extend(keys);
                        self.db.dae_put::<TimestampVolumetricsIndicesFiveMin>(timestamp, matched_existing)?;
                    },
                    None => {
                        let new_entry = VolumeKeys {
                            volume_keys: keys
                        };
                        self.db.dae_put::<TimestampVolumetricsIndicesFiveMin>(timestamp, new_entry)?;
                    }
                }
            },
        };
    Ok(())
    }
}



#[cfg(test)]
mod test {
    use crate::traits::{VolumetricWriter, Timeframe, VolumetricReader};
    use crate::{providers::options::AccessType, DatabaseProvider};
    use db::tables::models::sharded_key::NUM_OF_INDICES_IN_SHARD;
    use db::{
        init_db, 
        test_utils::ERROR_TEMPDIR,
    };
    use simp_primitives::address_balance::AddressBalance;
    use simp_primitives::{Volumetric, H256};
    use revm_primitives::U256;
    use std::str::FromStr;

    fn get_provider() -> DatabaseProvider {
        let db = init_db(&tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path()).unwrap();

        DatabaseProvider::new(db, AccessType::Primary)
    }


    #[test]
    fn insert_and_retrieve() {
        let provider = get_provider();
        let new_volume = Volumetric {
            timestamp: 1,
            market_address: H256::zero(),
            swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            transfer:  U256::from_str("0").unwrap(),
        };
        let key = provider.add_volume(new_volume,Timeframe::Daily,1,true).expect("Expect to insert volume");

        let matched_volume = provider.get_volume_by_key(key, Timeframe::Daily).expect("Expect to find matching volume");
        assert!(matched_volume.is_some());
        assert_eq!(key, 1);
        
        match matched_volume {
            Some(v) => assert!(v.timestamp == 1),
            _ => panic!()
        }

    }

    #[test]
    fn add_indices() {
        let provider = get_provider();
        let new_volume = Volumetric {
            timestamp: 1,
            market_address: H256::zero(),
            swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            transfer:  U256::from_str("0").unwrap(),
        };
        let new_volume2 = Volumetric {
            timestamp: 2,
            market_address: H256::zero(),
            swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            transfer:  U256::from_str("0").unwrap(),
        };
        let _ = provider.add_volume(new_volume,Timeframe::Daily,1,true).expect("Expect to insert volume");
        
        let index_keys = provider.get_market_volume_keys(H256::zero(), Timeframe::Daily).expect("Expect to find indices");
        match index_keys {
            Some(keys) => {
                assert!(keys.len() == 1);
            },
            _ => panic!("No indices found")
        }
        
        let _ = provider.add_volume(new_volume2,Timeframe::Daily,2,true).expect("Expect to insert volume");



        let index_keys = provider.get_market_volume_keys(H256::zero(), Timeframe::Daily).expect("Expect to find indices");
        match index_keys {
            Some(keys) => {
                assert!(keys.len() == 2);
                assert!(keys[0].timestamp == 1 && keys[1].timestamp == 2)
            },
            _ => panic!("No indices found")
        }


        // testing timestamp indices
        let ts_res1 = provider.get_by_timestamp(Timeframe::Daily, 1).expect("expect to find matching volumes");
        let ts_res2 = provider.get_by_timestamp(Timeframe::Daily, 2).expect("expect to find matching volumes");
        assert!(ts_res1[0].timestamp == 1);
        assert!(ts_res2[0].timestamp == 2);
        assert!(ts_res1.len() == 1);
        assert!(ts_res2.len() == 1);
        

    }

    #[test]
    fn get_latest() {
        let provider = get_provider();
        let new_volume = Volumetric {
            timestamp: 1,
            market_address: H256::zero(),
            swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            transfer:  U256::from_str("0").unwrap(),
        };
        let new_volume2 = Volumetric {
            timestamp: 2,
            market_address: H256::zero(),
            swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            transfer:  U256::from_str("0").unwrap(),
        };
        let _ = provider.add_volume(new_volume,Timeframe::Daily,1,true).expect("Expect to insert volume");
        let _ = provider.add_volume(new_volume2,Timeframe::Daily,2,true).expect("Expect to insert volume");

        let latest = provider.get_latest_market_volume(H256::zero(), Timeframe::Daily).expect("Expect to retrieve latest volume");
        
        
        assert!(latest.is_some());
        
        match latest {
            Some(v) => assert!(v.timestamp == 2),
            _ => panic!()
        }

    }
    
    
    #[test]
    fn get_range() {
        let provider = get_provider();
        let mut volumes_to_add = vec![];
        let num  = NUM_OF_INDICES_IN_SHARD;
        for i in 0..num {
            let new_volume = Volumetric {
                timestamp: i as u64,
                market_address: H256::zero(),
                swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
                swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
                withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
                mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
                transfer:  U256::from_str("0").unwrap(),
            };
            volumes_to_add.push(new_volume)
        }
        let _ = provider.add_market_volumes(volumes_to_add, Timeframe::Daily);
        
        let volumes_retrieved = provider.get_market_range(H256::zero(), Timeframe::Daily,None,None).expect("Expect to retrieve range");
        let volumes_retrieved2 = provider.get_market_range(H256::zero(), Timeframe::Daily,Some(20),Some(30)).expect("Expect to retrieve range");
        let volumes_retrieved3 = provider.get_market_range(H256::zero(), Timeframe::Daily,Some(num as u64 - 50),None).expect("Expect to retrieve range");
        assert!(volumes_retrieved.len() == num.clone() as usize);
        assert!(volumes_retrieved2.len() == 11);
        assert!(volumes_retrieved3.len() == 50);
    }
    #[test]
    fn get_by_timestamp() {
        let provider = get_provider();
        let new_volume = Volumetric {
            timestamp: 1,
            market_address: H256::zero(),
            swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            transfer:  U256::from_str("0").unwrap(),
        };
        let new_volume2 = Volumetric {
            timestamp: 2,
            market_address: H256::zero(),
            swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            transfer:  U256::from_str("0").unwrap(),
        };
        let new_volume3 = Volumetric {
            timestamp: 2,
            market_address: H256::zero(),
            swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            transfer:  U256::from_str("0").unwrap(),
        };
        let _ = provider.add_volume(new_volume,Timeframe::Daily,1,true).expect("Expect to insert volume");
        let _ = provider.add_volume(new_volume2,Timeframe::Daily,2,true).expect("Expect to insert volume");
        let _ = provider.add_volume(new_volume3,Timeframe::Daily,3,true).expect("Expect to insert volume");
        
        let retrieved_v = provider.get_by_timestamp(Timeframe::Daily, 2).expect("Expect to retrieve volume");
        assert!(retrieved_v.len() == 2);
    }

   
}
