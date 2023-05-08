use bson::{doc };
use chrono::Utc;
use futures::stream::StreamExt;
use mongodb::options::FindOneOptions;
use tokio;
use std::fs::read_dir;


use shared_types::mongo::abi::{ ContractAbiCollection, ContractAbiFlag, PartialIndexDoc, AbiJSONCollection, FactoryContractsCollection };
use solidity::default_abis::{get_factory_abis, get_default_market_abis, get_factory_market_index};
use third_parties::mongo::{Mongo, MongoConfig};

// create main function
#[tokio::main]
async fn main() {
    // Create a mongo connection with Mongo helper from shared_types
    let mongo_config = MongoConfig {
        uri: "mongodb://localhost:27017".to_string(),
        database: "abi_discovery_v5".to_string(),
    };

    // Create a new MongoDB client
    let mongo = Mongo::new(&mongo_config).await.unwrap();

    let abi_collection = mongo.database.collection::<AbiJSONCollection>("abis");
    let contract_abi_collection = mongo.database.collection::<ContractAbiCollection>("contract-abi");
    let factory_contracts_collection = mongo.database.collection::<FactoryContractsCollection>("factory-contracts");

    // Before starting the scraping, we are going to add the default abis to the database
    // We are going to add the default abis to the database
    let default_factory_abis =  get_factory_abis().await;
    let default_market_abis = get_default_market_abis().await;

    let timestamp = Utc::now().timestamp_millis() as i64;

    for (index, value) in default_factory_abis {
        abi_collection
            .insert_one(
                AbiJSONCollection {
                    timestamp,
                    index,
                    abi: value.abi,
                },
                None,
            )
            .await
            .expect("Failed to insert document in abi collection");

        contract_abi_collection.insert_one(
            ContractAbiCollection {
                timestamp,
                index,
                address: value.address.to_lowercase(),
                flag: ContractAbiFlag::Verified,
            }, None)
            .await
            .expect("Failed to insert document in contract-abi collection");
    }

    // Lets save the default market abis
    for (index, abi) in default_market_abis {
        abi_collection
            .insert_one(
                AbiJSONCollection {
                    timestamp,
                    index,
                    abi,
                },
                None,
            )
            .await
            .expect("Failed to insert document in abi collection");
    }

    let mut skip = 0;

    loop {
        let options= mongodb::options::FindOptions::builder()
        .limit(500)
        .skip(skip)
        .build();

        // options.clone().skip(skip);
        let cursor = factory_contracts_collection.find(None, options).await.expect("Failed to execute find");
        let results = cursor.collect::<Vec<_>>().await;

        if results.is_empty() {
            break;
        }

        let mut contract_indexed: Vec<ContractAbiCollection> = Vec::new();

        for result in results {
            let result = result.expect("Failed to get next from server");

            let factory_address = result.factory_address;
            let address = result.address;

            let index = get_factory_market_index(&factory_address);

            if index == 0 {
                print!("Found not tracked factory_address: {}", factory_address);

                continue;
            }

            contract_indexed.push(ContractAbiCollection {
                timestamp,
                address: address.to_lowercase(),
                index,
                flag: ContractAbiFlag::Verified,
            });
        }

        println!("Found {} contract indexes. Skipping {}", contract_indexed.len(), skip);

        contract_abi_collection.insert_many(contract_indexed, None).await.expect("Failed to insert many cotract_indexes");

        skip += 500;
    }

    let starting_index = 100;

    // Let's start the folder scraping
    // We need to get the list of folders in a directory
    let dir = read_dir("E:\\solidity\\QmdDJpWUxK3abZ2NMxdAGNTsMQZRJCuAWtXVCeakac4zZr")
        .expect("Failed to read directory");

    // Iterate over the directory entries
    for entry in dir {
        // Get the entry path
        let path = entry.unwrap().path();

        // Check if the path is a directory
        if path.is_dir() {

            // Get a list of files in the directory path
            let files = read_dir(path).expect("Failed to read directory");

            // Iterate over the files
            for file in files {
                // Get the file path
                let file_path = file.unwrap().path();

                // Check if the file path is a file
                if file_path.is_file() {

                    // GEt the folder name from file_path
                    let folder_name = file_path
                        .parent()
                        .unwrap()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap();

                    // println!("Folder_name: {:?}", folder_name);

                    if file_path.file_name().unwrap() == "metadata.json" {
                        // println!("Found metadata.json file");

                        // check if already exist contract with the folder name in contract-abi collection
                        let filter = doc! { "address": folder_name.to_lowercase() };

                        let result = contract_abi_collection
                            .find_one(filter, None)
                            .await
                            .unwrap();

                        match result {
                            Some(_) => {
                                // println!("Found a document: {:?}", document);

                                continue;
                            }
                            None => {
                                // println!("Contract not tracked. Saving ABI or getting index from ABI collection");
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

                        let filter = doc! { "abi": &abi_string };

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

                                        let mut new_index = index + 1;

                                        if index == 51 {
                                            println!("Found index 51. Pushing to 100");

                                            new_index = starting_index;
                                        }

                                        abi_collection
                                            .insert_one(
                                                AbiJSONCollection {
                                                    timestamp,
                                                    index: new_index,
                                                    abi: abi_string,
                                                },
                                                None,
                                            )
                                            .await
                                            .expect("Failed to insert document");

                                        contract_abi_collection
                                            .insert_one(
                                                ContractAbiCollection {
                                                    timestamp,
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
                                        panic!("Error. no documents found!");

                                        /* abi_collection
                                            .insert_one(
                                                AbiJSONCollection {
                                                    timestamp: Utc::now().timestamp_millis() as i64,
                                                    index: 100,
                                                    abi: abi_string,
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
                                            .expect("Failed to insert document"); */
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

}
