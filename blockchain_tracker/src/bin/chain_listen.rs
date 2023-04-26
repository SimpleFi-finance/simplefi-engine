use blockchain_tracker::{types::NewHeadsEvent, settings::load_settings};
use chrono::{Datelike, NaiveDateTime};
use lapin::options::BasicQosOptions;
use serde_json::json;
use settings::load_settings as load_global_settings;
use third_parties::{
    broker::{
        bind_queue_to_exchange, create_rmq_channel, declare_exchange, declare_rmq_queue,
        publish_rmq_message,
    },
    mongo::{lib::bronze::blocks::setters::save_blocks, Mongo, MongoConfig},
};
use tungstenite::{connect, Message};

#[tokio::main]
async fn main() {
    // connects to node wss endpoint and listens to new blocks (can store block data as it comes in)
    // todo load settings and select chain
    let glob_settings = load_global_settings().unwrap();
    let local_settings = load_settings().unwrap();
    // load wss from chain and start syncing

    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_subscribe",
        "params": ["newHeads", true]
    });

    let request_str = serde_json::to_string(&request).unwrap();
    
    //todo load global settings

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
            let mut block = result.params.unwrap().result.unwrap();

            let bytes_serde = serde_json::to_vec(&block.number).unwrap();

            let (_send_to_queue, _save_to_db) = tokio::join!(
                publish_rmq_message(&exchange_name, &routing_key, &bytes_serde, &channel),
                async {
                    let date = NaiveDateTime::from_timestamp_opt(block.timestamp, 0).unwrap();
                    let ts = date.timestamp_micros();
                    block.timestamp = ts;
                    block.year = Some(date.year() as i16);
                    block.month = Some(date.month() as i8);
                    block.day = Some(date.day() as i8);
                    save_blocks(&db, vec![block]).await.unwrap();
                }
            );
        }
    }
}
