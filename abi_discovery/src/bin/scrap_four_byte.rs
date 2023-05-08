use chrono::Utc;
use futures::StreamExt;
use log::{ info, warn, debug };
use mongodb::bson::doc;
use shared_types::mongo::abi::EventSignatureDocument;
use shared_utils::logger::init_logging;
use core::time;
use std::error::Error;

use abi_discovery::settings::load_settings;
use third_parties::{
    mongo::lib::abi_discovery::get_default_connection,
    http::four_byte::get_event_signatures
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logging();

    info!("Starting to scrap four byte event signatures process...");

    let mysettings = load_settings().expect("Failed to load settings");

    let mongo = get_default_connection(&mysettings.mongodb_uri.as_str(), &mysettings.mongodb_database_name.as_str()).await;

    let event_signature_collection = mongo.collection::<EventSignatureDocument>("event-signatures");

    info!("Mongo connected. Starting to scrap signatures...");

    let mut next_page: Option<String> = Some("https://www.4byte.directory/api/v1/event-signatures/?page=45".to_string());

    let timestamp = Utc::now().timestamp_millis() as u64;

    let mut retries = 0;

    loop {
        // try to call get_event_signatures 10 times and break the loop if it fails
        let signatures = get_event_signatures(next_page.clone()).await;

        if signatures.results.len() == 0 {
            if retries > 10 {
                info!("No more signatures to scrap...");
                break;
            } else {
                warn!("No more signatures to scrap. Sleeping for 10 seconds and retrying...");
                tokio::time::sleep(time::Duration::from_secs(10)).await;

                retries += 1;

                continue;
            }
        }

        retries = 0;

        let next = signatures.next.clone();

        debug!("signatures results : {:?}", signatures.results.len());

        next_page = signatures.next;

        debug!("next_page: {:?}", next_page);

        let mut event_signatures: Vec<EventSignatureDocument> = Vec::new();

        let results = signatures.results.clone();

        for result in results {
            let event_signature = EventSignatureDocument {
                timestamp,
                id: result.id,
                text_signature: result.text_signature,
                hex_signature: result.hex_signature,
            };

            event_signatures.push(event_signature);
        }

        debug!("event_signatures: {:?}", event_signatures.len());

        let ids = event_signatures.iter().map(|x| x.id.clone()).collect::<Vec<u32>>();

        debug!("ids: {:?}", ids.len());

        let mut ids_ind_db = Vec::new();

        let mut cursor = event_signature_collection.find(doc! { "id": { "$in": ids } }, None).await?;

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let id = document.id;
                    ids_ind_db.push(id);
                },
                Err(e) => {
                    warn!("Error while iterating over cursor: {:?}", e);
                }
            }
        }

        let event_signatures_to_insert = event_signatures.into_iter().filter(|x| !ids_ind_db.contains(&x.id)).collect::<Vec<EventSignatureDocument>>();

        debug!("event_signatures_to_insert to insert: {:?}", event_signatures_to_insert.len());

        if event_signatures_to_insert.len() > 0 {
            event_signature_collection.insert_many(&event_signatures_to_insert, None).await?;
        } else {
            info!("No more signatures to insert...");
        }

        if event_signatures_to_insert.len() < signatures.results.len() {
            info!("Found signatures already in the database. Breaking the loop...");

            break;
        } else {
            next_page = match next {
                Some(next) => Some(String::from(next)),
                None => break,
            };

            info!("Next page: {:?}", next_page);

            tokio::time::sleep(time::Duration::from_secs(2)).await;
        }
    }

    Ok(())
}
