use bson::{doc, Binary};
use mongodb::{ options::FindOneOptions };
use log::info;

use crate::settings::load_settings;
use shared_types::{abi::Abi, mongo::abi::{AbiCollection, ContractAbiCollection }};
// use solidity::abi_to_bytecode;
use third_parties::mongo::{Mongo, MongoConfig, lib::abi_discovery::save_abi};



/* {
    lib::abi_discovery::{ save_abi },

};
 */
pub async fn process_abi(
    address: &String,
    abi_string: &String,
) {
    let settings = load_settings().expect("Failed to load settings");

    let mongo_uri = settings.mongodb_uri;
    let mongodb_database_name = settings.mongodb_database_name;
    let abis_collection_name = settings.mongodb_abi_collection;

    let config = MongoConfig {
        uri: mongo_uri.clone(),
        database: mongodb_database_name.clone(),
    };

    info!("Processing ABI. Checking if it's already in the database otherwise inserting it");
    info!("mongodb_uri: {}", mongo_uri);
    info!("mongodb_database_name: {}", mongodb_database_name);

    let db = Mongo::new(&config)
        .await
        .expect("Failed to connect to mongo");

    let abi_collection = db.collection::<AbiCollection>(&abis_collection_name);

    let mut abi: Vec<Abi> = serde_json::from_str(&abi_string).unwrap();

    // sort abi
    Abi::sort_abi_elements(&mut abi);

    // sort parameters
    for abi in &mut abi {
        abi.sort_parameters();
    }

    let abi_bytecode = bincode::serialize(&abi).unwrap();

    // Check if there's any document with the 'abi' field equal to the ABI bytecode.
    let abi_binary = Binary {
        subtype: bson::spec::BinarySubtype::Generic,
        bytes: abi_bytecode,
    };

    let filter = doc! { "abi": &abi_binary };

    let result = abi_collection
        .find_one(filter, None)
        .await
        .expect("Failed to execute find");

    let mut new_index: u32 = 0;

    match result {
        Some(document) => {
            info!("Found a repeated ABI for document with index: {:?}", document.index);

            new_index = document.index;

            info!("Assining {:?} address with with index {:?}", address, new_index);

        }
        None => {
            info!("No document found. finding index on last document");

            let options = FindOneOptions::builder()
            .sort(doc! { "index": -1 })
            .build();

            let last_document_result = abi_collection
                .find_one(doc! {}, options)
                .await
                .expect("Failed to read last document");

            match last_document_result {
                Some(document) => {
                    info!("Found last document with index: {:?}", document.index);

                    new_index = document.index + 1;

                    info!("Assining {:?} address with with index {:?}", &address, &new_index);

                    save_abi(&abi_collection, &new_index, &abi_binary).await;
                }
                None => {
                    info!("No document found. Creating first document");

                    save_abi(&abi_collection, &new_index, &abi_binary).await;

                    info!("Saved first document in all collection");
                }
            }
        }
    }

    /* debug!("new_index: {:?}", new_index); */

    // First we check if the contract is not already in contracts-abi
    let contract_abi_collection = db.collection::<ContractAbiCollection>("contracts-abi");

    let filter = doc! { "address": address };

    let result = contract_abi_collection
        .find_one(filter, None)
        .await
        .expect("Failed to find a contract in contract_address");

    match result {
        Some(document) => {
            info!("Found a contract with address: {:?}. skipping", document.address);

            /* Ok(false) */
        }
        None => {
            info!("No contract found. Linking contract {:?}. ", address);

            /* let contract = ContractAbiCollection {
                timestamp: chrono::Utc::now().timestamp_millis(),
                address: address.clone(),
                index: new_index,
                flag: ContractAbiFlag::Verified,
            };

            contract_abi_collection
                .insert_one(contract, None)
                .await
                .expect("Failed to link the new contract");

            Ok(true) */
        }
    }
}
