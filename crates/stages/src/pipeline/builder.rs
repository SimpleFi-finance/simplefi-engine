// defines the pipeline

// defines the stages in order

// includes the db?

use simp_primitives::{StageId, BlockNumber,ChainSpec};
use storage_provider::DatabaseProvider;

use crate::{stage::BoxedStage, Stage};

use super::Pipeline;

pub struct PipelineBuilder {
    stages: Vec<BoxedStage>,

    max_block: Option<BlockNumber>,
}

impl PipelineBuilder {
    pub fn add_stage<S>(mut self, stage: S) -> Self
    where 
        S: Stage + 'static,
    {
        self.stages.push(Box::new(stage));
        self
    }

    pub fn with_max_block(mut self, block: BlockNumber) -> Self {
        self.max_block = Some(block);
        self
    }
    pub fn build(self, db: DatabaseProvider, chain_spec: ChainSpec) -> Pipeline {
        let Self { 
            stages,
            max_block
        } = self;
        Pipeline {
            db,
            chain: chain_spec,
            stages,
            max_block,
            listeners: Default::default(),
            progress: Default::default(),
        }
    }
}

impl  Default for PipelineBuilder {
    fn default() -> Self {
        Self { 
            stages: Vec::new(),
            max_block: None,
        }
    }
}

impl std::fmt::Debug for PipelineBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelineBuilder")
            .field("stages", &self.stages.iter().map(|stage| stage.id()).collect::<Vec<StageId>>())
            .field("max_block", &self.max_block)
            .finish()
    }
}
