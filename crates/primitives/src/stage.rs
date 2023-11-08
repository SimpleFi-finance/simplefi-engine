#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StageId {
    Waiting,
    Sync,
    BlockIndexing,
    Headers,
    SnapshotsIndexing,
    Finish,
}

impl StageId {
    pub const ALL: [StageId; 6] = [
        StageId::Waiting,
        StageId::Sync,
        StageId::BlockIndexing,
        StageId::Headers,
        StageId::SnapshotsIndexing,
        StageId::Finish,
    ];

    pub fn as_str(&self) -> &str {
        match self {
            StageId::Waiting => "Waiting",
            StageId::Sync => "Sync",
            StageId::BlockIndexing => "BlockIndexing",
            StageId::Headers => "Headers",
            StageId::SnapshotsIndexing => "SnapshotsIndexing",
            StageId::Finish => "Finish",
        }
    }

    pub fn is_finish(&self) -> bool {
        matches!(self, StageId::Finish)
    }

    pub fn is_downloading_stage(&self) -> bool {
        matches!(self, StageId::BlockIndexing | StageId::Sync)
    }
}

impl std::fmt::Display for StageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
