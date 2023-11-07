use simp_primitives::{ProcessId, Process};

pub struct AbiDiscoveryProcess;

impl Process for AbiDiscoveryProcess {
    fn id(&self) -> ProcessId {
        ProcessId::AbiDiscovery
    }

    fn execute<T>(self: Box<Self>) -> T {
        unimplemented!()
    }
}