use simp_settings::load_settings;
use chains_types::get_chain;
use chains_types::common::chain::{
    IndexFullBlocks,
    Info
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
                }
            }
        }
        _ => panic!("Chain not implemented to index blocks"),
    };
}
