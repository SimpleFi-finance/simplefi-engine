use interfaces::Result;
use primitives::{BlockNumber, StageId};

#[auto_impl::auto_impl(&, Arc)]
pub trait StageCheckpointProvider: Send + Sync {
    /// Fetch the checkpoint for the given stage.
    fn get_stage_checkpoint(&self, id: StageId) -> Result<Option<BlockNumber>>;
}

/// The trait for updating stage checkpoint related data.
#[auto_impl::auto_impl(&, Arc)]
pub trait StageCheckpointWriter: Send + Sync {
    /// Save stage checkpoint.
    fn save_stage_checkpoint(&self, id: StageId, checkpoint: BlockNumber) -> Result<()>;

    // /// Save stage checkpoint progress.
    // fn save_stage_checkpoint_progress(&self, id: StageId, checkpoint: Vec<u8>) -> Result<()>;

    // /// Update all pipeline sync stage progress.
    // fn update_pipeline_stages(
    //     &self,
    //     block_number: BlockNumber,
    //     drop_stage_checkpoint: bool,
    // ) -> Result<()>;
}
