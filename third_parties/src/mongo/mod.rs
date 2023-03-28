/* use mongodb::{Client, Collection, Database};
use once_cell::sync::OnceCell;
use std::sync::{Arc, RwLock};
use settings::load_settings; */

// We are creating a static thread safe reference to the mongodb client to be used across the monorepo
/* pub static CLIENT: Lazy<Arc<RwLock<Client>>> = Lazy::new(|| {
    let settings = load_settings().expect("Failed to load settings");
    let client_future = Client::with_uri_str(&settings.mongodb_uri);

    let client = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client_future)
        .expect("Failed to initialize standalone client.");

    Arc::new(RwLock::new(client))
}); */
/* static CLIENT: OnceCell<Client> = OnceCell::new(); */
 /*
async fn create_client() -> &'static Client {
    CLIENT.get_or_init(|| async {
        let client_uri = "mongodb://localhost:27017";
        Client::with_uri_str(client_uri)
            .await
            .expect("Failed to initialize client")
    }).await
}*/
/*
async fn create_client() -> &'static Client {
    let settings = load_settings().expect("Failed to load settings");
    CLIENT.get_or_init(|| async {
        let client_uri = settings.mongodb_uri;
        Client::with_uri_str(&client_uri).await.expect("Failed to initialize standalone client.");
    }).await
} */

/* pub fn get_database(database_name: &str) -> Database {
    let client = CLIENT.read().unwrap();

    client.database(database_name)
}

pub fn get_collection<T: serde::Serialize + serde::de::DeserializeOwned>(
    database_name: &str,
    collection_name: &str
) -> Collection<T> {
    get_database(database_name).collection::<T>(collection_name)
} */

/*
pub struct MongoClient {
    client: Arc<RwLock<Client>>,
}

impl MongoClient {
    pub fn new() -> Self {
        MongoClient {
            client: CLIENT.clone(),
        }
    }
}
 */

/* use mongodb::bson::{doc, Document, from_document};

use mongo::{get_collection, get_database};
use serde::{Deserialize, Serialize};
use std::error::Error; */
/*
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Abi {
    pub timestamp: u64,
    index: u32,
    abi: String,
}

#[tokio::test]
async fn test_get_collection() -> Result<(), Box<dyn Error>> {
    let database_name = "test";
    let collection_name = "test_abi";

    let db = get_database(database_name);
    db.drop(None).await?; // Drop the database if it exists

    let collection = get_collection::<Abi>(database_name, collection_name);

    let abi = Abi {
        timestamp: 123456789,
        index: 1,
        abi: "test".to_string(),
    };

    collection.insert_one(abi.clone(), None).await?;

    // Retrieve
    let result = collection.find_one(doc! {"index": 1}, None).await?;

    assert!(result.is_some());


    assert_eq!(abi.timestamp, retrieved_abi.timestamp);
    assert_eq!(abi.index, retrieved_abi.index);
    assert_eq!(abi.abi, retrieved_abi.abi);

    db.drop(None).await?; // Drop the database

    Ok(())

}
 */
