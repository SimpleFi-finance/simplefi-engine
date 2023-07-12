use crate::types::redis_driver::ProtocolRedisDriver;

pub async fn migrate_to_mongo() {
    let mut redis_driver = ProtocolRedisDriver::new().await;
    let stored_market_volumes = redis_driver.get_all_volumes().await;

    for market_volumes in stored_market_volumes {}
}
