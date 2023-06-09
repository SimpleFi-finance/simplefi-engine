use datafusion::{prelude::{SessionContext, ParquetReadOptions}, arrow::json::writer::record_batches_to_json_rows};
use object_store::{path::Path};
use serde_json::json;
use settings::load_settings as load_global_settings;
use sync_chain_data_lake::{utils::gcp::{gcp_object_store, get_latest_partition_in_bucket}, settings::load_settings};
use url::Url;
use std::sync::Arc;
/*
    this script allows to connect to a JSON-RPC endpoint for eth, check if previous data was saved already, if not, it will start from the first blokc or the blokc specified and sync the chain to the latest block

    it saves the data into a mongodb database to be staged

 */
#[tokio::main]
async fn main() {
    let glob_settings = load_global_settings().expect("Failed to load global settings");
    let local_settings = load_settings().expect("Failed to load local settings");

    let gcp_bucket = glob_settings.cloud_bucket;
    let object_store = gcp_object_store(&gcp_bucket);

    let data_path: Path = "ethereum/bronze/logs".try_into().unwrap();

    let gcs = Arc::new(object_store);

    let last_partition = get_latest_partition_in_bucket(&gcs, &data_path).await.unwrap();

    // load the partition and get latest block number
    let path = format!("gs://{}/{}", gcp_bucket, last_partition);

    let ctx = SessionContext::new();

    ctx
        .runtime_env()
        .register_object_store(
            &Url::parse(&path).unwrap(), 
            gcs.clone()
        );

    ctx.register_parquet("logs", &path, ParquetReadOptions::default())
        .await.unwrap();

    let start_block_query = "SELECT block_number FROM logs ORDER BY block_number DESC LIMIT 1".to_string();

    let res = ctx.sql(&start_block_query).await.unwrap();

    let latest_block_saved = res.collect().await.unwrap();

    let latest_block_number = record_batches_to_json_rows(&latest_block_saved)
        .unwrap()[0]["block_number"]
        .as_i64()
        .unwrap();
    // todo add exclusion if no latest block found

    println!("latest_block_saved: {:?}", latest_block_number);

    // query latest block to rpc
    /*
        get blocks data
        stage in mongo?
        digest from mongo to parquet
        clean mongo
        potentially decode the logs while gathering the data
     */

    let request_method = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_blockNumber",
        "params": []
    });

    let request_str = serde_json::to_string(&request_method).unwrap();

    // post to rpc to get latest block

    

}