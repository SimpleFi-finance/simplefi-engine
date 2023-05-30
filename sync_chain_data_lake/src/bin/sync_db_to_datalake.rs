/*
    this scripts reads from a staging mongodb database and digests the data to a data lake hosted in GCP.

    it digests data on a given interval, currently day. it will read up to the last full available day
 */

use settings::load_settings;

#[tokio::main]
async fn main() {
    let global_settings = load_settings().expect("Failed to load settings");

    // either use a pubsub to get the latest partition ready to be digested or get the latest partition from the bucket and find the next one available

    // read the data form the partition, transform into parquet and save to the data lake

    // delete the data from the db
    
}
