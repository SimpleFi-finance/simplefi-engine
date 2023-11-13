use storage_provider::DatabaseProvider;

use crate::{ProcessId, Process};

pub struct LogsProcess;

impl Process for LogsProcess {
    fn id(&self) -> ProcessId {
        ProcessId::Logs
    }

    fn execute<T>(&mut self, db_provider: Option<&DatabaseProvider>) -> T {

        // load chain Rpc methods
        // get headers and return or store them
        unimplemented!()
    }
}