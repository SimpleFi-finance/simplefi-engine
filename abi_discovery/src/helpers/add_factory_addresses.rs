
use mongodb::{ bson::doc };

use crate::settings::load_settings;
use third_parties::{mongo::{MongoConfig, Mongo}, redis::{connect, add_to_set}};

///
/// Function to add factory addresses to the factory_contracts collection
/// This is used to track the addresses that are created by a factory
///
/// # Arguments
///
/// * `factory_address` - String of the factory address
/// * `addresses` - Vec<String> of addresses to add to the factory_contracts collection
///
/// # Returns
///
/// * `Result<bool, Box<dyn std::error::Error>>` - Result of bool or Error
///
/// # Example
///
/// ```
/// use abi_discovery::add_factory_addresses;
///
/// let factory_address = "0x..";
/// let addresses = vec!["0x.."];
///
/// let result = add_factory_addresses(factory_address, addresses).await;
///
/// match result {
///   Ok(abis) => println!("abis: {:?}", abis),
///  Err(e) => println!("error: {:?}", e),
/// }
/// ```
///
/// # Panics
///
/// This function will panic if the mongodb_uri is not set in the settings file
///
/// # Notes
///
/// This function will not add an address if it already exists in the collection
///
/// # TODO
///
pub async fn add_factory_addresses(
    factory_address: String,
    addresses: Vec<String>,
) -> Result<bool, Box<dyn std::error::Error>> {
    // First we are going to add the addresses to the factory_contracts collection
    let settings = load_settings()?;

    let mongo_uri = settings.mongodb_uri;
    let mongo_engine_db = settings.mongodb_engine_db;

    let config = MongoConfig {
        uri: mongo_uri.clone(),
        database: mongo_engine_db.clone(),
    };

    println!("Adding factory addresses to the factory_contracts collection... ");
    println!("mongodb_uri: {}", mongo_uri);
    println!("mongo_engine_db: {}", mongo_engine_db);

    let db = Mongo::new(&config).await?;

    let contract_abi_collection = db.collection("factory_contracts");

    // Create an array of documents with the same field1 value and each element of field2 as a separate document
    let documents_to_insert = addresses
        .iter()
        .map(|address| doc! { "address": address, "factory": factory_address.clone() }  )
        .collect::<Vec<bson::Document>>();

    let mut skipped_documents = 0;
    let mut inserted_documents = 0;

    // Connect to redis
    let redis_uri = settings.redis_uri.to_string();

    let mut redis_con = connect(redis_uri.as_str()).await.unwrap();



    for document in &documents_to_insert {
        match contract_abi_collection.insert_one(document.clone(), None).await {
            Ok(result) => {
                println!("Document inserted with _id: {:?}", result.inserted_id);
                inserted_documents = inserted_documents + 1;

                let address = document.get("address").unwrap().as_str().unwrap();
                println!("Adding address to redis: {}", address);

                let result = add_to_set(&mut redis_con, "tracked_addresses", address).await;

                match result {
                    Ok(_) => println!("Added address to redis"),
                    Err(e) => println!("Error adding address to redis: {:?}", e),
                }
            }
            Err(_) => {
                skipped_documents = skipped_documents + 1;
            }
        }
    }

    println!("Skipped {} documents", skipped_documents);
    println!("Inserted {} documents", inserted_documents);

    Ok(true)
}
