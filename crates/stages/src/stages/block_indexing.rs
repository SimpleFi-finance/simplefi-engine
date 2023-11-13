use simp_primitives::StageId;
use storage_provider::DatabaseProvider;
use crate::{Stage, stage::{ExecInput, ExecOutput}, error::StageError};

pub struct BlockIndexingStage;

#[async_trait::async_trait]
impl Stage for BlockIndexingStage {
    fn id(&self) -> StageId {
        StageId::BlockIndexing
    }
    /// saves the Sealed header of the block in the database
    async fn execute(&mut self, input: ExecInput, db_provider: &DatabaseProvider) ->  Result<ExecOutput, StageError> {
        let target = input.target();
        let checkpoint = input.checkpoint();
        // load chain and load appropriate method
        for block in checkpoint..=target {
            // load chain to get block methods
            println!("block indexing: {}", block);
            // TODO: get headers and store them
        }
        Ok(ExecOutput { checkpoint: input.target(), done: true })
    }
}