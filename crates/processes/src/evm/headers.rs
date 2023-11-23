use simp_primitives::{ChainSpec, ComputationEngine};
use storage_provider::DatabaseProvider;

use crate::{ProcessId, Process, ExecInput};

pub struct HeaderProcess;

impl Process for HeaderProcess {
    fn id(&self) -> ProcessId {
        ProcessId::Headers
    }
    #[allow(unused_variables)]
    fn execute<T>(&mut self, input: ExecInput, db_provider: Option<&DatabaseProvider>, chain: ChainSpec) -> Vec<T> 
    {

        // load chain Rpc methods
        // get headers and return or store them

        // receives the header => converts to rockDb => stores and returns
        match chain.computation_engine {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                // let db = db_provider.unwrap();
                // if input.end_bn.is_some() {
                //     let headers = chain.get_blocks_headers::<Header>(input.start_bn, input.end_bn.unwrap()).await.unwrap();

                //     for header in headers.iter() {
                //         db.insert_block_hash(header.number, header.hash).unwrap();

                //         db.insert_block_number(header.hash, header.number).unwrap();

                //         db.insert_header(&header.number, header.clone()).unwrap();
                //     }

                //     // return headers;

                //     return vec![];
                // }

                // let header = chain.get_block_header::<Header>(input.start_bn).unwrap();
                
                // db.insert_block_hash(header.number, header.hash).unwrap();

                // db.insert_block_number(header.hash, header.number).unwrap();

                // db.insert_header(&header.number, header.clone()).unwrap();

                // store    
                // return vec![header];
                return vec![];
            },
            _ => panic!("Unsupported computation engine")
        }
    }
}