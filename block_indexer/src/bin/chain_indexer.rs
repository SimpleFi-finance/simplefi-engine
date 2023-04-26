use std::{collections::HashMap, time::Instant, str::FromStr};

use block_indexer::{utils::{get_block_logs, get_block_with_txs}, settings::load_settings};
use chrono::{Datelike, NaiveDateTime};
use ethabi::{Contract, Event, RawLog, ethereum_types::H256, Bytes};
use futures::{StreamExt, TryStreamExt};
use grpc_server::{client::AbiDiscoveryClient};
use lapin::{options::BasicConsumeOptions, types::FieldTable};
use rayon::{iter::ParallelIterator};
use rayon::prelude::IntoParallelRefIterator;
use settings::load_settings as load_global_settings;
use third_parties::{
    broker::create_rmq_channel,
    mongo::{
        lib::bronze::{
            logs::{setters::save_logs, types::Log},
            txs::setters::save_txs,
        },
        Mongo, MongoConfig,
    },
};

#[tokio::main]
async fn main() {
    // listens to the queue of blocks minted and gets logs and txs and saves in mongo
    // let mut interface_hashmap = HashMap::new();

    let global_settings = load_global_settings().unwrap();
    let local_settings = load_settings().unwrap();

    let queue_name = local_settings.new_blocks_queue_name.clone();
    let consumer_name = format!("{}_{}", String::from("ethereum"), String::from("block_indexer"));
    let rmq_uri = local_settings.rabbit_mq_url.clone();
    let channel = create_rmq_channel(&rmq_uri).await.unwrap();

    let consumer = channel
        .basic_consume(
            &queue_name,
            &consumer_name,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to start consumer");

    println!("Waiting for messages...");
    let mut consumer_stream = consumer.into_stream();

    let provider_url = format!("{}{}", global_settings.infura_mainnet_rpc, global_settings.infura_token);

    let db_config = MongoConfig {
        uri: local_settings.mongodb_uri.clone(),
        database: local_settings.mongodb_database_name.clone(),
    };

    let db = Mongo::new(&db_config).await.unwrap();

    while let Some(delivery) = consumer_stream.next().await {
        let delivery_data = delivery.unwrap();
        let block: i64 = serde_json::from_slice(&delivery_data.data).unwrap();
        println!("Got message: {:?}", block);

        let now = Instant::now();
        // channel.basic_ack(delivery_data.delivery_tag, BasicAckOptions::default()).await?;
        // get logs and txs and save in mongo
        let u64_block = block as u64;
        let (logs, block) = tokio::join!(
            get_block_logs(provider_url.clone(), &u64_block, &u64_block),
            get_block_with_txs(provider_url.clone(), &u64_block)
        );

        let logs_by_address = logs
            .unwrap()
            .par_iter()
            .fold(||HashMap::new(), |mut acc, log| {
                let mut log = log.clone();
                let ts = block.0.clone().unwrap().timestamp.clone();
                let date = NaiveDateTime::from_timestamp_opt(ts, 0).unwrap();

                log.timestamp = Some(date.timestamp_micros());
                log.year = Some(date.year() as i16);
                log.month = Some(date.month() as i8);
                log.day = Some(date.day() as i8);
                // acc.entry(log.address.clone()).or_insert(vec![]).push(log.clone());
                if acc.contains_key(&log.address.clone().unwrap()) {
                    let logs: &Vec<Log> = acc.get(&log.address.clone().unwrap()).unwrap();
                    let mut logs = logs.clone();
                    logs.push(log.clone());
                    acc.insert(log.address.clone().unwrap(), logs);
                } else {

                    acc.insert(log.address.clone().unwrap(), vec![log]);
                }
                acc
            })
            .reduce(||HashMap::new(), |mut acc, hm| {
                for (key, value) in hm {
                    acc.entry(key).or_insert(vec![]).extend(value);
                }
                acc
            });
        
        let unique_addresses = Vec::from_iter(logs_by_address.keys().cloned());

        // todo set client dtnamically
        let mut abi_discovery_client = AbiDiscoveryClient::new("http://[::1]:50051".to_string()).await;
        let abis_addresses = abi_discovery_client.get_addresses_abi_json(unique_addresses).await;
        let abis_response = abis_addresses.into_inner();

        for i in abis_response.addresses_abi {
            let data = i;
            let abi = &data.abi;
            let contract: Contract = serde_json::from_str(abi.as_str()).unwrap();

            let mut eventhm = HashMap::new();
            for event in &contract.events {
                println!("event: {:?}", event);
                let e = event.1[0].clone();

                eventhm.insert(e.signature(), e);
            }


            println!("address: {:?}", &data.address);
            let logs_of_address = logs_by_address.get(&data.address).unwrap();

            logs_of_address.par_iter().for_each(|log| {
                let log = log.clone();
                let h256_topics = log.topics.iter().map(|t| H256::from_str(t).unwrap()).collect::<Vec<H256>>();
                let bytes = hex::decode(log.data.clone().unwrap().strip_prefix("0x").unwrap()).unwrap();
                let raw_log = RawLog {
                    topics: h256_topics.clone(),
                    data: bytes,
                };

                let event = eventhm.get(&h256_topics[0]);
                match event {
                    Some(event) => {
                        let decoded_log = event.parse_log(raw_log);
                        match decoded_log {
                            Ok(decoded_log) => {
                                // todo push to array to save logs in mongo
                                println!("decoded_log: {:?}", decoded_log);
                            },
                            Err(e) => {
                                println!("error: {:?}", e);
                            }
                        }
                    },
                    None => {
                        println!("event not found");
                    }
                }
            });
        }

        println!("Time elapsed is: {:?}ms", now.elapsed().as_millis());

        // let (_, _) = tokio::join!(
        //     save_logs(&db, logs),
        //     save_txs(&db, block.1.unwrap())
        // );
    }
}
