use std::collections::BTreeMap;

use chrono::Utc;
use ethabi::Event;
use futures::StreamExt;
use log::{debug, info, error, warn};
use mongodb::{bson::doc, options::FindOptions};
use serde_json::{self, Map, Value, Error};

use abi_discovery::settings::load_settings;
use shared_types::mongo::abi::{AbiJSONCollection, AbiEventDocument};
use shared_utils::logger::init_logging;
use third_parties::mongo::{Mongo, MongoConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    info!("Starting generate_signatures");

    let mysettings = load_settings().expect("Failed to load settings");

    let mongodb_uri = mysettings.mongodb_uri;
    let mongodb_abi_collection = mysettings.mongodb_abi_collection;
    let mongodb_abi_events_collection = mysettings.mongodb_abi_events_collection;

    let mongo_config = MongoConfig {
        uri: mongodb_uri,
        database: "abi_discovery_v10".to_string(),
    };

    let mongo = Mongo::new(&mongo_config)
        .await
        .expect("Failed to create mongo Client");
    let abis_collection = mongo.database.collection::<AbiJSONCollection>(&mongodb_abi_collection);
    let abi_signatures_collection = mongo.database.collection::<AbiEventDocument>(&mongodb_abi_events_collection);

    let mut skip = 0;

    loop {
        let options = FindOptions::builder().limit(500).skip(skip).build();

        let cursor = abis_collection
            .find(None, options)
            .await
            .expect("Failed to execute find");
        let results = cursor.collect::<Vec<_>>().await;

        if results.is_empty() {
            break;
        }

        for abi_collection_item in results {
            let abi_item = abi_collection_item.expect("Failed to get next from server");

            let abi = abi_item.abi;

            let abi_definitions_results: Result<Vec<Value>, Error> = serde_json::from_str(&abi);

            // check if abi is valid
            let abi_definitions = match abi_definitions_results {
                Ok(result) => {
                    result
                }
                Err(e) => {
                    error!("Index: {}. Failed to parse abi: {:?}", abi_item.index, e);
                    continue;
                }
            };

            // Iterate over each ABI definition and generate a signature
            for abi_definition in abi_definitions {
                if let Some(type_field) = abi_definition.get("type") {
                    match type_field.as_str().unwrap() {
                        "function" => {

                            debug!("Found function");
                        }
                        "constructor" => {

                            debug!("Found constructor");
                        }
                        "event" => {
                            debug!("Found event");

                            let index: u32 = abi_item.index;
                            let name = abi_definition.get("name").unwrap().as_str().unwrap().to_string();

                            let mut event_definition = abi_definition.clone();

                            let object = event_definition.as_object_mut().unwrap().clone();

                            let mut sorted_map: BTreeMap<String, Value> = BTreeMap::new();

                            for (key, value) in object.iter() {
                                sorted_map.insert(key.to_string(), value.to_owned());
                            }

                            let sorted_object: Map<String, Value> = sorted_map.into_iter()
                                .map(|(k, v)| (k, v))
                                .collect();

                            let sorted_json_string = serde_json::to_string(&sorted_object).unwrap();

                            debug!("{}", sorted_json_string);

                            let sorted_json_value = serde_json::from_str::<Value>(&sorted_json_string).unwrap();

                            let event_result = serde_json::from_value::<Event>(sorted_json_value);

                            let event = match event_result {
                                Ok(result) => {
                                    result
                                }
                                Err(e) => {
                                    error!("Index {}. Failed to parse event: {:?}", index, e);
                                    continue;
                                }
                            };

                            let event_signature = event.signature();

                            debug!("Event signature: {}", event_signature);

                            let hex_string = format!("0x{}", hex::encode(&event_signature));

                            debug!("Event signature hex: {}", hex_string);

                            let query = doc! {
                                "signature": hex_string.clone()
                            };

                            let count = abi_signatures_collection.count_documents(query, None).await.unwrap();

                            if count > 0 {
                                warn!("index {}. Found existing signature. checking sorted", index);

                                let query = doc! {
                                    "sorted": sorted_json_string.clone()
                                };

                                let count = abi_signatures_collection.count_documents(query, None).await.unwrap();

                                if count > 0 {
                                    warn!("index {}. Found existing signature and sorted. skipping", index);

                                    continue;
                                }
                            }

                            let abi_signature = AbiEventDocument {
                                timestamp: Utc::now().timestamp_millis() as u64,
                                index,
                                name,
                                signature: hex_string,
                                sorted: sorted_json_string,
                            };

                            abi_signatures_collection.insert_one(abi_signature, None).await.unwrap();

                        }
                        "fallback" => {
                            // Generate a signature for a fallback
                            debug!("Found fallback");
                        }
                        _ => debug!("Found unknown type"),
                    }
                }
            }




        }

        warn!("Processed abis. skip {}", skip);

        skip += 500;

    }

    info!("Finished generate_signatures");

    Ok(())
}
