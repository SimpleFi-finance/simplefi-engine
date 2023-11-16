use simp_primitives::{StageId, ChainSpec, ComputationEngine, ChainRpcProvider, Header};
use storage_provider::DatabaseProvider;
use crate::{Stage, stage::{ExecInput, ExecOutput}, error::StageError};
use storage_provider::traits::*;
pub struct HeadersStage;

#[async_trait::async_trait]
impl Stage for HeadersStage {
    fn id(&self) -> StageId {
        StageId::Headers
    }
    /// saves the Sealed header of the block in the database
    async fn execute(&mut self, input: ExecInput, db_provider: &DatabaseProvider, chain: &ChainSpec) ->  Result<ExecOutput, StageError> {
        let target = input.target();
        let checkpoint = input.checkpoint() + 1;
            // load chain to get block methods
            match chain.chain_type() {
                ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                    // TODO: if more than 10k blocks, batch calls
                    let headers = chain.get_blocks_headers::<Header>(checkpoint, target).unwrap();

                    for header in headers.iter() {
                        db_provider.insert_block_hash(header.number, header.hash).unwrap();

                        db_provider.insert_block_number(header.hash, header.number).unwrap();

                        db_provider.insert_header(&header.number, header.clone()).unwrap();
                    }
                },
                _ => panic!("chain not supported")
            }
        Ok(ExecOutput { checkpoint: input.target(), done: true })
    }
}