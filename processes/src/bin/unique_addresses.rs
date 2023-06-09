use serde::{Serialize, Deserialize};
use settings::load_settings as load_global_settings;
use futures::{stream::StreamExt};
use chains_drivers::{
    ethereum::mainnet::ethereum_mainnet, 
};
use object_store::ObjectStore;
use shared_types::data_lake::{SupportedDataLevels, SupportedDataTypes};
use mongodb::bson::doc;

use object_store::{path::Path};
use std::{sync::Arc, collections::HashSet};

use duckdb::{Connection};
use duckdb::arrow::record_batch::RecordBatch;
use duckdb::arrow::json::writer::record_batches_to_json_rows;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UniqueAddresses {
    address: String,
}

async fn get_partition_data (path: &String) -> Vec<String>{

    let glob_settings = load_global_settings().unwrap();
    let gcp_bucket = glob_settings.cloud_bucket;

    let data_path: Path = "ethereum/bronze/logs".try_into().unwrap();

    // duck db

    let conn = Connection::open_in_memory().unwrap();
    
    conn.execute_batch("INSTALL parquet; LOAD parquet;INSTALL httpfs; LOAD httpfs").unwrap();
    let gcs_sql = format!("SET s3_endpoint='storage.googleapis.com'; SET s3_access_key_id='{}';  SET s3_secret_access_key='{}';", glob_settings.gcs_access_key_id, glob_settings.gcs_secret_access_key);
    conn.execute_batch(&gcs_sql).unwrap();

    let path = format!("s3://{}/{}/{}/**/*.parquet", gcp_bucket, data_path, path);
    println!("path: {:?}", path);

    let query = format!("SELECT DISTINCT address FROM read_parquet('{}');", path);
    let rbs: Vec<RecordBatch> = conn
        .prepare(&query).unwrap()
        .query_arrow([]).unwrap()
        .collect();

    let addresses = record_batches_to_json_rows(&rbs).unwrap().into_iter().map(|a| {
            a["address"].as_str().unwrap().to_string()
    }).collect::<Vec<String>>();
    addresses
}


#[tokio::main]
async fn main () {
    // todo make it flexible for multichain support

    let glob_settings = load_global_settings().unwrap();
    let gcp_bucket = glob_settings.cloud_bucket;
    let object_store = gcp_object_store(&gcp_bucket);

    let data_path: Path = "ethereum/bronze/logs".try_into().unwrap();

    let gcs = Arc::new(object_store);

    let chain = ethereum_mainnet().await.unwrap();

    let mut list_stream = gcs
        .list(Some(&data_path))
        .await
        .expect("Error listing files");

    let mut dirs = HashSet::new();

    while let Some(meta) = list_stream.next().await {
        let meta = meta.unwrap();

        if meta.location.to_string().contains(".DS_Store") {
            println!("object.name: {:?}", meta.location);
        } else {

            let path = meta.location.to_string();

            let partitions = path.split("/").collect::<Vec<&str>>();

            let path2 = format!("{}/{}", partitions[3], partitions[4]);
            dirs.insert(path2);
        }
    }

    for dir in dirs {
        let addresses_data = get_partition_data(&dir).await;

        let data = addresses_data.into_iter().map(|a| {
            UniqueAddresses {
                address: a,
            }
        }).collect::<Vec<UniqueAddresses>>();
    
        chain.chain.save_to_db::<UniqueAddresses>(data, &SupportedDataTypes::UniqueAddresses, &SupportedDataLevels::Bronze).await
    }

}