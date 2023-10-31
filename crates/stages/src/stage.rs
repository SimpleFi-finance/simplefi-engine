use primitives::StageId;


#[async_trait::async_trait]
pub trait Stage: Send + Sync {
    fn id(&self) -> StageId;

    async fn execute(&mut self) -> ();
}


pub(crate) type BoxedStage = Box<dyn Stage>;