use log::info;

use abi_discovery::settings::load_settings;
use shared_utils::logger::init_logging;
use third_parties::broker::{publish_rmq_message, create_rmq_channel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let mysettings = load_settings().expect("Failed to load settings");

    let queue_name = mysettings.rabbit_exchange_name.to_string();
    let exchange_name = format!("{}_exchange", queue_name);
    let routing_key = String::from("abi_discovery");

    let channel = create_rmq_channel(&mysettings.redis_uri)
        .await
        .expect("Failed to create channel");

    // path to assets folder
    let path = "E:\\rust\\simplefi\\assets\\blockchains\\ethereum\\assets";

    // get all files in assets folder
    let files = std::fs::read_dir(path).unwrap();

    // for each file in assets folder print the file name
    for file in files {
        let file = file.unwrap().file_name();

        let filename_option = file.to_str();

        if filename_option.is_none() {
            continue;
        }

        let filename = filename_option.unwrap().to_lowercase();

        publish_rmq_message(&exchange_name, &routing_key, &filename.as_bytes(), &channel)
            .await
            .expect("Failed to publish message");

        info!("{:?} -> published", filename);
    }

    Ok(())
}
