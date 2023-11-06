use db::{transaction::DbTx, tables::SyncStage};

use crate::{traits::{StageCheckpointProvider, StageCheckpointWriter}, DatabaseProvider};

impl StageCheckpointProvider for DatabaseProvider {
    fn get_stage_checkpoint(&self,id:primitives::StageId) -> interfaces::Result<Option<primitives::BlockNumber> > {
        let bn = self.db.dae_get::<SyncStage>(id.to_string());

        Ok(bn.unwrap())
    }
}

impl StageCheckpointWriter for DatabaseProvider {
    fn save_stage_checkpoint(&self,id:primitives::StageId, checkpoint: primitives::BlockNumber) -> interfaces::Result<()> {
        self.db.dae_put::<SyncStage>(id.to_string(), checkpoint).unwrap();
        Ok(())
    }
}