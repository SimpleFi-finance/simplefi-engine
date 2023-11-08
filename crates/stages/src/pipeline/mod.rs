mod event;

mod progress;

use std::sync::Arc;
use progress::PipelineProgress;

mod ctrl;
pub use ctrl::ControlFlow;
mod builder;
pub use builder::PipelineBuilder;
use simp_primitives::{BlockNumber, StageId};
use rocksdb::{TransactionDB, MultiThreaded};
use storage_provider::{DatabaseProvider, traits::*};
use tracing::*;
use simp_tokio_util::EventListeners;

use tokio_stream::wrappers::UnboundedReceiverStream;


use crate::{stage::{BoxedStage, ExecOutput, ExecInput}, PipelineError, error::StageError, pipeline::event::PipelineEvent};

use self::event::PipelineStagesProgress;

pub struct Pipeline {
    stages: Vec<BoxedStage>,

    db: DatabaseProvider,
    listeners: EventListeners<PipelineEvent>,

    max_block: Option<BlockNumber>,
    progress: PipelineProgress,
}

impl Pipeline {
    // builder
    pub fn builder() -> PipelineBuilder{
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
            if next_action.should_continue() &&
                self.progress
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
                return Ok(())
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
                    return Ok(ControlFlow::Unwind { target, bad_block })
                }
            }

            previous_stage = self.db.get_stage_checkpoint(stage_id).unwrap().map(|progress| progress);
        }

        Ok(self.progress.next_ctrl())
    }
    // run pipeline once

    // unwind ??
    pub async fn unwind(&mut self, target: BlockNumber, bad_block: Option<BlockNumber>) -> Result<(), PipelineError> {
        unimplemented!()
    }

    // execute stage to completion

    pub async fn execute_stage_to_completion(
        &mut self,
        previous_stage: Option<BlockNumber>,
        stage_index: usize,
    ) -> Result<ControlFlow, PipelineError>{
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
                })
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
                )
                .await
            {
                Ok(out @ ExecOutput { checkpoint, done }) => {
                    made_progress |=
                        checkpoint != prev_checkpoint.unwrap_or_default();
                    debug!(
                        target: "sync::pipeline",
                        stage = %stage_id,
                        progress = checkpoint,
                        %checkpoint,
                        %done,
                        "Stage committed progress"
                    );
                   
                    db_provider.save_stage_checkpoint(stage_id, checkpoint).unwrap();

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
                            ControlFlow::NoProgress { block_number: Some(block_number) }
                        })
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
                        continue
                    };
                    return out
                }
            }
        }
    }

}


impl std::fmt::Debug for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pipeline")
            .field("stages", &self.stages.iter().map(|stage| stage.id()).collect::<Vec<StageId>>())
            .field("listeners", &self.listeners)
            .finish()
    }
}