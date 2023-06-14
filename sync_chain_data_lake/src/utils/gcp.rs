use object_store::{gcp::{GoogleCloudStorageBuilder, GoogleCloudStorage}, ObjectStore, path::Path};
use settings::load_settings;
use futures::{stream::StreamExt, TryStreamExt};

pub fn gcp_object_store (bucket_name: &String) -> GoogleCloudStorage {
    let glob_settings = load_settings().expect("Failed to load settings");
    let account_file_path = glob_settings.gooogle_service_account_file.to_str().unwrap().to_string();

    let object_store = GoogleCloudStorageBuilder::new()
        .with_service_account_path(account_file_path)
        .with_bucket_name(bucket_name.to_string())
        .build()
        .unwrap();

    object_store
}

pub async fn get_latest_partition_in_bucket(gcs_os: &GoogleCloudStorage, data_path: &Path) -> Result<String, Box<dyn std::error::Error>> {

    let mut list_stream = gcs_os.clone()
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

    Ok(last_partition)
}
