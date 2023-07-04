use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProtocolStatus {
    pub protocol_id: String,
    pub factory_address: String,
    pub volumetric_fully_synced: bool,
    pub volumetric_last_block_synced: i64,
    pub snapshot_fully_synced: bool,
    pub snapshot_last_block_synced: i64,
}
