use db::{transaction::DbTx, tables::SyncStage};

use crate::{traits::{StageCheckpointProvider, StageCheckpointWriter}, DatabaseProvider};

impl StageCheckpointProvider for DatabaseProvider {
    fn get_stage_checkpoint(&self,id:primitives::StageId) -> interfaces::Result<Option<primitives::BlockNumber> > {
        let bn = self.db.get::<SyncStage>(id.to_string());

        Ok(bn.unwrap())
    }
}

impl StageCheckpointWriter for DatabaseProvider {
    fn save_stage_checkpoint(&self,id:primitives::StageId, checkpoint: primitives::BlockNumber) -> interfaces::Result<()> {
        self.db.put::<SyncStage>(id.to_string(), checkpoint).unwrap();
        Ok(())
    }

    // fn save_stage_checkpoint_progress(&self,id:primitives::StageId, checkpoint: Vec<u8>) -> interfaces::Result<()> {
    //     unimplemented!()
    // }
}