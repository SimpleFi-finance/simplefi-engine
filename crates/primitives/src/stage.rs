#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StageId {
    // waits for a block to be minted (or confirmed) and saves header
    // in update mode its the latest block confirmed
    // in sync mode its the next block in checkpoint until target
    Headers,
    // index the block
    BlockIndexing,
    // updated the market snapshots
    SnapshotsIndexing,
    // finish loop
    Finish,
    Other(&'static str)
}

impl StageId {
    pub const ALL: [StageId; 4] = [
        StageId::Headers,
        StageId::BlockIndexing,
        StageId::SnapshotsIndexing,
        StageId::Finish,
    ];

    pub fn as_str(&self) -> &str {
        match self {
            StageId::Headers => "Headers",
            StageId::BlockIndexing => "BlockIndexing",
            StageId::SnapshotsIndexing => "SnapshotsIndexing",
            StageId::Finish => "Finish",
            StageId::Other(name) => name,
        }
    }

    pub fn is_finish(&self) -> bool {
        matches!(self, StageId::Finish)
    }

    pub fn is_downloading_stage(&self) -> bool {
        matches!(self, StageId::BlockIndexing | StageId::Headers)
    }
}

impl std::fmt::Display for StageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
