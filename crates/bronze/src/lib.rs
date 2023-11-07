#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum ProcessId {
    Headers,
    Transactions,
    Logs,
    Decoding,
    Indexing,
    Finish,
    Other(&'static str),
}

impl ProcessId {
    /// All supported Stages
    pub const ALL: [ProcessId; 6] = [
        ProcessId::Headers,
        ProcessId::Transactions,
        ProcessId::Logs,
        ProcessId::Decoding,
        ProcessId::Indexing,
        ProcessId::Finish,
    ];

    /// Return stage id formatted as string.
    pub fn as_str(&self) -> &str {
        match self {
            ProcessId::Headers => "Headers",
            ProcessId::Transactions => "Transactions",
            ProcessId::Logs => "Logs",
            ProcessId::Decoding => "Decoding",
            ProcessId::Indexing => "Indexing",
            ProcessId::Finish => "Finish",
            ProcessId::Other(s) => s,
        }
    }

    /// Returns true if it's a downloading process [ProcessId::Headers] or [ProcessId::Bodies]
    pub fn is_downloading_stage(&self) -> bool {
        matches!(self, ProcessId::Headers | ProcessId::Transactions | ProcessId::Logs)
    }

    /// Returns true indicating if it's the finish process [ProcessId::Finish]
    pub fn is_finish(&self) -> bool {
        matches!(self, ProcessId::Finish)
    }
}

impl std::fmt::Display for ProcessId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}