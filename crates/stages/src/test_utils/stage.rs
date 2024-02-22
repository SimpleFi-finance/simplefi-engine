use crate::{Stage, stage::{ExecOutput, ExecInput}, error::StageError};
use simp_primitives::{StageId, ChainSpec};
use storage_provider::DatabaseProvider;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct TestStage {
    id: StageId,
    exec_outputs: VecDeque<Result<ExecOutput, StageError>>,
}

impl TestStage {
    pub fn new(id: StageId) -> Self {
        Self { id, exec_outputs: VecDeque::new() }
    }

    pub fn with_exec(mut self, exec_outputs: VecDeque<Result<ExecOutput, StageError>>) -> Self {
        self.exec_outputs = exec_outputs;
        self
    }

    pub fn add_exec(mut self, output: Result<ExecOutput, StageError>) -> Self {
        self.exec_outputs.push_back(output);
        self
    }
}

#[async_trait::async_trait]
impl Stage for TestStage {
    fn id(&self) -> StageId {
        self.id
    }

    async fn execute(
        &mut self,
        _input: ExecInput,
        _db_provider: &DatabaseProvider,
        _chain: &ChainSpec,
    ) -> Result<ExecOutput, StageError> {
        self.exec_outputs
            .pop_front()
            .unwrap_or_else(|| panic!("Test stage {} executed too many times.", self.id))
    }
}
