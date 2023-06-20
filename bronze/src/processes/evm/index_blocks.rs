use std::{thread, time::Duration};

use bronze::mongo::evm::data_sets::{blocks::Block, txs::Tx, logs::Log};
use chains_drivers::{chains::{SupportedChains, get_chain}, types::chain::{IndexFullBlocks, Info, ChainDB}};
use settings::load_settings;
use shared_types::data_lake::{SupportedDataTypes, SupportedDataLevels};
use third_parties::{redis::connect_client, mongo::{lib::bronze::setters::save_to_db, Mongo}};
use redis::{AsyncCommands};

#[tokio::main]
async fn main() {
    // load chain using settings name
    let glob_settings = load_settings().unwrap();

    let chain_id = "1"; //todo switch to env

    match chain_id {
        "1" => {
            let uri = glob_settings.redis_uri.clone();
            let chain = get_chain(chain_id).unwrap();

            let queue_name = format!("{}_blocks", chain.info().symbol.to_lowercase());

            let redis_cli = connect_client(&uri).await.unwrap();

            let mut redis_conn = redis_cli.get_connection().unwrap();
            let mut redis_async_conn = redis_cli.get_async_connection().await.unwrap();
            let mut pubsub = redis_conn.as_pubsub();
            pubsub.subscribe(&queue_name).unwrap();

            let db = chain.get_db();

            let mongo_db = Mongo::new(&db).await.unwrap();

            loop {
                let mut len: isize = redis_async_conn.llen(&queue_name).await.unwrap();
                while len > 0 {
                    let data: Vec<isize> = redis_async_conn.lpop(&queue_name, None).await.unwrap();
                    if data.len() > 0 {
                        len = redis_async_conn.llen(&queue_name).await.unwrap();

                        let data = chain
                            .index_full_blocks(
                                true, 
                                data[0] as u64, 
                                None)
                            .await
                            .unwrap();

                        // todo save to mongo
                        let blocks = data.0;
                        let transactions = data.1;
                        let logs = data.2;
                        
                        let mongo_blocks = blocks.into_iter().map(|block| {
                            let block : Block = serde_json::from_value(block).unwrap();
                            block
                        }).collect::<Vec<Block>>();
                        
                        let mongo_txs = transactions.into_iter().map(|tx| {
                            let tx : Tx = serde_json::from_value(tx).unwrap();
                            tx
                        }).collect::<Vec<Tx>>();

                        let mongo_logs = logs.into_iter().map(|log| {
                            let log : Log= serde_json::from_value(log).unwrap();
                            log
                        }).collect::<Vec<Log>>();
                        
                        // todo add decoding

                        let(_,_,_) = tokio::join!(
                            save_to_db::<Block>(mongo_blocks, &mongo_db, chain.resolve_collection_name(&SupportedDataTypes::Blocks, &SupportedDataLevels::Bronze)),
                            save_to_db::<Log>(mongo_logs, &mongo_db, chain.resolve_collection_name(&SupportedDataTypes::Logs, &SupportedDataLevels::Bronze)),
                            save_to_db::<Tx>(mongo_txs, &mongo_db, chain.resolve_collection_name(&SupportedDataTypes::Transactions, &SupportedDataLevels::Bronze)),
                        );

                        thread::sleep(Duration::from_millis(300));
                    } else {
                        len = 0;
                    }
                }
                // receives values back, convert to mongodb documents, insert into mongodb
            }
        },
        _ => panic!("Chain not implemented to index blocks"),
    };
}
