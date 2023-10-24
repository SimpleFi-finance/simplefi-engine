use interfaces::Result;
use primitives::Address;

#[auto_impl::auto_impl(&, Arc, Box)]
pub trait TrackingProvider: Send + Sync {
    fn is_contract_tracked(&self, address: Address) -> Result<bool>;
}

#[auto_impl::auto_impl(&, Arc, Box)]
pub trait TrackingWriter: Send + Sync {
    fn insert_tracked_contract(&self, address: Address) -> Result<()>;
}