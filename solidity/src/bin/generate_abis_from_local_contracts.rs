use bson::{doc, Binary};
use log::{ info, debug };
use mongodb::{ options::FindOneOptions };
use shared_types::abi::Abi;
use shared_types::mongo::abi::{AbiCollection, ContractAbiCollection, ContractAbiFlag, PartialIndexDoc };
use std::fs::read_dir;
use third_parties::mongo::{Mongo, MongoConfig};
use tokio;
use chrono::Utc;

// create main function
#[tokio::main]
async fn main() {
    info!("Starting to load contract indexes from MongoDB to Redis Set (verify_addresses) ...");

    // We need to get the list of folders in a directory
    let dir = read_dir("E:\\solidity\\QmdDJpWUxK3abZ2NMxdAGNTsMQZRJCuAWtXVCeakac4zZr")
        .expect("Failed to read directory");

    // Create a mongo connection with Mongo helper from shared_types
    let mongo_config = MongoConfig {
        uri: "mongodb://localhost:27017".to_string(),
        database: "abi_discovery_json".to_string(),
    };

    debug!("mongoConfig: {:?}", mongo_config);

    // Create a new MongoDB client
    let mongo = Mongo::new(&mongo_config).await.unwrap();

    let abi_collection = mongo.database.collection::<AbiCollection>("abis");
    let contract_abi_collection = mongo
        .database
        .collection::<ContractAbiCollection>("contract-abi");

    // Iterate over the directory entries
    for entry in dir {
        // Get the entry path
        let path = entry.unwrap().path();

        // Check if the path is a directory
        if path.is_dir() {
            // Print the path
            debug!("PATH => {:?}", path);

            // Get a list of files in the directory path
            let files = read_dir(path).expect("Failed to read directory");

            // Iterate over the files
            for file in files {
                // Get the file path
                let file_path = file.unwrap().path();

                // Check if the file path is a file
                if file_path.is_file() {
                    // Print the file path
                    debug!("FILE => {:?}", file_path);

                    // GEt the folder name from file_path
                    let folder_name = file_path
                        .parent()
                        .unwrap()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap();

                    debug!("Folder_name: {:?}", folder_name);

                    if file_path.file_name().unwrap() == "metadata.json" {
                        debug!("Found metadata.json file");

                        // check if already exist contract with the folder name in contract-abi collection
                        let filter = doc! { "address": folder_name.to_lowercase() };

                        let result = contract_abi_collection
                            .find_one(filter, None)
                            .await
                            .unwrap();

                        match result {
                            Some(_) => {
                                debug!("Found a document: {:?}", result);

                                continue;
                            }
                            None => {
                                debug!("Contract not tracked. Saving ABI or getting index from ABI collection");
                            }
                        }

                        // With serde_json, we can parse the metadata.json file
                        let metadata = std::fs::read_to_string(&file_path)
                            .expect("Failed to read metadata.json");

                        let metadata_json = serde_json::from_str::<serde_json::Value>(&metadata)
                            .expect("Failed to parse metadata as JSON");

                        let output = metadata_json
                            .get("output")
                            .expect("No output property with ABI definition in metadata.json");

                        // Generate ABI struct from output.abi property
                        let abi_string = output
                            .get("abi")
                            .expect("No ABI definition in metadata.json")
                            .to_string();



                        let mut abi: Vec<Abi> = serde_json::from_str(&abi_string).unwrap();

                        debug!("abi length: {:?}", abi.len());

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

                        match result {
                            Some(document) => {
                                println!("Found a repeated ABI for document: {:?}", &folder_name);

                                // get index property from document
                                let index = document.index;

                                // insert document in contract-abi collection
                                contract_abi_collection
                                    .insert_one(
                                        ContractAbiCollection {
                                            timestamp: Utc::now().timestamp_millis() as i64,
                                            address: folder_name.to_lowercase(),
                                            index,
                                            flag: ContractAbiFlag::Verified,
                                        },
                                        None,
                                    )
                                    .await
                                    .expect("Failed to insert document");
                            }
                            None => {
                                // println!("No document found. finding index on last document");

                                let projection = doc! { "timestamp": "1", "index": 1, "abi": 1, "_id": 0 };
                                let sort = doc! { "_id": -1 };
                                let find_options = FindOneOptions::builder()
                                    .projection(projection)
                                    .sort(sort)
                                    .build();

                                let lastest_document = abi_collection.clone_with_type::<PartialIndexDoc>().find_one(None, find_options).await.expect("Failed to execute find");

                                match lastest_document {
                                    Some(document) => {
                                        // get index property from document
                                        let index = document.index;

                                        // print!("Latest index: {:?}", index);

                                        let new_index = index + 1;

                                        abi_collection
                                            .insert_one(
                                                AbiCollection {
                                                    timestamp: Utc::now().timestamp_millis() as i64,
                                                    index: new_index,
                                                    abi: abi_binary,
                                                },
                                                None,
                                            )
                                            .await
                                            .expect("Failed to insert document");

                                        contract_abi_collection
                                            .insert_one(
                                                ContractAbiCollection {
                                                    timestamp: Utc::now().timestamp_millis() as i64,
                                                    address: folder_name.to_lowercase(),
                                                    index: new_index,
                                                    flag: ContractAbiFlag::Verified,
                                                },
                                                None,
                                            )
                                            .await
                                            .expect("Failed to insert document");
                                    }
                                    None => {
                                        // println!("No document found. First insert!");

                                        abi_collection
                                            .insert_one(
                                                AbiCollection {
                                                    timestamp: Utc::now().timestamp_millis() as i64,
                                                    index: 0,
                                                    abi: abi_binary,
                                                },
                                                None,
                                            )
                                            .await
                                            .expect("Failed to insert document");

                                        contract_abi_collection
                                            .insert_one(
                                                ContractAbiCollection {
                                                    timestamp: Utc::now().timestamp_millis() as i64,
                                                    address: folder_name.to_lowercase(),
                                                    index: 0,
                                                    flag: ContractAbiFlag::Verified,
                                                },
                                                None,
                                            )
                                            .await
                                            .expect("Failed to insert document");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /* mongo.close().await.unwrap(); */
}
