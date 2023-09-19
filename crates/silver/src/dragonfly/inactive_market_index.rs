use super::dragonfly_driver::ProtocolDragonflyDriver;
use async_trait::async_trait;
use redis::RedisError;
use simplefi_redis::{get_from_hset, store_in_hset};

#[async_trait]
pub trait InactiveMarketsDriver {
    async fn set_last_market_snapshot_timestamp(
        &mut self,
        market_address: &str,
        timestamp: &u64,
    ) -> Result<(), RedisError>;

    async fn get_last_market_snapshot_timestamp(
        &mut self,
        market_address: &str,
    ) -> Result<u64, RedisError>;
}

#[async_trait]
impl InactiveMarketsDriver for ProtocolDragonflyDriver {
    async fn set_last_market_snapshot_timestamp(
        &mut self,
        market_address: &str,
        timestamp: &u64,
    ) -> Result<(), RedisError> {
        let hmap_name = format!("{}_gold_inactive_markets", &self.chain);
        let parsed_ts = timestamp.to_string();
        let _ = store_in_hset(&mut self.connection, &hmap_name, market_address, &parsed_ts).await?;
        Ok(())
    }
    async fn get_last_market_snapshot_timestamp(
        &mut self,
        market_address: &str,
    ) -> Result<u64, RedisError> {
        let hmap_name = format!("{}_gold_inactive_markets", &self.chain);
        let value = get_from_hset(&mut self.connection, &hmap_name, market_address).await?;
        let timestamp: u64 = serde_json::from_str(&value).unwrap();

        Ok(timestamp)
    }
}
