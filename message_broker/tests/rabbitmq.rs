use tokio::runtime::Runtime;
use message_broker::*;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use std::{thread, time};


#[test]
fn test_rabbitmq_connection() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let conn = rabbit_connection("amqp://localhost:5672/%2f")
            .await;

        info!(configuration=?conn.configuration(), "CONNECTED");
        info!(status=?conn.status(), "STATUS");

        let status = conn.status();

        assert!(status.connected());
    });
}

#[test]
fn test_create_rmq_channel() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let channel = create_rmq_channel("amqp://localhost:5672/%2f")
            .await
            .expect("Failed to create channel");

        info!(channel=?channel, "CHANNEL");

        assert!(channel.id() == 1);
    });
}

#[test]
fn test_declare_exchange() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        declare_exchange("amqp://localhost:5672/%2f", &"test_exchange".to_string(), &lapin::ExchangeKind::Direct)
            .await
            .expect("Failed to declare exchange");


        let conn = rabbit_connection("amqp://localhost:5672/%2f")
        .await;

        info!(configuration=?conn.configuration(), "CONNNN!");

        let topology = conn.topology();

        thread::sleep(time::Duration::from_secs(1));


        info!(topology=?topology.exchanges, "TOPOLOGY");

        assert!(true);
    });

}


pub fn setup_test_tracing() {
    // You can use environment variables to control the tracing level.
    // For example, you can run your tests with `RUST_LOG=trace cargo test` to show `trace` and `debug` messages.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
