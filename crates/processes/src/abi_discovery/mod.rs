use storage_provider::DatabaseProvider;

use crate::{ProcessId, Process};

pub struct AbiDiscoveryProcess;

impl Process for AbiDiscoveryProcess {
    fn id(&self) -> ProcessId {
        ProcessId::AbiDiscovery
    }

    fn execute<T>(&mut self, db_provider: Option<&DatabaseProvider>) -> T {
        unimplemented!()
    }
}