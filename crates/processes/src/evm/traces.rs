use storage_provider::DatabaseProvider;

use crate::{ProcessId, Process};

pub struct TracesProcess;

impl Process for TracesProcess {
    fn id(&self) -> ProcessId {
        ProcessId::Traces
    }

    fn execute<T>(&mut self, db_provider: Option<&DatabaseProvider>) -> T {

        // load chain Rpc methods
        // get headers and return or store them
        unimplemented!()
    }
}