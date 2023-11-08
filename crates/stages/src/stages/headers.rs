use simp_primitives::StageId;
use storage_provider::DatabaseProvider;
use crate::{Stage, stage::{ExecInput, ExecOutput}, error::StageError};

pub struct HeadersStage;

#[async_trait::async_trait]
impl Stage for HeadersStage {
    fn id(&self) -> StageId {
        StageId::Headers
    }

    async fn execute(&mut self, input: ExecInput, db_provider: &DatabaseProvider) ->  Result<ExecOutput, StageError> {
        
        let target = input.target();
        let checkpoint = input.checkpoint();

        for block in checkpoint..=target {
            println!("block: {}", block);
            // TODO: get headers and store them
        }
        Ok(ExecOutput { checkpoint: input.target(), done: true })
    }
}