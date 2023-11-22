mod event;

mod progress;
use progress::PipelineProgress;

mod ctrl;
pub use ctrl::ControlFlow;
mod builder;
pub use builder::PipelineBuilder;
use simp_primitives::{BlockNumber, ChainSpec, StageId};
use simp_tokio_util::EventListeners;
use storage_provider::{traits::*, DatabaseProvider};
use tracing::*;

use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{
    error::StageError,
    pipeline::event::PipelineEvent,
    stage::{BoxedStage, ExecInput, ExecOutput},
    PipelineError,
};

use self::event::PipelineStagesProgress;

pub struct Pipeline {
    stages: Vec<BoxedStage>,

    db: DatabaseProvider,

    chain: ChainSpec,

    listeners: EventListeners<PipelineEvent>,

    max_block: Option<BlockNumber>,
    progress: PipelineProgress,
}

impl Pipeline {
    // builder
    pub fn builder() -> PipelineBuilder {
        PipelineBuilder::default()
    }
    // event listener

    pub fn events(&mut self) -> UnboundedReceiverStream<PipelineEvent> {
        self.listeners.new_listener()
    }
    // run pipeline in infinte loop

    /// Registers progress metrics for each registered stage
    pub fn register_metrics(&mut self) -> Result<(), PipelineError> {
        // TODO: metrics in db
        // let Some(metrics_tx) = &mut self.metrics_tx else { return Ok(()) };
        // let factory = ProviderFactory::new(&self.db, self.chain_spec.clone());
        // let provider = factory.provider()?;

        // for stage in &self.stages {
        //     let stage_id = stage.id();
        //     let _ = metrics_tx.send(MetricEvent::StageCheckpoint {
        //         stage_id,
        //         checkpoint: provider.get_stage_checkpoint(stage_id)?.unwrap_or_default(),
        //         max_block_number: None,
        //     });
        // }
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), PipelineError> {
        self.register_metrics()?;
        loop {
            let next_action = self.run_loop().await?;

            // Terminate the loop early if it's reached the maximum block number
            // configured block.
            if next_action.should_continue()
                && self
                    .progress
                    .minimum_block_number
                    .zip(self.max_block)
                    .map_or(false, |(progress, target)| progress >= target)
            {
                trace!(
                    target: "sync::pipeline",
                    ?next_action,
                    minimum_block_number = ?self.progress.minimum_block_number,
                    max_block = ?self.max_block,
                    "Terminating pipeline."
                );
                return Ok(());
            }
        }
    }

    pub async fn run_loop(&mut self) -> Result<ControlFlow, PipelineError> {
        let mut previous_stage = None;
        for stage_index in 0..self.stages.len() {
            let stage = &self.stages[stage_index];
            let stage_id = stage.id();

            trace!(target: "sync::pipeline", stage = %stage_id, "Executing stage");
            let next = self
                .execute_stage_to_completion(previous_stage, stage_index)
                .instrument(info_span!("execute", stage = %stage_id))
                .await?;

            trace!(target: "sync::pipeline", stage = %stage_id, ?next, "Completed stage");

            match next {
                ControlFlow::NoProgress { block_number } => {
                    if let Some(block_number) = block_number {
                        self.progress.update(block_number);
                    }
                }
                ControlFlow::Continue { block_number } => self.progress.update(block_number),
                ControlFlow::Unwind { target, bad_block } => {
                    self.unwind(target, Some(bad_block)).await?;
                    return Ok(ControlFlow::Unwind { target, bad_block });
                }
            }

            previous_stage = self
                .db
                .get_stage_checkpoint(stage_id)
                .unwrap()
                .map(|progress| progress);
        }

        Ok(self.progress.next_ctrl())
    }
    // run pipeline once

    // unwind ??
    pub async fn unwind(
        &mut self,
        target: BlockNumber,
        bad_block: Option<BlockNumber>,
    ) -> Result<(), PipelineError> {
        unimplemented!()
    }

    // execute stage to completion

