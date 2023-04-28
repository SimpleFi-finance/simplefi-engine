use bson::Bson;
use mongodb::{bson::doc, options::AggregateOptions};
use futures::StreamExt;
#[allow(unused)]
use log::{debug, info, error};

use abi_discovery::settings::load_settings;
use shared_types::mongo::abi::{ContractAbiCollection, AbiJSONCollection};
use shared_utils::logger::init_logging;
#[allow(unused)]
use third_parties::{redis::connect, mongo::{MongoConfig, Mongo}, broker::{publish_rmq_message, create_rmq_channel}};



#[allow(unused)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let mysettings = load_settings().expect("Failed to load settings");

    let mongodb_uri = mysettings.mongodb_uri;
    let mongodb_database_name = mysettings.mongodb_database_name;
    let rabbit_uri = mysettings.rabbit_mq_url.to_string();
    let queue_name = mysettings.rabbit_exchange_name.to_string();
    let exchange_name = format!("{}_exchange", queue_name);
    let routing_key = String::from("abi_discovery");
    let etherscan_keys = mysettings.etherscan_api_keys;
    let redis_uri = mysettings.redis_uri.to_string();

    let channel = create_rmq_channel("amqp://localhost:5672/%2f")
        .await
        .expect("Failed to create channel");

    let mut con = connect(redis_uri.as_str()).await.unwrap();

    let mongo_config = MongoConfig {
        uri: mongodb_uri,
        database: "abi_discovery_v10".to_string(),
    };

    let mongo = Mongo::new(&mongo_config).await.expect("Failed to create mongo Client");
    let contracts_collection = mongo.database.collection::<ContractAbiCollection>("contract-abi");
    let abis_collection = mongo.database.collection::<AbiJSONCollection>("abis");

    // let mut clean_abis: Vec<String> = Vec::new();
    let mut clean_indexes: Vec<i32> = Vec::new();
    let mut publish_contracts: Vec<String> = Vec::new();

    // Run mongo aggregation to get the list of abi with repeated index property
    let pipeline = vec![
        doc! {
            "$group": {
                "_id": "$index",
                "count": { "$sum": 1 },
                "docs": { "$push": "$$ROOT" }
            }
        },
        doc! {
            "$match": {
                "count": { "$gt": 1 }
            }
        },
        doc! {
            "$unwind": "$docs"
        },
        doc! {
            "$replaceRoot": { "newRoot": "$docs" }
        },
    ];

    let mut options = AggregateOptions::default();
    options.allow_disk_use = Some(true);


    let cursor = abis_collection.aggregate(pipeline, options).await.expect("Failed to execute aggregate");
    let results = cursor.collect::<Vec<_>>().await;

    debug!("Found {} abis with repeated index", results.len());

    // loop results and find all contracts with that index
    for result in results {
        match result {
            Ok(document) => {
                if let Some(index) = document.get("index").and_then(Bson::as_i32) {
                    if !clean_indexes.contains(&index) {
                        clean_indexes.push(index);
                    }

                } else {
                    println!("no propertyField found");
                }
            }

            Err(error_msg) => {
                error!("Error: {}", error_msg);
            }
        }
    }

    debug!("Found {} indexes with repeated index", clean_indexes.len());
    debug!("Indexes {:?} ", clean_indexes);

    // for each index, get all contracts with that index
    for index in &clean_indexes {
        let query = doc! {
            "index": index
        };

        // get all addresses from contracts collection with this index
        let cursor = contracts_collection.find(query, None).await.expect("Failed to execute find");
        let contracts_results = cursor.collect::<Vec<_>>().await;

        debug!("Found {} contracts with index {}", contracts_results.len(), index);

        // get all contract addresses from contracts_results
        for contract_result in contracts_results {
            match contract_result {
                Ok(contract) => {
                    let address = contract.address;

                    if !publish_contracts.contains(&address) {
                        publish_contracts.push(address);
                    }

                    /* publish_contracts.push(address); */
                }
                Err(error_msg) => {
                    error!("Error: {}", error_msg);
                }
            }
        }
    }

    debug!("Found {} contracts to publish", publish_contracts.len());
