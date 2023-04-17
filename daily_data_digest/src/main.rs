use settings::load_settings;
use third_parties::mongo::{MongoConfig, Mongo};

#[tokio::main]
async fn main() {
    // connects to mongo instance, gets all data type for the day and saves into parquet and saves to given bucket/storage
    // todo add selector for data type from envs
    let datatype = "bronze_logs";
    // load mongo instance
    let settings = load_settings().expect("Failed to load settings");

    let mongo_config = MongoConfig {
        uri: settings.mongodb_uri.to_string(),
        database: "bronze_logs".to_string(),
    };

    // Create a new MongoDB client

    let client_db = Mongo::new(&mongo_config).await.expect("Failed to create mongo Client");


    
}
