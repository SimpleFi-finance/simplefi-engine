use async_trait::async_trait;
use simplefi_redis::{get_from_hset, key_exists_hset, store_in_hset};

use super::dragonfly_driver::ProtocolDragonflyDriver;

#[async_trait]
pub trait DailyAggregation {
    fn resolve_aggregation_key(&self) -> String;
    async fn get_latest_aggregation_ts(&self) -> Result<u64, Box<dyn std::error::Error>>;

    async fn set_latest_aggregation_ts(
        &self,
        ts: &u64,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
impl DailyAggregation for ProtocolDragonflyDriver {
    fn resolve_aggregation_key(&self) -> String {
        format!("gold_daily_aggregation")
    }
    async fn get_latest_aggregation_ts(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let hmap_name = self.resolve_aggregation_key();

        let exists = key_exists_hset(&mut self.connection, &hmap_name, &self.chain).await?;

        if exists {
            let latest_aggregation_ts =
                get_from_hset(&mut self.connection, &hmap_name, &self.chain).await?;

            let ts: u64 = latest_aggregation_ts.parse().unwrap();
            Ok(ts)
        } else {
            // if no aggregations have taken place
            self.set_latest_aggregation_ts(&0).await?;

            Ok(0)
        }
    }

    async fn set_latest_aggregation_ts(
        &self,
        ts: &u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hmap_name = self.resolve_aggregation_key();
        let _ = store_in_hset(
            &mut self.connection,
            &hmap_name,
            &self.chain,
            &ts.to_string(),
        )
        .await?;

        Ok(())
    }
}
