use crate::mongo::{MongoConfig, Mongo};

///
/// Create a mongo connection with the default settings
///
/// # Example
///
/// ```
/// use abi_discovery::helpers::connections::get_default_mongo;
///
/// #[tokio::main]
/// async fn main() {
///    let mongo = get_default_mongo().await;
/// }
/// ```
///
/// # Returns
///
/// A Mongo connection
///
/// # Panics
///
/// Panics if the settings file is not found or if the mongo connection fails
///
pub async fn get_default_connection(uri: &str, database: &str) -> Mongo {
    let mongo_config = MongoConfig {
        uri: uri.to_string(),
        database: database.to_string(),
    };

    let mongo = Mongo::new(&mongo_config)
        .await
        .expect("Failed to create mongo Client");

    mongo
}
