// defines the pipeline

// defines the stages in order

// includes the db?

use simp_primitives::StageId;

use crate::{stage::BoxedStage, Stage};

pub struct PipelineBuilder {
    stages: Vec<BoxedStage>,
}

impl PipelineBuilder {
    pub fn add_stage<S>(mut self, stage: S) -> Self
    where 
        S: Stage + 'static,
    {
        self.stages.push(Box::new(stage));
        self
    }

    // pub fn build(self, db: DB, chain_spec: Arc<ChainSpec>) -> Pipeline<DB> {
    //     let Self { stages} = self;
    //     Pipeline {
    //         db,
    //         chain_spec,
    //         stages,
    //         listeners: Default::default(),
    //         progress: Default::default(),
    //     }
    // }
}

impl  Default for PipelineBuilder {
    fn default() -> Self {
        Self { stages: Vec::new() }
    }
}

impl std::fmt::Debug for PipelineBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelineBuilder")
            .field("stages", &self.stages.iter().map(|stage| stage.id()).collect::<Vec<StageId>>())
            .finish()
    }
}
