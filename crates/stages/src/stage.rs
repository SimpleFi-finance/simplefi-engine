use simp_primitives::{StageId, BlockNumber, ChainSpec};

use crate::error::StageError;
use storage_provider::DatabaseProvider;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct ExecInput {
    /// The target block number the stage needs to execute towards.
    pub target: Option<BlockNumber>,
    /// The checkpoint of this stage the last time it was executed.
    pub checkpoint: Option<BlockNumber>,
}

impl ExecInput {
    pub fn target(&self) -> BlockNumber {
        self.target.unwrap_or_default()
    }

    pub fn checkpoint(&self) -> BlockNumber {
        self.checkpoint.unwrap_or_default()
    }

    pub fn next_block(&self) -> BlockNumber {
        let current_block = self.checkpoint();
        current_block + 1
    }

    pub fn target_reached(&self) -> bool {
        self.checkpoint() >= self.target()
    }

    pub fn is_first_range(&self) -> bool {
        self.checkpoint.is_none()
    }
}
/// The output of a stage execution.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExecOutput {
    /// How far the stage got.
    pub checkpoint: BlockNumber,
    /// Whether or not the stage is done.
    pub done: bool,
}

impl ExecOutput {
    /// Mark the stage as done, checkpointing at the given place.
    pub fn done(checkpoint: BlockNumber) -> Self {
        Self { checkpoint, done: true }
    }
}

#[async_trait::async_trait]
pub trait Stage: Send + Sync {
    fn id(&self) -> StageId;
    
    async fn execute(&mut self, input: ExecInput, db_provider: &DatabaseProvider, chain: &ChainSpec) -> Result<ExecOutput, StageError>;
}


pub(crate) type BoxedStage = Box<dyn Stage>;