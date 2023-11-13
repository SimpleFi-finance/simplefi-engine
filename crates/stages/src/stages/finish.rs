use simp_primitives::{StageId, ChainSpec};
use storage_provider::DatabaseProvider;
use crate::{stage::{ExecInput, ExecOutput}, error::StageError, Stage};

/// The finish stage.
///
/// This stage does not write anything; it's checkpoint is used to denote the highest fully synced
/// block.
#[derive(Default, Debug, Clone)]
pub struct FinishStage;

#[async_trait::async_trait]
impl Stage for FinishStage {
    fn id(&self) -> StageId {
        StageId::Finish
    }

    async fn execute(
        &mut self,
        input: ExecInput,
        db_provider: &DatabaseProvider,
        chain: &ChainSpec
    ) -> Result<ExecOutput, StageError> {
        Ok(ExecOutput { checkpoint: input.target(), done: true })
    }
}