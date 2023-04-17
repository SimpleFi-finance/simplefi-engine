
use mongodb::{ Client, Collection, Database };

// Define a struct to hold the MongoDB configuration
#[derive(Clone, Debug)]
pub struct MongoConfig {
    pub uri: String,
    pub database: String,
}

// Define a struct to hold the MongoDB client
#[derive(Clone)]
pub struct Mongo {
    pub client: Client,
    pub database: Database,
}

// Implement the Mongo struct
impl Mongo {
    // Create a new instance of the Mongo struct
    pub async fn new(config: &MongoConfig) -> Result<Self, mongodb::error::Error> {
        // Create a new MongoDB client
        let client = Client::with_uri_str(&config.uri).await?;

        // Get a handle to the database
        let database = client.database(&config.database);

        // Return the Mongo struct
        Ok(Self { client, database })
    }

    // Get a handle to a MongoDB collection
    pub fn collection<T>(&self, name: &str) -> Collection<T>
    where
        T: serde::de::DeserializeOwned + serde::Serialize,
    {
        self.database.collection(name)
    }
}

pub fn binary_to_vec(binary: &bson::Binary) -> Vec<u8> {
    binary.bytes.to_vec()

    /* let mut vec = Vec::new();

    for byte in binary.bytes() {
        vec.push(*byte);
    }

    vec */
}

// create a test for Mongo Struct which connects to a localhost mongo
#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::doc;

    #[tokio::test]
    async fn test_mongo() {
        // Create a new MongoDB configuration
        let config = MongoConfig {
            uri: "mongodb://localhost:27017".to_string(),
            database: "test".to_string(),
        };

        // Create a new MongoDB client
        let mongo = Mongo::new(&config).await.unwrap();

        // Get a handle to the "users" collection
        let users = mongo.collection::<User>("users");

        // Insert a new user
        let inserted = users
            .insert_one(
                User {
                    name: "John Doe".to_string(),
                    age: 42,
                },
                None,
            )
            .await
            .unwrap();

        print!("{:?}", &inserted);

        // Find the user we just inserted
        let user = users
            .find_one(doc! { "name": "John Doe" }, None)
            .await
            .unwrap()
            .unwrap();

        print!("{:?}", &user);

        // Assert that the user's name is correct
        assert_eq!(user.name, "John Doe");

        // Assert that the user's age is correct
        assert_eq!(user.age, 42);

        // Drop the "users" collection
        users.drop(None).await.unwrap();

    }

    // Define a struct to hold user information
    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    struct User {
        name: String,
        age: u8,
    }
}



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
