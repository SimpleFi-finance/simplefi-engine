use super::types::ProtocolStatus;
use mongo_types::Mongo;
use mongodb::bson::doc;
use simplefi_engine_settings::load_settings;

pub async fn create_protocol_status(
    protocol_id: String,
    factory_address: String,
    chain_id: String,
    db: &Mongo,
) -> Result<ProtocolStatus, Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();
    let collection =
        db.collection::<ProtocolStatus>(&global_settings.protocol_status_gold_collection_name);

    let new_doc = ProtocolStatus {
        protocol_id,
        chain_id,
        factory_address,
        last_sync_block_timestamp: 0,
        should_update: false, // volumetric_fully_synced: false,
                              // volumetric_last_block_synced: 0,
                              // snapshot_fully_synced: false,
                              // snapshot_last_block_synced: 0,
    };

    let res = collection.insert_one(&new_doc, None).await;

    match res {
        Ok(_) => Ok(new_doc),
        _ => panic!("Error creating protocol status"),
    }
}

pub async fn updated_protocol_status(
    protocol_id: String,
    update: ProtocolStatus,
    db: &Mongo,
) -> Result<(), Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();
    let collection =
        db.collection::<ProtocolStatus>(&global_settings.protocol_status_gold_collection_name);

    let res = collection
        .find_one_and_replace(
            doc! {"protocol_id": protocol_id
            },
            update,
            None,
        )
        .await?;

    match res {
        Some(_) => Ok(()),
        _ => panic!("Error updating protocol status"),
    }
}
pub async fn updated_protocol_volumetric_synced_status(
    protocol_id: String,
    status: bool,
    db: &Mongo,
) -> Result<(), Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();
    let collection =
        db.collection::<ProtocolStatus>(&global_settings.protocol_status_gold_collection_name);

    let res = collection
        .find_one_and_update(
            doc! {"protocol_id": protocol_id
            },
            doc! {"volumetric_sync_status": status},
            None,
        )
        .await?;

    match res {
        Some(_) => Ok(()),
        _ => panic!("Error updating protocol status"),
    }
}
pub async fn updated_protocol_snapshot_synced_status(
    protocol_id: String,
    status: bool,
    db: &Mongo,
) -> Result<(), Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();
    let collection =
        db.collection::<ProtocolStatus>(&global_settings.protocol_status_gold_collection_name);

    let res = collection
        .find_one_and_update(
            doc! {"protocol_id": protocol_id
            },
            doc! {"snapshot_sync_status": status},
            None,
        )
        .await?;

    match res {
        Some(_) => Ok(()),
        _ => panic!("Error updating protocol status"),
    }
}
pub async fn updated_protocol_volumetric_last_block(
    protocol_id: &str,
    block: u64,
    db: &Mongo,
) -> Result<(), Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();
    let collection =
        db.collection::<ProtocolStatus>(&global_settings.protocol_status_gold_collection_name);

    let res = collection
        .find_one_and_update(
            doc! {"protocol_id": protocol_id
            },
            doc! {"volumetric_last_block_synced": block as f64},
            None,
        )
        .await?;

    match res {
        Some(_) => Ok(()),
        _ => panic!("Error updating protocol status"),
    }
}
pub async fn updated_protocol_snapshot_last_block(
    protocol_id: &str,
    block: u64,
    db: &Mongo,
) -> Result<(), Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();
    let collection =
        db.collection::<ProtocolStatus>(&global_settings.protocol_status_gold_collection_name);

    let res = collection
        .find_one_and_update(
            doc! {"protocol_id": protocol_id
            },
            doc! {"snapshot_last_block_synced": block as f64},
            None,
        )
        .await?;

    match res {
        Some(_) => Ok(()),
        _ => panic!("Error updating protocol status"),
    }
}
