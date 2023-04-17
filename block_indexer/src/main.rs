use chrono::{NaiveDateTime, Datelike};
use ethers::providers::{Provider, Http};
use lapin::{options::BasicConsumeOptions, types::FieldTable};
use rayon::prelude::IntoParallelRefIterator;
use third_parties::{broker::create_rmq_channel, mongo::{lib::bronze::{blocks::setters::save_blocks, logs::{setters::save_logs, types::Log}, txs::setters::save_txs}, MongoConfig, Mongo}};
use futures::{StreamExt, TryStreamExt};
mod utils;
use crate::utils::{get_block_logs, get_block_with_txs};
use rayon::iter::ParallelIterator;

#[tokio::main]
async fn main() {
    // listens to the queue of blocks minted and gets logs and txs and saves in mongo
    let queue_name = String::from("ethereum_blocks");
    let consumer_name = String::from("ethereum_blocks_consumer");
    let rmq_uri = String::from("amqp://guest:guest@localhost:5672");
    let channel = create_rmq_channel(&rmq_uri).await.unwrap();

    let consumer = channel
        .basic_consume(
            &queue_name,
            &consumer_name,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await.expect("Failed to start consumer");

    println!("Waiting for messages...");
    let mut consumer_stream = consumer.into_stream();

    let provider_url = String::from("https://mainnet.infura.io/v3/__infura_key__");

    let rpc_provider = Provider::<Http>::try_from(
        &provider_url,
    ).expect("could not instantiate HTTP Provider");


    let db_blocks_config = MongoConfig {
        uri: String::from("mongodb://localhost:27017"),
        database: String::from("blocks_bronze"),
    };

    let db_blocks = Mongo::new(&db_blocks_config).await.unwrap();

    let db_logs_config = MongoConfig {
        uri: String::from("mongodb://localhost:27017"),
        database: String::from("logs_bronze"),
    };

    let db_logs = Mongo::new(&db_logs_config).await.unwrap();

    let db_txs_config = MongoConfig {
        uri: String::from("mongodb://localhost:27017"),
        database: String::from("txs_bronze"),
    };

    let db_txs = Mongo::new(&db_txs_config).await.unwrap();

    while let Some(delivery) = consumer_stream.next().await {
        let delivery_data = delivery.unwrap();
        let block: i64 = serde_json::from_slice(&delivery_data.data).unwrap();
        println!("Got message: {:?}", block);
        // channel.basic_ack(delivery_data.delivery_tag, BasicAckOptions::default()).await?;
        // get logs and txs and save in mongo
        let u64_block = block as u64;
        let (logs,block) = tokio::join!(
            get_block_logs(provider_url.clone(), &u64_block, &u64_block),
            get_block_with_txs(provider_url.clone(), &u64_block)
        );

        let logs = logs.unwrap().par_iter().map(|log| {
            let mut log = log.clone();
            let ts = block.0.clone().unwrap().timestamp.clone();
            let date = NaiveDateTime::from_timestamp_opt(ts, 0).unwrap();

            log.timestamp = Some(date.timestamp_micros());
            log.year = Some(date.year() as i16);
            log.month = Some(date.month() as i8);
            log.day = Some(date.day() as i8);
            log
        }).collect::<Vec<Log>>();

        // todo add timestamps to txs and to logs
        let (_,_,_) = tokio::join!(
            save_blocks(&db_blocks, vec![block.0.unwrap()]),
            save_logs(&db_logs, logs),
            save_txs(&db_txs, block.1.unwrap())
        );

    }
}
