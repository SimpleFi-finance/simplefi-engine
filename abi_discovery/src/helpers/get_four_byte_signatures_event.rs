use futures::StreamExt;
use log::{info, error};
use mongodb::{ Collection, bson::doc };

use crate::settings::load_settings;
use shared_types::mongo::abi::EventSignatureDocument;
use third_parties::mongo::lib::abi_discovery::get_default_connection;


pub async fn get_four_byte_signatures_event(
    signatures: &Vec<String>,
) -> Result<Vec<EventSignatureDocument>, Box<dyn std::error::Error>> {
    info!("get_four_byte_signatures_event called");

    let mysettings = load_settings().expect("Failed to load settings");

    let mongo = get_default_connection(&mysettings.mongodb_uri.as_str(), &mysettings.mongodb_database_name.as_str()).await;

    let signatures_event_collection: Collection<EventSignatureDocument> = mongo.collection("event-signatures");

    let query = doc! { "hex_signature": { "$in": signatures }};

    let cursor = signatures_event_collection
            .find(query, None)
            .await
            .expect("Failed to execute find");

    let results = cursor.collect::<Vec<_>>().await;

    let mut signatures_event: Vec<EventSignatureDocument> = Vec::new();

    if results.is_empty() {
        error!("No results found");

        return Ok(Vec::new());
    }

    for result in results {
        let signature_event = match result {
            Ok(document) => document,
            Err(e) => {
                error!("Error while getting result: {:?}", e);

                continue;
            }
        };

        signatures_event.push(signature_event);
    }

    Ok(signatures_event)
}
