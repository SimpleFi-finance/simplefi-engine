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
use third_parties::mongo::lib::abi_discovery::get_default_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    info!("Starting generate_signatures process");

    let mysettings = load_settings().expect("Failed to load settings");

    let mongo = get_default_connection(&mysettings.mongodb_uri.as_str(), &mysettings.mongodb_database_name.as_str()).await;

    let abis_collection = mongo.database.collection::<AbiJSONCollection>(&mysettings.mongodb_abi_collection);
    let abi_signatures_collection = mongo.database.collection::<AbiEventDocument>(&mysettings.mongodb_abi_events_collection);

    let mut skip = 0;

    loop {
        let options = FindOptions::builder().limit(500).skip(skip).build();

        let cursor = abis_collection
            .find(None, options)
            .await
            .expect("Failed to execute find");

        let results = cursor.collect::<Vec<_>>().await;

        if results.is_empty() {
            info!("No more results");

            break;
        }

        for abi_collection_item in results {
            let abi_item = match abi_collection_item {
                Ok(item) => {
                    item
                }
                Err(e) => {
                    error!("Failed to create abi item: {:?}", e);

                    continue;
                }
            };

            let abi_event_result: Result<Vec<Value>, Error> = serde_json::from_str(&abi_item.abi);

            let abi_arr_definitions = match abi_event_result {
                Ok(result) => {
                    result
                }
                Err(e) => {
                    error!("Index: {}. Failed to parse abi: {:?}", abi_item.index, e);

                    continue;
                }
            };

            for abi_definition in abi_arr_definitions {
                if let Some(type_field) = abi_definition.get("type") {
                    match type_field.as_str().unwrap() {
                        "event" => {
                            debug!("Found event");

                            let index: u32 = abi_item.index;
                            let name = abi_definition.get("name").unwrap().as_str().unwrap().to_string();

                            // Generate a sorted map with abi event definition
                            let mut sorted_map: BTreeMap<String, Value> = BTreeMap::new();
                            let mut event_definition = abi_definition.clone();
                            let object = event_definition.as_object_mut().unwrap().clone();

                            for (key, value) in object.iter() {
                                sorted_map.insert(key.to_string(), value.to_owned());
                            }

                            let sorted_object: Map<String, Value> = sorted_map.into_iter()
                                .map(|(k, v)| (k, v))
                                .collect();

                            // Serialize the sorted map to a string
                            let sorted_json_string = serde_json::to_string(&sorted_object).unwrap();

                            debug!("Sorted event => {}", sorted_json_string);

                            // Serialize the sorted string to a json value
                            let event_definition = serde_json::from_str::<Event>(&sorted_json_string);

                            let event = match event_definition {
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
                        "function" => {
                            debug!("Found function");
                        }
                        "constructor" => {
                            debug!("Found constructor");
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
