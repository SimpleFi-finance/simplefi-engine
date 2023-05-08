use log::{ info, debug, error };
use mongodb::{ bson::doc };

use crate::settings::load_settings;
use third_parties::{
    mongo::lib::abi_discovery::get_default_connection,
    redis::{connect, add_to_set}
};

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
    info!("Adding factory addresses to the factory_contracts collection... ");

    let mysettings = load_settings()?;
    let factory_contracts_collection = mysettings.mongodb_factory_contracts_collection;

    let mongo = get_default_connection(&mysettings.mongodb_uri.as_str(), &mysettings.mongodb_database_name.as_str()).await;
    let contract_abi_collection = mongo.collection(&factory_contracts_collection);

    let documents_to_insert = addresses
        .iter()
        .map(|address| doc! { "address": address, "factory": factory_address.clone() }  )
        .collect::<Vec<bson::Document>>();

    let mut skipped_documents = 0;
    let mut inserted_documents = 0;

    let mut redis_con = connect(mysettings.redis_uri.as_str()).await.expect("Failed to connect to redis");
    let redis_tracked_addresses_set = mysettings.redis_tracked_addresses_set.to_string();

    for document in &documents_to_insert {
        match contract_abi_collection.insert_one(document.clone(), None).await {
            Ok(result) => {
                debug!("Document inserted with _id: {:?}", result.inserted_id);

                inserted_documents = inserted_documents + 1;

                let address = document.get("address").unwrap().as_str().unwrap();

                debug!("Adding address to redis: {}", address);

                let result = add_to_set(&mut redis_con, &redis_tracked_addresses_set, address).await;

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
