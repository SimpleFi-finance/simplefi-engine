use std::sync::Arc;

use datafusion::{prelude::{ParquetReadOptions, SessionContext}};
use object_store::{gcp::GoogleCloudStorageBuilder, ObjectStore, path::Path};
use settings::load_settings;
use futures::{stream::StreamExt, TryStreamExt};
use url::Url;

// given a range of dates, load the data from gcp, decode it, and write it to a file in gcp
#[tokio::main]
async fn main() {
    let glob_settings = load_settings().expect("Failed to load settings");   

    let gcp_bucket = glob_settings.cloud_bucket;
    let account_file_path = glob_settings.gooogle_service_account_file.to_str().unwrap().to_string();
    
    let object_store = GoogleCloudStorageBuilder::new()
        .with_service_account_path(account_file_path.clone())
        .with_bucket_name(gcp_bucket.clone())
        .build().unwrap();

    let data_path: Path = "ethereum/bronze/logs".try_into().unwrap();

    let objs = Arc::new(object_store);
    let gcs = objs.clone();

    let mut list_stream = gcs
        .list(Some(&data_path))
        .await
        .expect("Error listing files");
    
    while let Some(meta) = list_stream.next().await{
        let gcs = objs.clone();
        let bucket = gcp_bucket.clone();

        let ctx = SessionContext::new();
        let meta = meta.expect("Error listing");

        if meta.location.to_string().contains(".DS_Store") {
            println!("object.name: {:?}", meta.location);
        } else {
            let path = format!("gs://{}", bucket);
            let gcs_url = Url::parse(&path).unwrap();
            ctx
                .runtime_env()
                .register_object_store(&gcs_url, gcs);
    
            println!("Name: {}, size: {}", meta.location, meta.size);
            
            let path = format!("gs://{}/{}", bucket, meta.location.clone());
            ctx.register_parquet("logs", &path, ParquetReadOptions::default())
                .await.unwrap();
    
            let start_block_query = "SELECT block_number, timestamp FROM logs ORDER BY block_number".to_string();
            println!("table registered");
    
            let res = ctx.sql(&start_block_query).await.unwrap();
            println!("res: {:?}", res.count().await);
        }
    }
}