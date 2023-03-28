use third_parties::redis::{ connect, add_to_set, is_in_set };
use settings::{ load_settings };
use redis::RedisError;


pub async fn check_abis(addresses: &[String]) -> Result<Vec<String>, RedisError> {
    let set_key = "tracked_addresses";
    let set_verify_key = "verify_addresses";

    let mut tracked_addresses = Vec::new();

    let settings = load_settings().expect("Failed to load settings");
    let redis_uri = settings.redis_uri.to_string();

    let mut con = connect(redis_uri.as_str()).await.unwrap();

    for address in addresses {
        if is_in_set(&mut con, set_key, address).await? {
            tracked_addresses.push(address.clone());
        } else {
            add_to_set(&mut con, set_verify_key, address).await?;
        }
    }

    Ok(tracked_addresses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_abis() {
        let redis_uri = "redis://localhost:6379/";

        let mut con = connect(redis_uri).await.unwrap();

        // generate 20 random ethereum addresses
        let mut addresses = Vec::new();
        for _ in 0..20 {
            let address = format!("0x{}", hex::encode(&rand::random::<[u8; 20]>()));
            addresses.push(address);
        }

        // add 10 of them to the tracked_addresses set
        for address in &addresses[0..10] {
            add_to_set(&mut con, "tracked_addresses", address).await.unwrap();
        }

        // check if the 20 addresses are in the tracked_addresses set
        let tracked_addresses = check_abis(&addresses).await.unwrap();

        // check if the 10 addresses are in the tracked_addresses set
        assert_eq!(tracked_addresses.len(), 10);

        // check if the 10 addresses are in the tracked_addresses set
        for address in &addresses[0..10] {
            assert!(tracked_addresses.contains(address));
        }

        // check if the 10 addresses are in the verify_addresses set
        for address in &addresses[10..20] {
            assert!(is_in_set(&mut con, "verify_addresses", address).await.unwrap());
        }



    }
}

