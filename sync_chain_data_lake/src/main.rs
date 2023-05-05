use datafusion::{prelude::{SessionContext, ParquetReadOptions}, arrow::json::writer::record_batches_to_json_rows};
use object_store::{gcp::GoogleCloudStorageBuilder, path::Path, ObjectStore};
use settings::load_settings;
use url::Url;
use std::sync::Arc;
use futures::{stream::StreamExt, TryStreamExt};

#[tokio::main]
async fn main() {
    let glob_settings = load_settings().expect("Failed to load settings");
    let gcp_bucket = glob_settings.cloud_bucket;
    let account_file_path = glob_settings.gooogle_service_account_file.to_str().unwrap().to_string();
    
    let data_path: Path = "ethereum/bronze/logs".try_into().unwrap();

    // load object store and get dir, get latest day partition, if none start from blocknumber 0 or given, else start from the latest block found

    let object_store = GoogleCloudStorageBuilder::new()
        .with_service_account_path(account_file_path)
        .with_bucket_name(gcp_bucket.clone())
        .build()
        .unwrap();

    let objs = Arc::new(object_store);
    let gcs = objs.clone();

    let mut list_stream = gcs
        .list(Some(&data_path))
        .await
        .expect("Error listing files");

    let mut year = 0;
    let mut year_dirs = vec![];

    while let Some(meta) = list_stream.next().await {
        let meta = meta.expect("Error listing");

        if meta.location.to_string().contains(".DS_Store") {
            println!("object.name: {:?}", meta.location);
        } else {
            let path = meta.location.to_string();
            let date = path.split("/").collect::<Vec<&str>>();
            let year_path = date[3].split("=").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            let month_path = date[4].split("=").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            let day_path = date[5].split("=").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

            if year < year_path {
                year = year_path;
                year_dirs = vec![];
            }

            if year == year_path {
                year_dirs.push((month_path, day_path, date[6].to_string()));
            }
        }
    }

    year_dirs.sort_by(|a, b| a.0.cmp(&b.0));
    let month = year_dirs[year_dirs.len() - 1].0;

    let mut filtered =year_dirs.into_iter().filter(|a| a.0 == month).collect::<Vec<(i32, i32, String)>>();
    filtered.sort_by(|a,b| a.1.cmp(&b.1));
    let day = filtered[filtered.len() - 1].1;

    let last_partition = format!("{}/year={}/month={}/day={}/{}", data_path.to_string(), year, month, day, filtered[filtered.len() - 1].2);

    // load the partition and get latest block number
    let path = format!("gs://{}/{}", gcp_bucket, last_partition);
    let gcs_url = Url::parse(&path).unwrap();
    let ctx = SessionContext::new();
    let gcs = objs.clone();

    ctx
        .runtime_env()
        .register_object_store(&gcs_url, gcs);

    ctx.register_parquet("logs", &path, ParquetReadOptions::default())
        .await.unwrap();

    println!("table registered");
    let start_block_query = "SELECT block_number FROM logs ORDER BY block_number DESC LIMIT 1".to_string();

    let res = ctx.sql(&start_block_query).await.unwrap();

    let latest_block_saved = res.collect().await.unwrap();

    let records = record_batches_to_json_rows(&latest_block_saved).unwrap();
    let latest_block_number = records[0]["block_number"].as_i64().unwrap();
    // todo add exclusion if no latest block found

    
    println!("latest_block_saved: {:?}", latest_block_number);



}
