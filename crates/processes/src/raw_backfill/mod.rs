use simp_primitives::{ProcessId, Process};

pub struct HeaderProcess;

impl Process for HeaderProcess {
    fn id(&self) -> ProcessId {
        ProcessId::Headers
    }

    fn execute<T>(self: Box<Self>) -> T {
        unimplemented!()
    }
}

pub struct TransactionsProcess;

impl Process for TransactionsProcess {
    fn id(&self) -> ProcessId {
        ProcessId::Transactions
    }

    fn execute<T>(self: Box<Self>) -> T {
        unimplemented!()
    }
}

pub struct LogsProcess;

impl Process for LogsProcess {
    fn id(&self) -> ProcessId {
        ProcessId::Logs
    }

    fn execute<T>(self: Box<Self>) -> T {
        unimplemented!()
    }
}

pub struct TracesProcess;

impl Process for TracesProcess {
    fn id(&self) -> ProcessId {
        ProcessId::Traces
    }

    fn execute<T>(self: Box<Self>) -> T {
        unimplemented!()
    }
}

pub struct DecodingProcess;

impl Process for DecodingProcess {
    fn id(&self) -> ProcessId {
        ProcessId::Decoding
    }

    fn execute<T>(self: Box<Self>) -> T {
        unimplemented!()
    }
}