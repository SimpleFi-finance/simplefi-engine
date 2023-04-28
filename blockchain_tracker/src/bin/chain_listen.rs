use blockchain_tracker::{types::NewHeadsEvent, settings::load_settings};
use chrono::{Datelike, NaiveDateTime};
use lapin::options::BasicQosOptions;
use serde_json::json;
use settings::load_settings as load_global_settings;
use shared_utils::logger::init_logging;
use third_parties::{
    broker::{
        bind_queue_to_exchange, create_rmq_channel, declare_exchange, declare_rmq_queue,
        publish_rmq_message,
    },
    mongo::{lib::bronze::blocks::{setters::save_blocks, types::Block}, Mongo, MongoConfig},
};
use tungstenite::{connect, Message};

#[tokio::main]
async fn main() {
    // connects to node wss endpoint and listens to new blocks (can store block data as it comes in)
    // todo load settings and select chain
    let glob_settings = load_global_settings().unwrap();
    let local_settings = load_settings().unwrap();
    init_logging();
    // load wss from chain and start syncing

    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_subscribe",
        "params": ["newHeads", true]
    });

    let request_str = serde_json::to_string(&request).unwrap();

    let wss_url = String::from(format!("{}{}",glob_settings.infura_mainnet_ws, glob_settings.infura_token ));

    let (mut socket, _response) = connect(&wss_url).expect("Can't connect");
    socket.write_message(Message::Text(request_str)).unwrap();

    let mongo_config = MongoConfig {
        uri: local_settings.mongodb_uri,
        database: local_settings.mongodb_database_name,
    };

    let db = Mongo::new(&mongo_config).await.unwrap();

    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = msg.into_text().unwrap();
        let result: NewHeadsEvent = serde_json::from_str(&msg).unwrap();

        // load from localsettings
        let queue_name = local_settings.new_blocks_queue_name.clone();
        let exchange_name = format!("{}_{}", String::from("ethereum"), local_settings.new_block_exchange_name.clone());

        let rmq_uri = local_settings.rabbit_mq_url.clone();

        let channel = create_rmq_channel(&rmq_uri).await.unwrap();

        let kind = lapin::ExchangeKind::Direct;

        declare_exchange(&rmq_uri, &exchange_name, &kind)
            .await
            .expect("Failed to declare exchange");

        channel
            .basic_qos(1, BasicQosOptions { global: true })
            .await
            .unwrap();
        declare_rmq_queue(&queue_name, &channel)
            .await
            .expect("Failed to declare queue");

        let routing_key = format!("{}_{}", String::from("ethereum"), String::from("blocks"));

        bind_queue_to_exchange(&queue_name, &exchange_name, &routing_key, &channel)
            .await
            .expect("Failed to bind queue");

        if result.params.is_some() {
            let block = result.params.unwrap().result.unwrap();

            let bytes_serde = serde_json::to_vec(&block.number).unwrap();

            let (_send_to_queue, _save_to_db) = tokio::join!(
                publish_rmq_message(&exchange_name, &routing_key, &bytes_serde, &channel),
                async {
                    let date = NaiveDateTime::from_timestamp_opt(block.timestamp, 0).unwrap();
                    let ts = date.timestamp_micros();
                    
                    let block = Block {
                        timestamp: ts,
                        year: date.year() as i16,
                        month: date.month() as i8,
                        day: date.day() as i8,
                        number: block.number,
                        hash: block.hash,
                        parent_hash: block.parent_hash,
                        nonce: block.nonce,
                        transactions_root: block.transactions_root,
                        state_root: block.state_root,
                        receipts_root: block.receipts_root,
                        miner: block.miner,
                        difficulty: block.difficulty,
                        mix_hash: block.mix_hash,
                        extra_data: block.extra_data,
                        logs_bloom: block.logs_bloom,
                        gas_limit: block.gas_limit,
                        gas_used: block.gas_used,
                        uncles_hash: block.uncles_hash,
                        base_fee_per_gas: block.base_fee_per_gas,
                        withdrawals_root: block.withdrawals_root,
                    };

                    save_blocks(&db, vec![block]).await.unwrap();
                }
            );
        }
    }
}
