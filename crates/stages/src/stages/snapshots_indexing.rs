use simp_primitives::{StageId, ChainSpec};
use storage_provider::DatabaseProvider;
use crate::{Stage, stage::{ExecInput, ExecOutput}, error::StageError};

pub struct SnapshotsIndexingStage;

#[async_trait::async_trait]
impl Stage for SnapshotsIndexingStage {
    fn id(&self) -> StageId {
        StageId::SnapshotsIndexing
    }
    /// saves the Sealed header of the block in the database
    async fn execute(&mut self, input: ExecInput, db_provider: &DatabaseProvider, chain: &ChainSpec) ->  Result<ExecOutput, StageError> {
        let target = input.target();
        let checkpoint = input.checkpoint();

        for block in checkpoint..=target {
            // load chain to get block methods
            println!("snapshots indexing: {}", block);
            // TODO: get headers and store them
        }
        Ok(ExecOutput { checkpoint: input.target(), done: true })
    }
}