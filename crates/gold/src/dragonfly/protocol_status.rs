use crate::types::protocols::ProtocolStatus;
use async_trait::async_trait;
use simplefi_redis::{get_complete_hset, get_from_hset, store_in_hset, store_multiple_in_hset};

use super::dragonfly_driver::ProtocolDragonflyDriver;

#[async_trait]
pub trait ProtocolStatusTrait {
    fn resolve_protocol_status_hmap_name(&self) -> String;
    async fn get_protocol_status(
        &mut self,
        protocol_id: &str,
    ) -> Result<ProtocolStatus, Box<dyn std::error::Error>>;

    async fn get_all_protocols(
        &mut self
    ) -> Result<Vec<ProtocolStatus>, Box<dyn std::error::Error>>;

    async fn create_protocol_status(
        &mut self,
        protocol_id: String,
        factory_address: String,
        chain_id: String,
    ) -> Result<ProtocolStatus, Box<dyn std::error::Error>>;

    async fn update_protocol_status(
        &mut self,
        update: ProtocolStatus,
    ) -> Result<ProtocolStatus, Box<dyn std::error::Error>>;
    async fn update_many_protocol_status(
        &mut self,
        update: Vec<ProtocolStatus>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
impl ProtocolStatusTrait for ProtocolDragonflyDriver {
    fn resolve_protocol_status_hmap_name(&self) -> String {
        format!("{}_gold_status", &self.chain)
    }

    async fn get_protocol_status(
        &mut self,
        protocol_id: &str,
    ) -> Result<ProtocolStatus, Box<dyn std::error::Error>> {
        let hmap_name = self.resolve_protocol_status_hmap_name();
        let value = get_from_hset(&mut self.connection, &hmap_name, protocol_id).await?;

        let parsed: ProtocolStatus = serde_json::from_str(&value).unwrap();
        Ok(parsed)
    }
    async fn get_all_protocols(
        &mut self
    ) -> Result<Vec<ProtocolStatus>, Box<dyn std::error::Error>> {
        let hmap_name = self.resolve_protocol_status_hmap_name();
        let value = get_complete_hset(&mut self.connection, &hmap_name).await?;

        let parsed: Vec<ProtocolStatus> = value
            .iter()
            .map(|x| {
                let p: ProtocolStatus = serde_json::from_str(x).unwrap();
                p
            })
            .collect();
        Ok(parsed)
    }

    async fn create_protocol_status(
        &mut self,
        protocol_id: String,
        factory_address: String,
        chain_id: String,
    ) -> Result<ProtocolStatus, Box<dyn std::error::Error>> {
        let hmap_name = self.resolve_protocol_status_hmap_name();
        let new_doc = ProtocolStatus {
            protocol_id,
            chain_id,
            factory_address,
            last_sync_block_timestamp: 0,
            should_update: false,
        };

        let string_status = serde_json::to_string(&new_doc).unwrap();

        let _ = store_in_hset(
            &mut self.connection,
            &hmap_name,
            &new_doc.protocol_id,
            &string_status,
        )
        .await?;

        Ok(new_doc)
    }
    async fn update_protocol_status(
        &mut self,
        update: ProtocolStatus,
    ) -> Result<ProtocolStatus, Box<dyn std::error::Error>> {
        let hmap_name = self.resolve_protocol_status_hmap_name();

        let string_status = serde_json::to_string(&update).unwrap();

        let _ = store_in_hset(
            &mut self.connection,
            &hmap_name,
            &update.protocol_id,
            &string_status,
        )
        .await?;

        Ok(update)
    }
    async fn update_many_protocol_status(
        &mut self,
        updates: Vec<ProtocolStatus>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hmap_name = self.resolve_protocol_status_hmap_name();

        let mut to_save: Vec<(String, String)> = vec![];

        for update in updates {
            let string_status = serde_json::to_string(&update).unwrap();
            to_save.push((update.protocol_id.clone(), string_status.clone()))
        }

        let _ = store_multiple_in_hset(&mut self.connection, &hmap_name, to_save).await?;

        Ok(())
    }
}
