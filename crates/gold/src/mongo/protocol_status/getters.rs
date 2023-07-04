use mongo_types::Mongo;
use mongodb::bson::doc;
use simplefi_engine_settings::load_settings;

use super::types::ProtocolStatus;

pub async fn get_protocol_status(
    protocol_id: &str,
    db: &Mongo,
) -> Result<ProtocolStatus, Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();

    let collection =
        db.collection::<ProtocolStatus>(&global_settings.protocol_status_gold_collection_name);

    let res = collection
        .find_one(doc! { "protocol_id": protocol_id }, None)
        .await?;

    match res {
        Some(x) => Ok(x),
        _ => panic!("protocol_status not found for id {}", protocol_id),
    }
}
