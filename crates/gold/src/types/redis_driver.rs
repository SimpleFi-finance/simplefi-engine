use redis::{aio::Connection, RedisError};
use simplefi_engine_settings::load_settings;
use simplefi_redis::{add_to_set, connect, is_in_set};
pub struct ProtocolRedisDriver {
    connection: Connection,
}

impl ProtocolRedisDriver {
    pub fn resolve_set_name(
        &self,
        protocol_controller_id: &str,
    ) -> String {
        format!("gold_protocol_driver_{}", protocol_controller_id)
    }

    pub async fn new() -> Self {
        let mysettings = load_settings().expect("Failed to load settings");
        let redis_connection = connect(&mysettings.redis_uri)
            .await
            .expect("Expect to connect to redis");

        Self {
            connection: redis_connection,
        }
    }

    pub async fn set_market_driver(
        &mut self,
        market_address: String,
        protocol_driver_id: &str,
    ) -> Result<(), RedisError> {
        let list_name = self.resolve_set_name(&protocol_driver_id).clone();
        add_to_set(&mut self.connection, &list_name, &market_address).await
    }

    pub async fn get_protocol_driver(
        &mut self,
        market_address: &str,
        protocol_id: &str,
    ) -> Result<bool, RedisError> {
        let list_name = self.resolve_set_name(&protocol_id).clone();
        is_in_set(&mut self.connection, &list_name, market_address).await
    }
    // TODO:
    // set volumetric

    // get volumetric

    // set snapshot

    // get snapshot

    // get all snapshots

    // get all volumetrics
}
