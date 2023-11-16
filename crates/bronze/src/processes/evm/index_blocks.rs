use rayon::{prelude::{IntoParallelIterator}, iter::ParallelIterator};
use simplefi_engine_settings::load_settings;
use chains_types::get_chain;
use chains_types::common::chain::{
    IndexFullBlocks,
    Info,
    DecodeLogs
};
use simplefi_redis::connect_client;

use redis::AsyncCommands;

#[tokio::main]
async fn main() {
    // load chain using settings name
    let glob_settings = load_settings().unwrap();

    let chain_id = "1"; //todo switch to env

    match chain_id {
        "1" => {
            let uri = "".clone();
            let chain = get_chain(chain_id).unwrap();

            let queue_name = format!("{}_blocks", chain.info().symbol.to_lowercase());

            let redis_cli = connect_client(&uri).await.unwrap();

            let mut redis_conn = redis_cli.get_connection().unwrap();
            let mut redis_async_conn = redis_cli.get_async_connection().await.unwrap();

            let mut pubsub = redis_conn.as_pubsub();
            pubsub.subscribe(&queue_name).unwrap();

            // let db = chain.get_db();

            // let mongo_db = Mongo::new(&db).await.unwrap();

            loop {
                loop {
                    let data: Vec<isize> = redis_async_conn
                        .lpop(&queue_name, None)
                        .await
                        .unwrap_or_default();

                    if data.len() == 0 {
                        break;
                    }

                    let data = chain
                        .index_full_blocks(true, data[0] as u64, None)
                        .await
                        .unwrap();

                    let blocks = data.0;
                    let transactions = data.1;
                    let logs = data.2;

                    // let mongo_blocks = blocks
                    //     .into_par_iter()
                    //     .map(|block| {
                    //         let block: Block = serde_json::from_value(block).unwrap();
                    //         block
                    //     })
                    //     .collect::<Vec<Block>>();

                    // let mongo_txs = transactions
                    //     .into_par_iter()
                    //     .map(|tx| {
                    //         let tx: Tx = serde_json::from_value(tx).unwrap();
                    //         tx
                    //     })
                    //     .collect::<Vec<Tx>>();
                    
                    // let decoded = chain.decode_logs(logs).await.unwrap();

                    // let mongo_logs = decoded
                    //     .0
                    //     .into_iter()
                    //     .map(|log| {
                    //         let log: Log = serde_json::from_value(log).unwrap();
                    //         log
                    //     })
                    //     .collect::<Vec<Log>>();

                    // let decoding_errors = decoded
                    //     .1
                    //     .into_iter()
                    //     .map(|error| {
                    //         let error: DecodingError = serde_json::from_value(error).unwrap();
                    //         error
                    //     })
                    //     .collect::<Vec<DecodingError>>();

                    // let (_, _, _, _) = tokio::join!(
                    //     save_to_db::<Block>(
                    //         mongo_blocks,
                    //         &mongo_db,
                    //         chain.resolve_collection_name(
                    //             &SupportedDataTypes::Blocks,
                    //             &SupportedDataLevels::Bronze
                    //         )
                    //     ),
                    //     save_to_db::<Log>(
                    //         mongo_logs,
                    //         &mongo_db,
                    //         chain.resolve_collection_name(
                    //             &SupportedDataTypes::Logs,
                    //             &SupportedDataLevels::Bronze
                    //         )
                    //     ),
                    //     save_to_db::<Tx>(
                    //         mongo_txs,
                    //         &mongo_db,
                    //         chain.resolve_collection_name(
                    //             &SupportedDataTypes::Transactions,
                    //             &SupportedDataLevels::Bronze
                    //         )
                    //     ),
                    //     save_to_db(
                    //         decoding_errors,
                    //         &mongo_db,
                    //         chain.resolve_collection_name(
                    //             &SupportedDataTypes::DecodingError,
                    //             &SupportedDataLevels::Bronze
                    //         )
                    //     )
                    // );
                }
            }
        }
        _ => panic!("Chain not implemented to index blocks"),
    };
}
