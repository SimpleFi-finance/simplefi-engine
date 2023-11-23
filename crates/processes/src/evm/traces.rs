use simp_primitives::ChainSpec;
use storage_provider::DatabaseProvider;

use crate::{ProcessId, Process, ExecInput};

pub struct TracesProcess;

impl Process for TracesProcess {
    fn id(&self) -> ProcessId {
        ProcessId::Traces
    }
    #[allow(unused_variables)]
    fn execute<T>(&mut self, input: ExecInput, db_provider: Option<&DatabaseProvider>, chain: ChainSpec) -> Vec<T> {

        // load chain Rpc methods
        // get headers and return or store them
        unimplemented!()
    }
}