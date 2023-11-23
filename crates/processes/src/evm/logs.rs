use simp_primitives::ChainSpec;
use storage_provider::DatabaseProvider;

use crate::{ExecInput, Process, ProcessId};

pub struct LogsProcess;

impl Process for LogsProcess {
    fn id(&self) -> ProcessId {
        ProcessId::Logs
    }
    #[allow(unused_variables)]
    fn execute<T>(
        &mut self,
        input: ExecInput,
        db_provider: Option<&DatabaseProvider>,
        chain: ChainSpec,
    ) -> Vec<T> {
        // load chain Rpc methods
        // get headers and return or store them
        unimplemented!()
    }
}
