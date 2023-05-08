// This binary should be listening a rabbit queue and when a message is received, it should
// call the etherscan api to get the contract abi and save it in the database.
//
use lapin::{message::Delivery, Error};
use log::{debug, error, info};
use shared_utils::logger::init_logging;
use std::sync::{Arc, Mutex};
use third_parties::redis::{add_to_set, connect};
use tokio::time::{timeout, Duration};

use abi_discovery::{helpers::process_abi_json, settings::load_settings};
use third_parties::broker::{create_rmq_channel, process_queue_with_rate_limit};
use third_parties::http::etherscan::get_abi;

async fn handle_message(
    delivery: Arc<Delivery>,
    counter: usize,
    key: String,
) -> Result<(), Error> {
    let contract_address = String::from_utf8_lossy(&delivery.data).to_lowercase();

    let keys = key.split(',').collect::<Vec<&str>>();

    let etherscan_key = keys[counter % keys.len()];

    let abi = timeout(
        Duration::from_secs(30),
        get_abi(&contract_address, &etherscan_key),
    )
        .await
        .expect("Failed to get ABI from etherscan");

    if abi.is_err() {
        error!("Error: {:?}", abi.err());


    } else {
        let response = match abi {
            Ok(response) => response,
            Err(err) => {
                error!("Error: {:?}", err);

                return Ok(());
            }
        };

        let result = process_abi_json(&contract_address, &response).await;

        if result == false {
            debug!("contract: {}. ABI already exists", &contract_address);
        } else {
            let mysettings = load_settings().expect("Failed to load settings");

            let redis_uri = mysettings.redis_uri.to_string();
            let redis_tracked_addresses_set = mysettings.redis_tracked_addresses_set.to_string();

            let mut con = connect(&redis_uri.as_str())
                .await
                .expect("Failed to connect to redis");

            add_to_set(&mut con, &redis_tracked_addresses_set, &contract_address)
                .await
                .expect("Failed to add to redis set");

            info!("ABI added to redis set successfully")
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let settings = load_settings().expect("Failed to load settings");

    // Rabbit MQ Settings
    let rabbit_uri = settings.rabbit_mq_url.to_string();
    let queue_name = settings.rabbit_exchange_name.to_string();
    let exchange_name = format!("{}_exchange", queue_name);
    let routing_key = String::from("abi_discovery");
    let etherscan_keys = settings.etherscan_api_keys;

    // Log settings
    info!("Setting rabbit mq connection");

    debug!("Rabbit URI: {} \n Rabbit Queue Name: {} \n  Rabbit Exchange Name: {} \n Router key: {} \n Etherscan keys: {:?}", rabbit_uri, queue_name, exchange_name, routing_key, etherscan_keys);

    let max_reads_per_second = etherscan_keys.split(",").count() * 2;

    debug!("Max reads per second {:?}", max_reads_per_second);

    let rate_limit_duration = Duration::from_secs(2);

    debug!("Rate limit duration {:?}", rate_limit_duration);

    let counter = Arc::new(Mutex::new(0));

    let handler = move |delivery: Delivery, current_count: usize| {
        let cloned_keys: String = String::from(&etherscan_keys);
        let cloned_delivery = Arc::new(delivery);
        let cloned_counter = current_count.clone();

        let fut = async move {
            handle_message(cloned_delivery, cloned_counter, cloned_keys)
                .await
                .expect("Failed to handle message");
        };

        tokio::spawn(fut);
    };

    let channel = create_rmq_channel("amqp://localhost:5672/%2f")
        .await
        .expect("Failed to create channel");

    // debug!("Channel created");

    /* produce_messages(&exchange_name, &routing_key, &channel)
    .await
    .expect("Failed to produce messages"); */

    // debug!("Messages produced");

    let result = process_queue_with_rate_limit(
        &channel,
        &queue_name,
        max_reads_per_second,
        rate_limit_duration,
        counter,
        handler,
    )
    .await;

    debug!("Result possible error: {:?}", result);

    Ok(())
}

/*
async fn produce_messages(
    exchange: &String,
    routing_key: &String,
    channel: &lapin::Channel,
) -> Result<(), Error> {
    let contracts = vec![
        "0x6982508145454ce325ddbe47a25d4ec3d2311933",
        /*"0x00000000000001ad428e4906aE43D8F9852d0dD6",
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
        "0xEf1c6E67703c7BD7107eed8303Fbe6EC2554BF6B", */
    ];

    for contract in contracts {
        publish_rmq_message(exchange, routing_key, &contract.as_bytes(), channel)
            .await
            .expect("Failed to publish message");
    }

    Ok(())
} */