    pub async fn execute_stage_to_completion(
        &mut self,
        previous_stage: Option<BlockNumber>,
        stage_index: usize,
    ) -> Result<ControlFlow, PipelineError> {
        let total_stages = self.stages.len();
        let db_provider = &self.db;

        let stage = &mut self.stages[stage_index];
        let stage_id = stage.id();
        let mut made_progress = false;
        let target = self.max_block.or(previous_stage);

        loop {
            let prev_checkpoint = db_provider.get_stage_checkpoint(stage_id).unwrap();

            let stage_reached_max_block = prev_checkpoint
                .zip(self.max_block)
                .map_or(false, |(prev_progress, target)| prev_progress >= target);
            if stage_reached_max_block {
                warn!(
                    target: "sync::pipeline",
                    stage = %stage_id,
                    max_block = self.max_block,
                    prev_block = prev_checkpoint.map(|progress| progress),
                    "Stage reached target block, skipping."
                );
                self.listeners.notify(PipelineEvent::Skipped { stage_id });

                // We reached the maximum block, so we skip the stage
                return Ok(ControlFlow::NoProgress {
                    block_number: prev_checkpoint.map(|progress| progress),
                });
            }

            self.listeners.notify(PipelineEvent::Running {
                pipeline_stages_progress: PipelineStagesProgress {
                    current: stage_index + 1,
                    total: total_stages,
                },
                stage_id,
                checkpoint: prev_checkpoint,
            });

            // if stage does not error update and continue pipeline
            // else fail gracefully (try again or stop process)
            match stage
                .execute(
                    ExecInput {
                        target,
                        checkpoint: prev_checkpoint,
                    },
                    db_provider,
                    &self.chain,
                )
                .await
            {
                Ok(out @ ExecOutput { checkpoint, done }) => {
                    made_progress |= checkpoint != prev_checkpoint.unwrap_or_default();
                    debug!(
                        target: "sync::pipeline",
                        stage = %stage_id,
                        progress = checkpoint,
                        %checkpoint,
                        %done,
                        "Stage committed progress"
                    );

                    db_provider
                        .save_stage_checkpoint(stage_id, checkpoint)
                        .unwrap();

                    self.listeners.notify(PipelineEvent::Ran {
                        pipeline_stages_progress: PipelineStagesProgress {
                            current: stage_index + 1,
                            total: total_stages,
                        },
                        stage_id,
                        result: out.clone(),
                    });

                    if done {
                        let block_number = checkpoint;
                        return Ok(if made_progress {
                            ControlFlow::Continue { block_number }
                        } else {
                            ControlFlow::NoProgress {
                                block_number: Some(block_number),
                            }
                        });
                    }
                }
                Err(err) => {
                    self.listeners.notify(PipelineEvent::Error { stage_id });
                    // notify error
                    // unwind stage
                    //
                    let out = if let StageError::Block { block } = err {
                        unimplemented!()
                    } else if err.is_fatal() {
                        error!(
                            target: "sync::pipeline",
                            stage = %stage_id,
                            "Stage encountered a fatal error: {err}."
                        );
                        Err(err.into())
                    } else {
                        // On other errors we assume they are recoverable if we discard the
                        // transaction and run the stage again.
                        // TODO: add action
                        warn!(
                            target: "sync::pipeline",
                            stage = %stage_id,
                            "Stage encountered a non-fatal error: {err}. Retrying..."
                        );
                        continue;
                    };
                    return out;
                }
            }
        }
    }
}

impl std::fmt::Debug for Pipeline {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("Pipeline")
            .field(
                "stages",
                &self
                    .stages
                    .iter()
                    .map(|stage| stage.id())
                    .collect::<Vec<StageId>>(),
            )
            .field("listeners", &self.listeners)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use db::test_utils::create_test_rw_db;
    use simp_primitives::{StageId, MAINNET};
    use storage_provider::options::AccessType;

    use crate::{
        pipeline::{progress::PipelineProgress, ControlFlow},
        test_utils::stage::TestStage,
    };
    use tokio_stream::StreamExt;
    
    #[test]
    fn record_progress_calculates_outliers() {
        let mut progress = PipelineProgress::default();

        progress.update(10);
        assert_eq!(progress.minimum_block_number, Some(10));
        assert_eq!(progress.maximum_block_number, Some(10));

        progress.update(20);
        assert_eq!(progress.minimum_block_number, Some(10));
        assert_eq!(progress.maximum_block_number, Some(20));

        progress.update(1);
        assert_eq!(progress.minimum_block_number, Some(1));
        assert_eq!(progress.maximum_block_number, Some(20));
    }

    #[test]
    fn progress_ctrl_flow() {
        let mut progress = PipelineProgress::default();

        assert_eq!(
            progress.next_ctrl(),
            ControlFlow::NoProgress { block_number: None }
        );

        progress.update(1);
        assert_eq!(
            progress.next_ctrl(),
            ControlFlow::Continue { block_number: 1 }
        );
    }

    #[tokio::test]
    async fn run_pipeline() {
        let db = create_test_rw_db();
        let db_provider = DatabaseProvider::new(db, AccessType::Primary);

        let mut pipeline = Pipeline::builder()
            .add_stage(TestStage::new(StageId::Other("A")).add_exec(Ok(ExecOutput {
                checkpoint: 20,
                done: true,
            })))
            .add_stage(TestStage::new(StageId::Other("B")).add_exec(Ok(ExecOutput {
                checkpoint: 10,
                done: true,
            })))
            .with_max_block(10)
            .build(
                db_provider,
                MAINNET.clone().as_ref().clone(),
            );
        let events = pipeline.events();

        // Run pipeline
        tokio::spawn(async move {
            pipeline.run().await.unwrap();
        });

        // Check that the stages were run in order
        assert_eq!(
            events.collect::<Vec<PipelineEvent>>().await,
            vec![
                PipelineEvent::Running {
                    pipeline_stages_progress: PipelineStagesProgress {
                        current: 1,
                        total: 2
                    },
                    stage_id: StageId::Other("A"),
                    checkpoint: None
                },
                PipelineEvent::Ran {
                    pipeline_stages_progress: PipelineStagesProgress {
                        current: 1,
                        total: 2
                    },
                    stage_id: StageId::Other("A"),
                    result: ExecOutput {
                        checkpoint: 20,
                        done: true
                    },
                },
                PipelineEvent::Running {
                    pipeline_stages_progress: PipelineStagesProgress {
                        current: 2,
                        total: 2
                    },
                    stage_id: StageId::Other("B"),
                    checkpoint: None
                },
                PipelineEvent::Ran {
                    pipeline_stages_progress: PipelineStagesProgress {
                        current: 2,
                        total: 2
                    },
                    stage_id: StageId::Other("B"),
                    result: ExecOutput {
                        checkpoint: 10,
                        done: true
                    },
                },
            ]
        );
    }
}
