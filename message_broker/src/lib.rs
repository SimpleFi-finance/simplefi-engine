use lapin::{
    options::*,
    types::FieldTable,
    Connection as LapinConnection,
    Channel as LapinChannel,
    ConnectionProperties,
    Error,
    Queue
};

pub async fn rabbit_connection(uri: &str) -> LapinConnection {
    LapinConnection::connect(
        uri,
        ConnectionProperties::default(),
    )
    .await
    .expect("Failed to connect to RabbitMQ")
}

pub async fn create_rmq_channel(uri: &str) -> Result<LapinChannel, Error> {
    let conn = rabbit_connection(uri).await;
    let channel = conn.create_channel().await;

    return channel;
}

pub async fn declare_exchange(uri: &str, exchange: &String, kind: &lapin::ExchangeKind) -> Result<(), Error> {
    let channel = create_rmq_channel(uri).await.expect("Failed to create channel");
    let exchange_kind = kind.clone();
    channel
        .exchange_declare(
            exchange,
            exchange_kind,
            ExchangeDeclareOptions::default(),
            FieldTable::default()
        )
        .await.expect("Failed to declare exchange");

    Ok(())
}

pub async fn bind_queue_to_exchange(queue: &String, exchange: &String, routing_key: &String, channel: &lapin::Channel) -> Result<(), Error> {
    channel
        .queue_bind(
            queue,
            exchange,
            routing_key,
            QueueBindOptions::default(),
            FieldTable::default()
        )
        .await.expect("Failed to bind queue");

    Ok(())
}

pub async fn declare_rmq_queue(name: &String, channel: &lapin::Channel) -> Result<Queue, Error> {
    let queue = channel
        .queue_declare(
            name,
            QueueDeclareOptions::default(),
            FieldTable::default()
        )
        .await;

    return queue;
}

pub async fn publish_rmq_message(exchange: &String, routing_key: &String, message: &[u8], channel: &lapin::Channel) -> Result<(), Error> {
    let message_properties = lapin::BasicProperties::default();
    channel
        .basic_publish(
            exchange,
            routing_key,
            BasicPublishOptions::default(),
            &message.to_vec(),
            message_properties,
        )
        .await.expect("Failed to publish message");

    Ok(())
}
