use storage_provider::DatabaseProvider;

use crate::{ProcessId, Process};

pub struct HeaderProcess;

impl Process for HeaderProcess {
    fn id(&self) -> ProcessId {
        ProcessId::Headers
    }

    fn execute<T>(&mut self, db_provider: Option<&DatabaseProvider>) -> T {

        // load chain Rpc methods
        // get headers and return or store them
        unimplemented!()
    }
}