use interfaces::Result;
use primitives::{Protocol,H256};

/// Client trait for [Protocol]s
pub trait ProtocolProvider: Send + Sync {

    /// Creates a new protocol with a blank status
    fn create_protocol (&self, factory_address: H256) -> Result<()>;
    // fn create_protocol (&self, protocol: Protocol) -> Result<()>;

    /// deletes an existing protocol
    fn delete_protocol (&self, protocol_id: u64) -> Result<()>;

    /// retrieves one protocol
    fn get_protocol (&self, protocol_id: u64) -> Result<Option<Protocol>>;

    /// retrieves all protocols
    fn get_all_protocols (&self) -> Result<Vec<Protocol>>;

    /// retrieves all protocols where the should_update property vof the status is set to true
    fn get_all_synced_protocols (&self) -> Result<Vec<Protocol>>;

    /// updates an existing protocol.  If the protocol_id does not exist, an Error result will occur.
    fn update_protocol (&self, updated_protocol: Protocol, protocol_id: u64) -> Result<bool>;
}