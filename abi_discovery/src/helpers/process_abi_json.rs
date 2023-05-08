use chrono::Utc;
use log::{ info, debug, error };
use mongodb::{ options::FindOneOptions, bson::doc };

use crate::settings::load_settings;
use shared_types::mongo::abi::{ ContractAbiCollection, AbiJSONCollection, ContractAbiFlag };
use third_parties::mongo::lib::abi_discovery::{save_abi_json, get_default_connection};

pub async fn process_abi_json(
    address: &String,
    abi_string: &String,
) -> bool {
    info!("Starting process_abi_json");

    let mysettings = load_settings().expect("Failed to load settings");

    let mongo = get_default_connection(&mysettings.mongodb_uri.as_str(), &mysettings.mongodb_database_name.as_str()).await;

    let abi_collection = mongo.collection::<AbiJSONCollection>(&mysettings.mongodb_abi_collection.as_str());
    let contract_abi_collection = mongo.collection::<ContractAbiCollection>(&mysettings.mongodb_contract_abi_collection.as_str());

    let filter = doc! { "address": address };

    let result = contract_abi_collection
        .find_one(filter, None)
        .await
        .expect("Failed to find a contract in contract_address");

    match result {
        Some(document) => {
            debug!("Found a contract with address: {:?}. skipping", document);

            return false;
        }
        None => {
            error!("No contract found. Looking for ABI in etherscan");
        }
    }

    if &abi_string.trim().replace(" ", "").len() < &10 {
        debug!("ABI is too short. skipping");

        return false;
    }

    let filter = doc! { "abi": &abi_string.trim().replace(" ", "") };

    let result = abi_collection
        .find_one(filter, None)
        .await
        .expect("Failed to execute find");

    // get the index from the result
    let found_index = match result {
        Some(document) => {
            document.index
        }
        None => {
            debug!("No document found. finding index on last document");

            let options = FindOneOptions::builder()
                .sort(doc! { "index": -1 })
                .build();

            let last_document_result = abi_collection
                .find_one(doc! {}, options)
                .await
                .expect("Failed to read last document");

            let last_returned_index = match last_document_result {
                Some(document) => {
                    debug!("Found last document with index: {:?}", document.index);

                    document.index + 1
                }
                None => {
                    0
                }
            };

            // save the abi
            save_abi_json(&abi_collection, &last_returned_index, &abi_string).await;

            last_returned_index
        }
    };

    debug!("Linking address {} with index {}", address, found_index);

    // Linking contract with abi
    let contract = ContractAbiCollection {
        timestamp: Utc::now().timestamp_millis(),
        address: address.clone(),
        index: found_index,
        flag: ContractAbiFlag::Verified,
    };

    contract_abi_collection
        .insert_one(contract, None)
        .await
        .expect("Failed to link the new contract");

    true
}
