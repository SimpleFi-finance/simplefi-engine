// builder

// control

// progress

// event
mod event;

mod progress;

use progress::PipelineProgress;

mod ctrl;
pub use ctrl::ControlFlow;
mod builder;
pub use builder::PipelineBuilder;
use primitives::BlockNumber;
use tokio_util::EventListeners;
use tracing::*;

use tokio_stream::wrappers::UnboundedReceiverStream;


use crate::{stage::BoxedStage, StageId, PipelineError};


pub struct Pipeline {
    stages: Vec<BoxedStage>,

    // TODO: add Pipeline events
    listeners: EventListeners<String>,

    max_block: Option<BlockNumber>,
    // TODO: keep track of pipeline progress
    progress: PipelineProgress,
}

impl Pipeline {
    // builder
    pub fn builder() -> PipelineBuilder{
        PipelineBuilder::default()
    }
    // event listener

    pub fn events(&mut self) -> UnboundedReceiverStream<String> {
        self.listeners.new_listener()
    }
    // run pipeline in infinte loop

    pub async fn run(&mut self) -> Result<(), PipelineError> {
        loop {
            let next_action = self.run_loop().await?;

            // Terminate the loop early if it's reached the maximum user
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

            // let factory = ProviderFactory::new(&self.db, self.chain_spec.clone());

            // previous_stage = Some(
            //     factory
            //         .provider()?
            //         .get_stage_checkpoint(stage_id)?
            //         .unwrap_or_default()
            //         .block_number,
            // );
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

        let stage = &mut self.stages[stage_index];
        let stage_id = stage.id();
        let mut made_progress = false;
        let target = self.max_block.or(previous_stage);

        let factory = ProviderFactory::new(&self.db, self.chain_spec.clone());

        let mut provider_rw = factory.provider_rw().map_err(PipelineError::Interface)?;

        loop {
            let prev_checkpoint = provider_rw.get_stage_checkpoint(stage_id)?;

            let stage_reached_max_block = prev_checkpoint
                .zip(self.max_block)
                .map_or(false, |(prev_progress, target)| prev_progress.block_number >= target);
            if stage_reached_max_block {
                warn!(
                    target: "sync::pipeline",
                    stage = %stage_id,
                    max_block = self.max_block,
                    prev_block = prev_checkpoint.map(|progress| progress.block_number),
                    "Stage reached target block, skipping."
                );
                self.listeners.notify(PipelineEvent::Skipped { stage_id });

                // We reached the maximum block, so we skip the stage
                return Ok(ControlFlow::NoProgress {
                    block_number: prev_checkpoint.map(|progress| progress.block_number),
                })
            }

            self.listeners.notify(PipelineEvent::Running {
                pipeline_position: stage_index + 1,
                pipeline_total: total_stages,
                stage_id,
                checkpoint: prev_checkpoint,
            });

            match stage
                .execute()
                .await
            {
                Ok(out @ ExecOutput { checkpoint, done }) => {
                    made_progress |=
                        checkpoint.block_number != prev_checkpoint.unwrap_or_default().block_number;
                    debug!(
                        target: "sync::pipeline",
                        stage = %stage_id,
                        progress = checkpoint.block_number,
                        %checkpoint,
                        %done,
                        "Stage committed progress"
                    );
                    if let Some(metrics_tx) = &mut self.metrics_tx {
                        let _ = metrics_tx.send(MetricEvent::StageCheckpoint {
                            stage_id,
                            checkpoint,
                            max_block_number: target,
                        });
                    }
                    provider_rw.save_stage_checkpoint(stage_id, checkpoint)?;

                    self.listeners.notify(PipelineEvent::Ran {
                        pipeline_position: stage_index + 1,
                        pipeline_total: total_stages,
                        stage_id,
                        result: out.clone(),
                    });

                    // TODO: Make the commit interval configurable
                    provider_rw.commit()?;
                    provider_rw = factory.provider_rw().map_err(PipelineError::Interface)?;

                    if done {
                        let block_number = checkpoint.block_number;
                        return Ok(if made_progress {
                            ControlFlow::Continue { block_number }
                        } else {
                            ControlFlow::NoProgress { block_number: Some(block_number) }
                        })
                    }
                }
                Err(err) => {
                    self.listeners.notify(PipelineEvent::Error { stage_id });

                    let out = if let StageError::DetachedHead { local_head, header, error } = err {
                        warn!(target: "sync::pipeline", stage = %stage_id, ?local_head, ?header, ?error, "Stage encountered detached head");

                        // We unwind because of a detached head.
                        let unwind_to = local_head
                            .number
                            .saturating_sub(BEACON_CONSENSUS_REORG_UNWIND_DEPTH)
                            .max(1);
                        Ok(ControlFlow::Unwind { target: unwind_to, bad_block: local_head })
                    } else if let StageError::Block { block, error } = err {
                        match error {
                            BlockErrorKind::Validation(validation_error) => {
                                error!(
                                    target: "sync::pipeline",
                                    stage = %stage_id,
                                    bad_block = %block.number,
                                    "Stage encountered a validation error: {validation_error}"
                                );

                                drop(provider_rw);
                                provider_rw =
                                    factory.provider_rw().map_err(PipelineError::Interface)?;
                                provider_rw.save_stage_checkpoint_progress(
                                    StageId::MerkleExecute,
                                    vec![],
                                )?;
                                provider_rw.save_stage_checkpoint(
                                    StageId::MerkleExecute,
                                    prev_checkpoint.unwrap_or_default(),
                                )?;
                                provider_rw.commit()?;

                                // We unwind because of a validation error. If the unwind itself
                                // fails, we bail entirely,
                                // otherwise we restart the execution loop from the
                                // beginning.
                                Ok(ControlFlow::Unwind {
                                    target: prev_checkpoint.unwrap_or_default().block_number,
                                    bad_block: block,
                                })
                            }
                            BlockErrorKind::Execution(execution_error) => {
                                error!(
                                    target: "sync::pipeline",
                                    stage = %stage_id,
                                    bad_block = %block.number,
                                    "Stage encountered an execution error: {execution_error}"
                                );

                                // We unwind because of an execution error. If the unwind itself
                                // fails, we bail entirely,
                                // otherwise we restart
                                // the execution loop from the beginning.
                                Ok(ControlFlow::Unwind {
                                    target: prev_checkpoint.unwrap_or_default().block_number,
                                    bad_block: block,
                                })
                            }
                        }
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