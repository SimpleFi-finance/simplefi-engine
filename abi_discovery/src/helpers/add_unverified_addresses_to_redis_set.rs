use redis::RedisError;

use crate::settings::load_settings;


pub async fn add_unverified_addresses_to_redis_set(addresses: Vec<String>) -> Result<(), RedisError> {
    let settings = load_settings().expect("Failed to load settings");

    let redis_uri = settings.redis_uri.to_string();

    let client = redis::Client::open(redis_uri)?;
    let mut con = client.get_connection()?;

    for address in addresses {
        let _: () = redis::cmd("SADD")
            .arg("unverified_addresses")
            .arg(address)
            .query(&mut con)?;
    }

    Ok(())
}
