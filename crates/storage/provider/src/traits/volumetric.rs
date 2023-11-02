use db::tables::models::VolumeKeyWithData;
use simp_primitives::{H256, MarketAddress, VolumeKey, Volumetric, PeriodVolumes};
use interfaces::Result;
use db::tables::models::sharded_key::ShardedKey;
use db::tables::models::volumetric::VolumeKeysWithData;

// TODO switch to main repo timeframe type

#[derive(Debug, Clone, Copy)]
pub enum Timeframe {
    Daily,
    Hourly,
    FiveMinute,
}





/// Client trait for reading [Volumetric]s
pub trait VolumetricReader: Send + Sync {
    // /// Retrieves all volumes for a given market, for a given timeframe
    // fn get_market_volumes(&self, market_address: H256, timeframe: Timeframe) -> Result<Vec<Volumetric>>;


    /// a helper for getting volumes
    /// Internal use only
    fn get_volume_helper (&self, key: VolumeKey, timeframe: Timeframe) -> Result<Option<Volumetric>>;




    /// Gets all volumes for a given range for a market address, for a given timeframe
    /// Some(from) and Some(to):  Between range
    /// Some(from) and None(to): All more recent than from value
    /// None, None: All market volumes
    fn get_market_range(&self, market_address: H256, timeframe: Timeframe, from: Option<u64>, to:Option<u64>) -> Result<Vec<Volumetric>>;

    
    /// Gets all volumes for a given timestamp, for a given range
    /// Returns a vector of Tuples, containing the market address and market volume
    fn get_by_timestamp(&self, timeframe:Timeframe, timestamp: u64) -> Result<Vec<Volumetric>>;

    fn get_latest_market_volume (&self, market_address: H256,timeframe:Timeframe) -> Result<Option<Volumetric>>;

    fn get_volume_by_key (&self, key: u64,timeframe: Timeframe) -> Result<Option<Volumetric>>;

    fn get_market_volume_keys (&self, market_address: H256, timeframe: Timeframe) -> Result<Option<Vec<VolumeKeyWithData>>>;
}


/// Client trait for inserting [Volumetric] entries and relevant indices into index tables
pub trait VolumetricWriter: Send + Sync {
    /// A helper for getting the last active key
    fn get_last_volume_id_or_default(&self,timeframe:Timeframe) -> Result<u64>;

    /// a helper for setting volumes
    /// Internal use only
    fn set_volume_helper (&self, key: VolumeKey, volume: Volumetric, timeframe: Timeframe) -> Result<()>;


    /// a helper for setting market volume indices
    /// Internal use only
    fn set_market_index_helper(&self, key: ShardedKey<MarketAddress>, value: VolumeKeysWithData, timeframe:Timeframe) -> Result<()>;


    /// Inserts a volumetric and returns it's entry key
    fn add_volume(&self, volume: Volumetric, timeframe: Timeframe,key:u64, add_indices:bool) -> Result<u64>;

    /// Inserts a vector of market volumes for a given timeframe
    fn add_market_volumes(&self, volumes: Vec<Volumetric>,timeframe:Timeframe) -> Result<()>;

    /// Add volumetric entry key to timestamp index table
    /// volume_data: Tuple (timestamp, VolumeKey)
    fn add_ts_index(&self, volume_data:(&u64,&u64), timeframe:Timeframe) -> Result<()>;

    /// Add volumetric entry key to marketAddress index table
    /// volume_data: Tuple (address,timestamp, VolumeKey)
    fn add_market_index(&self, volume_data:(&H256,&u64,&u64), timeframe:Timeframe) -> Result<()>;

    /// Add volumetric entry keys to index tables
    /// volume_data: Tuple.  Tuple: (market_address, timestamp, location_key)
    fn add_indices(&self, volume_data: (&H256,&u64,&u64), timeframe: Timeframe) -> Result<()>;

    /// Creates indices from a vector of volume keys for a given market
    fn bulk_volume_market_indices (&self, market_address: &H256, keys: Vec<VolumeKeyWithData>,timeframe: Timeframe) -> Result<()>;


    /// Creates indices from a vector of volume keys for a given timestamp
    fn bulk_volume_timestamp_indices (&self, timestamp: u64, keys: Vec<u64>, timeframe: Timeframe) -> Result<()>;
}

pub trait TempVolumetrics:  Send + Sync {
    // Helper for retrieving temp volumes
    fn get_temp_volumetric (&self, market_address: &H256, timestamp: u64, timeframe:Timeframe) -> Result<Option<PeriodVolumes>>;
    /// Helper for setting temp volumes
    /// It is the callees responsibility to ensure there are no duplications
    fn set_temp_volumetric (&self, market_address: &H256, timestamp: u64, timeframe:Timeframe, period_volume: Volumetric) -> Result<()>;
    /// retrieves period volumes for a given market/timestamp
    fn read_period_volumes (&self, market_address: &H256, timestamp: u64, timeframe: Timeframe) -> Result<Option<PeriodVolumes>>;
    /// Inserts period volumes for a given market/timestamp
    /// It is the callees responsibility to ensure there are no duplications and volumes are all for the same period
    fn write_period_volumes (&self, market_address: &H256, timestamp: u64,volumes: Vec<Volumetric>, timeframe: Timeframe) -> Result<()>;
    /// Deletes market/timestamp period entry
    fn delete_period_volumes (&self, market_address: &H256, timestamp: u64, timeframe: Timeframe) -> Result<()>;
}

