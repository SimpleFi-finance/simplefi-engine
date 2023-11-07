use primitives::{StageId, BlockNumber};

use crate::error::StageError;

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

    async fn execute(&mut self) -> Result<ExecOutput, StageError>;
}


pub(crate) type BoxedStage = Box<dyn Stage>;