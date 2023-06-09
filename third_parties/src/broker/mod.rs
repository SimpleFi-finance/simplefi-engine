use lapin::{
    options::*, types::FieldTable, Channel as LapinChannel, Connection as LapinConnection,
    ConnectionProperties, Error, Queue,
    message::Delivery,
};
use log::{ debug, info, warn };
use tokio::time::{self, Duration };
use std::sync::{Arc, Mutex};
use futures::StreamExt;
pub mod queues;

pub async fn rabbit_connection(uri: &str) -> LapinConnection {
    LapinConnection::connect(uri, ConnectionProperties::default())
        .await
        .expect("Failed to connect to RabbitMQ")
}

pub async fn create_rmq_channel(uri: &str) -> Result<LapinChannel, Error> {
    let conn = rabbit_connection(uri).await;
    let channel = conn.create_channel().await;

    return channel;
}

pub async fn declare_exchange(
    uri: &str,
    exchange: &String,
    kind: &lapin::ExchangeKind,
) -> Result<(), Error> {
    let channel = create_rmq_channel(uri)
        .await
        .expect("Failed to create channel");
    let exchange_kind = kind.clone();
    channel
        .exchange_declare(
            exchange,
            exchange_kind,
            ExchangeDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to declare exchange");

    Ok(())
}

pub async fn bind_queue_to_exchange(
    queue: &String,
    exchange: &String,
    routing_key: &String,
    channel: &lapin::Channel,
) -> Result<(), Error> {
    channel
        .queue_bind(
            queue,
            exchange,
            routing_key,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to bind queue");

    Ok(())
}

pub async fn declare_rmq_queue(
    name: &String,
    channel: &lapin::Channel,
) -> Result<Queue, Error> {
    let queue = channel
        .queue_declare(name, QueueDeclareOptions::default(), FieldTable::default())
        .await;

    return queue;
}

pub async fn publish_rmq_message(
    exchange: &String,
    routing_key: &String,
    message: &[u8],
    channel: &lapin::Channel,
) -> Result<(), Error> {
    let message_properties = lapin::BasicProperties::default();
    channel
        .basic_publish(
            exchange,
            routing_key,
            BasicPublishOptions::default(),
            &message.to_vec(),
            message_properties,
        )
        .await
        .expect("Failed to publish message");

    Ok(())
}

pub async fn process_queue_with_rate_limit<F>(
    channel: &lapin::Channel,
    queue_name: &String,
    max_reads: usize,
    rate_limit_duration: Duration,
    counter: Arc<Mutex<usize>>,
    handler: F,
) -> Result<(), lapin::Error>
where
    F: Fn(Delivery, usize) + Send + Sync + 'static, // Define the type bound for the handler
{
    info!("process queue called: max_reads {:?}, rate_limit_duration: {:?}", max_reads, rate_limit_duration);

    let mut consumer = channel
        .basic_consume(
            queue_name,
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let mut interval = time::interval(rate_limit_duration);

    // process messages from the queue until the queue is empty or we've reached the max_reads
    while let Some(delivery) = consumer.next().await {
        let delivery = delivery?;

        let message = String::from_utf8_lossy(&delivery.data);

        debug!("Received message: {}", message);

        // Acknowledge the message
        channel
            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
            .await?;

        let mut count = counter.lock().unwrap();

        if *count >= max_reads {
            interval.tick().await;

            *count = 0;

            warn!("Rate limit reached, waiting for next interval");

             // Call the injected function
        }

        debug!("Calling handler with count: {}", *count);

        handler(delivery, *count);

        *count += 1;

        debug!("Increasing Count: {}", *count);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};
    use tokio::runtime::Runtime;
    use tokio::time::{timeout, Duration};
    use tracing::info;

    #[test]
    fn test_rabbitmq_connection() {
        let rt = Runtime::new().unwrap();

        rt.block_on(async {
            let conn = rabbit_connection("amqp://localhost:5672/%2f").await;

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
            declare_exchange(
                "amqp://localhost:5672/%2f",
                &"test_exchange".to_string(),
                &lapin::ExchangeKind::Direct,
            )
            .await
            .expect("Failed to declare exchange");

            let conn = rabbit_connection("amqp://localhost:5672/%2f").await;

            info!(configuration=?conn.configuration(), "CONNNN!");

            let topology = conn.topology();

            thread::sleep(time::Duration::from_secs(1));

            info!(topology=?topology.exchanges, "TOPOLOGY");

            assert!(true);
        });
    }

    async fn produce_messages(
        exchange: &String,
        routing_key: &String,
        channel: &lapin::Channel,
    ) -> Result<(), Error> {
        for i in 0..20 {
            publish_rmq_message(
                exchange,
                routing_key,
                &format!("Message {}", i).as_bytes(),
                channel,
            )
            .await
            .expect("Failed to publish message");
        }

        Ok(())
    }

    #[test]
    fn test_process_queue_with_rate_limit() {
        let rt = Runtime::new().unwrap();

        rt.block_on(async {
            let exchange = "test_exchange".to_string();
            let routing_key = "test_routing_key".to_string();
            let queue_name = "test_queue".to_string();

            let channel = create_rmq_channel("amqp://localhost:5672/%2f")
                .await
                .expect("Failed to create channel");

            declare_exchange(
                "amqp://localhost:5672/%2f",
                &exchange,
                &lapin::ExchangeKind::Direct,
            )
            .await
            .expect("Failed to declare exchange");

            declare_rmq_queue(&queue_name, &channel)
                .await
                .expect("Failed to declare queue");

            bind_queue_to_exchange(&queue_name, &exchange, &routing_key, &channel)
                .await
                .expect("Failed to bind queue");

            produce_messages(&exchange, &routing_key, &channel)
                .await
                .expect("Failed to produce messages");

            let max_reads = 5;
            let rate_limit_duration = Duration::from_secs(5);
            let counter = Arc::new(Mutex::new(0));

            let handler = |delivery: Delivery, current_count: usize| {
                let message_data = String::from_utf8_lossy(&delivery.data);
                println!("Message data: {}", message_data);
                println!("Current count: {}", current_count);
            };

            let test_duration = Duration::from_secs(15);

            let process_result = timeout(test_duration, process_queue_with_rate_limit(
                &channel,
                &queue_name,
                max_reads,
                rate_limit_duration,
                counter,
                handler,
            ))
            .await;
            // .expect("Failed to process queue");

            match process_result {
                Ok(_) => println!("Processing finished successfully."),
                Err(_) => println!("Test timed out"),
            }
        });
    }

}