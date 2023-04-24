use log::{ info, debug, error };
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
///   Ok(abis) => info!("abis: {:?}", abis),
///  Err(e) => error!("error: {:?}", e),
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
    let mysettings = load_settings()?;

    let mongo_uri = mysettings.mongodb_uri;
    let mongodb_database_name = mysettings.mongodb_database_name;
    let factory_contracts_collection = mysettings.mongodb_factory_contracts_collection;

    let config = MongoConfig {
        uri: mongo_uri.clone(),
        database: mongodb_database_name.clone(),
    };

    info!("Adding factory addresses to the factory_contracts collection... ");
    debug!("mongodb_uri: {}", mongo_uri);
    debug!("mongodb_database_name: {}", mongodb_database_name);

    let db = Mongo::new(&config).await?;

    let contract_abi_collection = db.collection(&factory_contracts_collection);

    // Create an array of documents with the same field1 value and each element of field2 as a separate document
    let documents_to_insert = addresses
        .iter()
        .map(|address| doc! { "address": address, "factory": factory_address.clone() }  )
        .collect::<Vec<bson::Document>>();

    let mut skipped_documents = 0;
    let mut inserted_documents = 0;

    // Connect to redis
    let redis_uri = mysettings.redis_uri.to_string();

    let mut redis_con = connect(redis_uri.as_str()).await.unwrap();

    for document in &documents_to_insert {
        match contract_abi_collection.insert_one(document.clone(), None).await {
            Ok(result) => {
                debug!("Document inserted with _id: {:?}", result.inserted_id);
                inserted_documents = inserted_documents + 1;

                let address = document.get("address").unwrap().as_str().unwrap();
                debug!("Adding address to redis: {}", address);

                let result = add_to_set(&mut redis_con, "tracked_addresses", address).await;

                match result {
                    Ok(_) => debug!("Added address to redis"),
                    Err(e) => error!("Error adding address to redis: {:?}", e),
                }
            }
            Err(_) => {
                skipped_documents = skipped_documents + 1;
            }
        }
    }

    info!("Inserted {} documents", inserted_documents);
    debug!("Skipped {} documents", skipped_documents);

    Ok(true)
}
