
// create fn main that uses third_parties helper to create a queue
use lapin;
use log::info;

use abi_discovery::settings::load_settings;
use shared_utils::logger::init_logging;
use third_parties::broker::{create_rmq_channel, declare_exchange, declare_rmq_queue, bind_queue_to_exchange};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    // Load rabbit url from settings with load_settings helper
    let mysettings = load_settings().expect("Failed to load settings");
    let rabbit_uri = mysettings.rabbit_mq_url.to_string();

    let queue_name = mysettings.rabbit_exchange_name.to_string();
    let exchange_name = format!("{}_exchange", queue_name);
    let routing_key = String::from("abi_discovery");

    info!("Rabbit URI: {}", rabbit_uri);
    info!("Rabbit Queue Name: {}", queue_name);
    info!("Rabbit Exchange Name: {}", exchange_name);

    let channel = create_rmq_channel(&rabbit_uri.as_str())
        .await
        .expect("Failed to create channel");

    // Create a queue with declare_exchange helper
    declare_exchange(
        &rabbit_uri.as_str(),
        &exchange_name,
        &lapin::ExchangeKind::Direct,
    ).await.expect("Failed to declare exchange");

    declare_rmq_queue(&queue_name, &channel)
        .await
        .expect("Failed to declare queue");

    bind_queue_to_exchange(&queue_name, &exchange_name, &routing_key, &channel)
        .await
        .expect("Failed to bind queue");

    Ok(())
}
