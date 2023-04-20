use abi_discovery::helpers::process_abi;
// This binary should be listening a rabbit queue and when a message is received, it should
// call the etherscan api to get the contract abi and save it in the database.
//
use lapin::{ Error, message::Delivery };
use log::{ info, debug, error };
use shared_utils::logger::init_logging;
use std::sync::{Arc, Mutex};
use tokio::time::{timeout, Duration, sleep};

use abi_discovery::settings::load_settings;
use third_parties::broker::{create_rmq_channel, process_queue_with_rate_limit, publish_rmq_message};
use third_parties::http::etherscan::get_abi;

async fn produce_messages(
    exchange: &String,
    routing_key: &String,
    channel: &lapin::Channel,
) -> Result<(), Error> {
    let contracts = vec![
        "0x00000000000001ad428e4906aE43D8F9852d0dD6",
        "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
        "0x495f947276749Ce646f68AC8c248420045cb7b5e",
        "0xe8129d05532340cA156d9f146a28F68AcD96e80D",
        "0xDEF171Fe48CF0115B1d80b88dc8eAB59176FEe57",
        "0x0c7Ad07b985405C3f74d39d090a5916469B56f25",
        "0xb584D4bE1A5470CA1a8778E9B86c81e165204599",
        "0xdAC17F958D2ee523a2206206994597C13D831ec7",
        "0xAf5191B0De278C7286d6C7CC6ab6BB8A73bA2Cd6",
        "0x0000000000A39bb272e79075ade125fd351887Ac",
        "0x000000000000Ad05Ccc4F10045630fb830B95127",
        "0x643388199C804c593cA2aaE56E2C150b8e7A5876",
        "0xEf1c6E67703c7BD7107eed8303Fbe6EC2554BF6B",
    ];

    for contract in contracts {
        publish_rmq_message(
            exchange,
            routing_key,
            &contract.as_bytes(),
            channel,
        )
        .await
        .expect("Failed to publish message");
    }

    Ok(())
}

async fn handle_message(
    delivery: Arc<Delivery>,
    counter: usize,
    key: String,
) -> Result<(), Error> {
    let contract_address = String::from_utf8_lossy(&delivery.data).to_lowercase();

    debug!("Message data: {}", contract_address);

    let keys = key.split(',').collect::<Vec<&str>>();

    let etherscan_key = keys[counter % keys.len()];

    debug!("Key: {}", etherscan_key);

    let abi = timeout(
        Duration::from_secs(30),
        get_abi(&contract_address, &etherscan_key),
    ).await.expect("Failed to get ABI from etherscan");

    if abi.is_err() {
        error!("Error: {:?}", abi.err());
    } else {
        let response = abi.unwrap(); // .unwrap();

        // TODO: Saved in mongo
        process_abi(&contract_address, &response).await;


        // TODO: Add address to tracked addresses in redis set
    }


    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let settings = load_settings().expect("Failed to load settings");
    let rabbit_uri = settings.rabbit_mq_url.to_string();
    let queue_name = settings.rabbit_exchange_name.to_string();
    let exchange_name = format!("{}_exchange", queue_name);
    let routing_key = String::from("abi_discovery");
    let etherscan_keys = settings.etherscan_api_keys;


    info!("Rabbit URI: {}", rabbit_uri);
    info!("Rabbit Queue Name: {}", queue_name);
    info!("Rabbit Exchange Name: {}", exchange_name);
    info!("Router key: {}", routing_key);
    info!("Etherscan keys: {:?}", etherscan_keys);

    // max per second
    let max_reads_per_second = 1; // etherscan_keys.split(",").count() * 2;
    info!("Max reads per second {:?}", max_reads_per_second);

    // It should be per second but we are going to give 2 seconds
    let rate_limit_duration = Duration::from_secs(10);
    info!("Rate limit duration {:?}", rate_limit_duration);

    let counter = Arc::new(Mutex::new(0));

    // We define the handler that will be called when a message is received.
    // The handler will be called for each message received.
    let handler = move |delivery: Delivery, current_count: usize| {
        let message_data = String::from_utf8_lossy(&delivery.data);

        debug!("Message data: {}", message_data);
        debug!("Current count: {}", current_count);

        let cloned_keys: String = String::from(&etherscan_keys);
        let cloned_delivery = Arc::new(delivery);
        let cloned_counter = current_count.clone();

        let fut = async move {
            handle_message(cloned_delivery, cloned_counter, cloned_keys).await.expect("Failed to handle message");
        };

        tokio::spawn(fut);
    };

    let channel = create_rmq_channel("amqp://localhost:5672/%2f")
                .await
                .expect("Failed to create channel");

    debug!("Channel created");

    produce_messages(&exchange_name, &routing_key, &channel)
        .await
        .expect("Failed to produce messages");

    debug!("Messages produced");

    let result = process_queue_with_rate_limit(
        &channel,
        &queue_name,
        max_reads_per_second,
        rate_limit_duration,
        counter,
        handler,
    ).await;

    debug!("Result: {:?}", result);

    Ok(())
}
