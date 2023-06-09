/*
    listens to stream/pubsub to get the range of blockNumbers to index to create partitions, saves the data in mongo staging as it comes along and once done it notifies the parquet pubsub to convert the data from mongo to parquet data lake

    or

    start getting the data and save it into the parquet directly without using a staging mongo db

 */

#[tokio::main]
async fn main() {
    todo!("Implement sync process")
}