/*
    // for each contract, publish it to rabbit
    for address in publish_contracts {
        publish_rmq_message(&exchange_name, &routing_key, &address.as_bytes(), &channel)
            .await
            .expect("Failed to publish message");
    }

    // for each index, delete all contracts with that index and all abis with that index
    for index in clean_indexes {
        // delete all contracts with that index
        let query = doc! {
            "index": index
        };

        contracts_collection.delete_many(query, None).await.expect("Failed to delete contracts");

        // delete all abis with that index
        let query = doc! {
            "index": index
        };

        abis_collection.delete_many(query, None).await.expect("Failed to delete abis");
    }
 */



    /* cursor.into_iter().for_each(|result| {
        match result {
            Ok(document) => {
                if let Some(property_field) = document.get("propertyField").and_then(Bson::as_str) {
                    println!("{}", property_field);
                } else {
                    println!("no propertyField found");
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    });
 */
    /* let cursor = contracts_collection.aggregate(vec![query, matches], None).await.expect("Failed to execute aggregate");
    let results = cursor.collect::<Vec<_>>().await;

    debug!("Found {} abis with repeated index", results.len()); */


    /* // First we are going to get all abis which len is lower than 10
    let query = doc! {
        "$expr": {
            "$lt": [
                { "$strLenCP": "$abi" },
                10
            ]
        }
    };

    let cursor = abis_collection.find(query, None).await.expect("Failed to execute find");
    let results = cursor.collect::<Vec<_>>().await;

    debug!("Found {} abis with len < 10", results.len());

    let mut clean_abis: Vec<String> = Vec::new();
    let mut publish_contracts: Vec<String> = Vec::new();

    // loop results
    for result in results {
        match result {
            Ok(abi) => {
                let index = abi.index;

                // log debug abi _id
                clean_abis.push(abi.abi.clone());

                let query = doc! {
                    "index": index
                };

                // get all addresses from contracts collection with this index
                let cursor = contracts_collection.find(query, None).await.expect("Failed to execute find");
                let contracts_results = cursor.collect::<Vec<_>>().await;

                debug!("Found {} contracts with index {}", contracts_results.len(), index);

                // get all contract addresses from contracts_results
                for contract_result in contracts_results {
                    match contract_result {
                        Ok(contract) => {
                            let address = contract.address;

                            publish_contracts.push(address);
                        }
                        Err(error_msg) => {
                            error!("Error: {}", error_msg);
                        }
                    }
                }

            }
            Err(error_msg) => {
                error!("Error: {}", error_msg);
            }
        }
    }

    // print length of publish_contracts
    debug!("Found {} contracts to publish", publish_contracts.len());
    debug!("Found {} abis to delete", clean_abis.len()); */

    // loop publish_contracts
    /* for address in publish_contracts {
        publish_rmq_message(&exchange_name, &routing_key, &address.as_bytes(), &channel)
            .await
            .expect("Failed to publish message");
    } */

    // for each contract in publich_contracts, display the amount of documents with that address
    /* for address in publish_contracts {
        let query = doc! {
            "address": &address
        };

        let cursor = contracts_collection.find(query, None).await.expect("Failed to execute find");
        let contracts_results = cursor.collect::<Vec<_>>().await;

        debug!("Found {} contracts with address {}", contracts_results.len(), address);
    } */

    // for each abi in clean_abis, delete it from abis_collection
    /* for address in publish_contracts {
        let query = doc! {
            "address": &address
        };

        contracts_collection.delete_one(query, None).await.expect("Failed to delete contract abi");
    }

    for abi in clean_abis {
        let query = doc! {
            "abi": abi
        };

        abis_collection.delete_many(query, None).await.expect("Failed to delete abi");
    } */

    Ok(())

}
