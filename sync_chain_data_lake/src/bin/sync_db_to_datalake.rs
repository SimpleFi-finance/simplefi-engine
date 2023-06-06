/*
    this scripts reads from a staging mongodb database and digests the data to a data lake hosted in GCP.

    it digests data on a given interval, currently day. it will read up to the last full available day
 */

use settings::load_settings;

#[tokio::main]
async fn main() {
    let global_settings = load_settings().expect("Failed to load settings");
    /*
        listen to pubsub to get latest set of data available in db
        read from db in stream, store in parquet file in data lake
        delete from db
    */    
}
