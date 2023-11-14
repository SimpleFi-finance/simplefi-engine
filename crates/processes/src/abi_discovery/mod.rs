use simp_primitives::ChainSpec;
use storage_provider::DatabaseProvider;

use crate::{ProcessId, Process, ExecInput};

pub struct AbiDiscoveryProcess;

impl Process for AbiDiscoveryProcess {
    fn id(&self) -> ProcessId {
        ProcessId::AbiDiscovery
    }

    fn execute<T>(&mut self, input: ExecInput, db_provider: Option<&DatabaseProvider>, chain: ChainSpec) -> Vec<T> {
        unimplemented!()
    }
}