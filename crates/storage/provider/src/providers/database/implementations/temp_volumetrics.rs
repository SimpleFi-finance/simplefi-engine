use crate::{DatabaseProvider, traits::TempVolumetrics};
use crate::traits::Timeframe;
use db::tables::{TempPeriodVolumesFive, ShardedKey, TempPeriodVolumesHour};
use interfaces::Result;
use db::transaction::DbTx;
use primitives::{PeriodVolumes, Volumetric,H256};



impl TempVolumetrics for DatabaseProvider {

    fn get_temp_volumetric (&self, market_address: &H256, timestamp: u64, timeframe:crate::traits::Timeframe) -> Result<Option<PeriodVolumes>> {
        let result = match timeframe {
          Timeframe::FiveMinute => self.db.get::<TempPeriodVolumesFive>(ShardedKey{key: market_address.clone(),max_shard_value:timestamp}),
          Timeframe::Hourly => self.db.get::<TempPeriodVolumesHour>(ShardedKey{key: market_address.clone(),max_shard_value:timestamp}),
          _ => panic!("Timeframe not valid")
        }?;
        Ok(result)
    }

    fn set_temp_volumetric (&self, market_address: &H256, timestamp: u64, timeframe:crate::traits::Timeframe, period_volume: Volumetric) -> Result<()> {
      let  existing = self.get_temp_volumetric(market_address, timestamp, timeframe)?;

      match existing {
        Some(mut e) => {
          e.volumes.push(period_volume);
          let _ = match timeframe {
            Timeframe::FiveMinute => self.db.put::<TempPeriodVolumesFive>(ShardedKey{key: market_address.clone(),max_shard_value:timestamp},e),
            Timeframe::Hourly => self.db.put::<TempPeriodVolumesHour>(ShardedKey{key: market_address.clone(),max_shard_value:timestamp},e),
            _ => panic!("Timeframe not valid")
          }?;
        },
        _ => {
          let new: PeriodVolumes = PeriodVolumes{volumes:vec![period_volume]};
          let _ = match timeframe {
            Timeframe::FiveMinute => self.db.put::<TempPeriodVolumesFive>(ShardedKey{key: market_address.clone(),max_shard_value:timestamp},new),
            Timeframe::Hourly => self.db.put::<TempPeriodVolumesHour>(ShardedKey{key: market_address.clone(),max_shard_value:timestamp},new),
            _ => panic!("Timeframe not valid")
          }?;
        }
      }
      
      
      Ok(())
      
    }

    fn read_period_volumes (&self, market_address: &H256, timestamp: u64, timeframe: crate::traits::Timeframe) -> Result<Option<PeriodVolumes>> {        
        let res = self.get_temp_volumetric(market_address, timestamp, timeframe)?;
        Ok(res)
    }

    fn write_period_volumes (&self, market_address: &H256, timestamp: u64,volumes: Vec<Volumetric>, timeframe: crate::traits::Timeframe) -> Result<()> {
        let existing = self.get_temp_volumetric(market_address, timestamp, timeframe)?;

        match existing  {
          Some(mut pv) => {
              pv.volumes.extend(volumes);
              let _ = match timeframe {
                Timeframe::FiveMinute => self.db.put::<TempPeriodVolumesFive>(ShardedKey{key: market_address.clone(),max_shard_value:timestamp},pv),
                Timeframe::Hourly => self.db.put::<TempPeriodVolumesHour>(ShardedKey{key: market_address.clone(),max_shard_value:timestamp},pv),
                _ => panic!("Timeframe not valid")
              }?;
          },
          _ => {
            let new: PeriodVolumes = PeriodVolumes{volumes};
            let _ = match timeframe {
              Timeframe::FiveMinute => self.db.put::<TempPeriodVolumesFive>(ShardedKey{key: market_address.clone(),max_shard_value:timestamp},new),
              Timeframe::Hourly => self.db.put::<TempPeriodVolumesHour>(ShardedKey{key: market_address.clone(),max_shard_value:timestamp},new),
              _ => panic!("Timeframe not valid")
            }?;
          }
        }
        
        Ok(())
    }

    fn delete_period_volumes (&self, market_address: &H256, timestamp: u64, timeframe: crate::traits::Timeframe) -> Result<()> {
      let _ = match timeframe {
        Timeframe::FiveMinute => self.db.delete::<TempPeriodVolumesFive>(ShardedKey{key: market_address.clone(),max_shard_value:timestamp}),
        Timeframe::Hourly => self.db.delete::<TempPeriodVolumesHour>(ShardedKey{key: market_address.clone(),max_shard_value:timestamp}),
        _ => panic!("Timeframe not valid")
      }?;
      Ok(())
    }

 
}


#[cfg(test)]
mod test {
  use crate::traits::{Timeframe, TempVolumetrics};
  use crate::{providers::options::AccessType, DatabaseProvider};
  use db::{
      implementation::sip_rocksdb::DB, init_db, 
      test_utils::ERROR_TEMPDIR,
  };
  use primitives::address_balance::AddressBalance;
  use primitives::{Volumetric, H256};
  use revm_primitives::U256;
  use std::str::FromStr;


  fn get_provider() -> DatabaseProvider {
    let db = init_db(&tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path());

    let db = DB::new(db.unwrap());

    DatabaseProvider::new(db, AccessType::Primary)
}
  #[test]
  fn insert_and_retrieve_volume()  {
    let provider = get_provider();
    let volume_to_add = Volumetric {
        timestamp: 1,
        market_address: H256::zero(),
        swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
        swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
        withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
        mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
        transfer:  U256::from_str("0").unwrap(),
    };

    let _ = provider.set_temp_volumetric(&H256::zero(), 2,Timeframe::FiveMinute,volume_to_add.clone());
    let _ = provider.set_temp_volumetric(&H256::zero(), 2,Timeframe::FiveMinute,volume_to_add);
    let volumes = provider.read_period_volumes(&H256::zero(), 2,Timeframe::FiveMinute).unwrap();

    match volumes {
      Some(vs) => {
        assert!(vs.volumes.len() == 2)
      },
      _ => panic!("failed to retrieve volumes")
    }
  
  }
  #[test]
  fn insert_and_retrieve_bulk()  {
    let provider = get_provider();
    let volumes_to_add = vec![Volumetric {
        timestamp: 1,
        market_address: H256::zero(),
        swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
        swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
        withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
        mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
        transfer:  U256::from_str("0").unwrap(),
    },Volumetric {
      timestamp: 2,
      market_address: H256::zero(),
      swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
      swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
      withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
      mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
      transfer:  U256::from_str("0").unwrap(),
  }];
    let volumes_to_add2 =vec![Volumetric {
        timestamp: 3,
        market_address: H256::zero(),
        swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
        swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
        withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
        mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
        transfer:  U256::from_str("0").unwrap(),
    },Volumetric {
      timestamp: 4,
      market_address: H256::zero(),
      swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
      swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
      withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
      mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
      transfer:  U256::from_str("0").unwrap(),
  }];

    let _ = provider.write_period_volumes(&H256::zero(), 5,volumes_to_add.clone(),Timeframe::FiveMinute);
    let _ = provider.write_period_volumes(&H256::zero(), 5,volumes_to_add2.clone(),Timeframe::FiveMinute);
    let volumes = provider.read_period_volumes(&H256::zero(), 5,Timeframe::FiveMinute).unwrap();

    match volumes {
      Some(vs) => {
        assert!(vs.volumes.len() == 4)
      },
      _ => panic!("failed to retrieve volumes")
    }
  
  }

}
 
