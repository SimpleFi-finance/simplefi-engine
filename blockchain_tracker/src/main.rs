use chrono::{NaiveDateTime, Datelike};
use lapin::{options::{BasicQosOptions, BasicConsumeOptions}, types::FieldTable};
use serde_json::json;
use third_parties::{mongo::{lib::bronze::blocks::setters::save_blocks, MongoConfig, Mongo}, broker::{create_rmq_channel, declare_rmq_queue, bind_queue_to_exchange, publish_rmq_message, declare_exchange}};
use tungstenite::{connect, Message};

use crate::types::NewHeadsEvent;
mod types;

#[tokio::main]
async fn main() {
    // connects to node wss endpoint and listens to new blocks (can store block data as it comes in)
    // todo load settings and select chain

    // load wss from chain and start syncing

    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_subscribe",
        "params": ["newHeads", true]
    });

    let request_str = serde_json::to_string(&request).unwrap();

    let wss_url = String::from("wss://mainnet.infura.io/ws/v3/__infura_key__");

    let (mut socket, _response) = connect(&wss_url).expect("Can't connect");
    socket.write_message(Message::Text(request_str)).unwrap();
    let mongo_config = MongoConfig {
        uri: String::from("mongodb://localhost:27017"),
        database: String::from("blocks_bronze"),
    };

    let db = Mongo::new(&mongo_config).await.unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = msg.into_text().unwrap();
        // println!("{}", msg);
        let result: NewHeadsEvent = serde_json::from_str(&msg).unwrap();


        let queue_name = String::from("ethereum_blocks");
        let exchange_name = String::from("ethereum_blocks_exchange");

        let rmq_uri = String::from("amqp://guest:guest@localhost:5672");

        let channel = create_rmq_channel(&rmq_uri).await.unwrap();

        let kind = lapin::ExchangeKind::Direct;

        declare_exchange(&rmq_uri, &exchange_name, &kind)
            .await
            .expect("Failed to declare exchange");

        channel.basic_qos(1, BasicQosOptions{global:true}).await.unwrap();
        declare_rmq_queue(&queue_name, &channel)
            .await
            .expect("Failed to declare queue");

        let routing_key = String::from("ethereum_blocks");

        bind_queue_to_exchange(&queue_name, &exchange_name, &routing_key, &channel)
            .await
            .expect("Failed to bind queue");

        // either save and push number to queue or push data to queue
        if result.params.is_some() {
            let mut block = result.params.unwrap().result.unwrap();

            // todo push to queue

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
