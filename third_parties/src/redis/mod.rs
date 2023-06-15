use std::{pin::Pin};

// use futures::prelude::*;
use redis::{
    Client, AsyncCommands, RedisError, RedisResult,
    aio::{AsyncStream, Connection, ConnectionManager},
};
// create a helper to establish a connection to redis in async way
pub async fn connect(redis_uri: &str) -> RedisResult<Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>> {
    let client = redis::Client::open(redis_uri)?;
    let con = client.get_async_connection().await?;

    Ok(con)
}

// create a helper to establish a connection to redis in async way
pub async fn connection_manager(redis_uri: &str) -> Result<ConnectionManager, RedisError> {
    let client = redis::Client::open(redis_uri)?;

    ConnectionManager::new(client).await
}

pub async fn connect_client(redis_uri: &str) -> Result<Client, redis::RedisError> {
    Client::open(redis_uri)
}

// create a helper to add a String into a redis list in async way
pub async fn add_to_set(
    con: &mut Connection,
    list_name: &str,
    value: &str,
) -> RedisResult<()> {
    let _: () = con.sadd(list_name, value).await?;

    Ok(())
}

pub async fn publish_message(
    con: &mut Connection,
    channel: &str,
    message: &str,
) -> RedisResult<()> {
    let _: () = con.publish(channel, message).await?;

    Ok(())
}
// create a helper to check if a String is in a redis list in async way
pub async fn is_in_set(
    con: &mut Connection,
    set_name: &str,
    value: &str,
) -> RedisResult<bool> {
    let result: bool = con.sismember(set_name, value).await?;
    Ok(result)
}

pub async fn delete_set(
    connection: &mut redis::aio::Connection,
    set: &str,
) -> Result<(), RedisError> {
    connection.del(set).await?;
    Ok(())
}

pub async fn check_set_exists(connection: &mut Connection, set_key: &str) -> RedisResult<bool> {
    let exists: bool = connection.exists(set_key).await?;
    Ok(exists)
}

pub async fn has_items_in_queue(connection: &mut Connection, set_key: &str) -> RedisResult<bool> {
    let size: i64 = connection.scard(set_key).await?;

    Ok(size > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_to_set() {
        let redis_uri = "redis://localhost:6379/";
        let mut con = connect(redis_uri).await.unwrap();
        let result = add_to_set(&mut con, "test_set", "test").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_is_in_list() {
        let redis_uri = "redis://localhost:6379/";
        let mut con = connect(redis_uri).await.unwrap();
        let result = is_in_set(&mut con, "test_set", "test").await;

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_remove_list_and_check_fails() {
        let redis_uri = "redis://localhost:6379/";
        let mut con = connect(redis_uri).await.unwrap();
        let _ = add_to_set(&mut con, "test_set", "test").await;

        let _ = is_in_set(&mut con, "test_set", "test").await;

        let exists: bool = redis::cmd("EXISTS")
            .arg("test_set")
            .query_async(&mut con)
            .await
            .unwrap();

        assert_eq!(exists, true);

        let _ = delete_set(&mut con, "test_set").await;

        let exists: bool = redis::cmd("EXISTS")
            .arg("test_set")
            .query_async(&mut con)
            .await
            .unwrap();

        assert_eq!(exists, false);
    }
}
