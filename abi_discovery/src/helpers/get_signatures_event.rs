use futures::StreamExt;
use log::{info, error};
use mongodb::{ Collection, bson::doc };

use crate::settings::load_settings;
use shared_types::mongo::abi::{AbiEvent, AbiEventDocument};
use third_parties::mongo::{MongoConfig, Mongo};

///
/// Function to get signatures event from mongo
///
/// # Arguments
///
/// * `signatures` - Vec<String> of signatures to get event for
///
/// # Returns
///
/// * `Result<Vec<AbiEvent>, Box<dyn std::error::Error>>` - Result of Vec<AbiEvent> or Error
///
/// # Example
///
/// ```
/// use abi_discovery::get_signatures_event;
///
/// let signatures = vec!["0x.."];
///
/// let result = get_signatures_event(signatures).await;
///
/// match result {
///   Ok(signatures_event) => info!("signatures_event: {:?}", signatures_event),
///  Err(e) => error!("error: {:?}", e),
/// }
/// ```
///
/// # Panics
///
/// This function will panic if the mongodb_uri is not set in the settings file
///
/// # Remarks
///
/// This function is used to get the event for a signature
///
///
pub async fn get_signatures_event(
    signatures: &Vec<String>,
) -> Result<Vec<AbiEvent>, Box<dyn std::error::Error>> {
    info!("get_signatures_event called");

    let mysettings = load_settings().expect("Failed to load settings");

    let mongo_uri = mysettings.mongodb_uri;
    let mongodb_database_name = mysettings.mongodb_database_name;
    let mongodb_abi_events_collection = mysettings.mongodb_abi_events_collection;

    let config = MongoConfig {
        uri: mongo_uri,
        database: mongodb_database_name,
    };

    let db = Mongo::new(&config).await?;

    let signatures_event_collection: Collection<AbiEventDocument> = db.collection(&mongodb_abi_events_collection);

    let query = doc! { "signature": { "$in": signatures }};

    let cursor = signatures_event_collection
            .find(query, None)
            .await
            .expect("Failed to execute find");

    let results = cursor.collect::<Vec<_>>().await;

    let mut signatures_event = Vec::new();

    if results.is_empty() {
        error!("No results found");

        return Ok(Vec::new());
    }

    for signature_found in results {
        let abi_item = match signature_found {
            Ok(abi_item) => abi_item,
            Err(e) => {
                error!("Error: {}", e);
                continue;
            }
        };

        let result = AbiEvent {
            timestamp: abi_item.timestamp,
            signature: abi_item.signature,
            event: abi_item.sorted,
            name: abi_item.name,
        };

        signatures_event.push(result);
    }

    Ok(signatures_event)
}
