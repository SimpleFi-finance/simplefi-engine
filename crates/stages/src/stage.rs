
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StageId {
    Waiting,
    Sync,
    BlockIndexing,
    SnapshotsIndexing,
    Finish,
}

impl StageId {
    pub const ALL: [StageId; 5] = [
        StageId::Waiting,
        StageId::Sync,
        StageId::BlockIndexing,
        StageId::SnapshotsIndexing,
        StageId::Finish,
    ];

    pub fn as_str(&self) -> &str {
        match self {
            StageId::Waiting => "Waiting",
            StageId::Sync => "Sync",
            StageId::BlockIndexing => "BlockIndexing",
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

#[async_trait::async_trait]
pub trait Stage: Send + Sync {
    fn id(&self) -> StageId;

    async fn execute(&mut self) -> ();
}


pub(crate) type BoxedStage = Box<dyn Stage>;