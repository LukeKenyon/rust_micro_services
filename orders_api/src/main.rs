use futures::StreamExt;
use lapin::{Channel, Connection, ConnectionProperties, options::*, types::FieldTable};

use lapin::options::QueueDeclareOptions;

use log::info;

pub async fn create_msg_connection() -> Result<Connection, lapin::Error> {
    let connection_url = std::env::var("RABBITMQ_URL")
        .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672/%2f".to_string());

    let connection_properties = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    let connection = Connection::connect(&connection_url, connection_properties).await?;

    Ok(connection)
}

pub async fn create_channel() -> Result<Channel, lapin::Error> {
    match create_msg_connection().await {
        Ok(connection) => {
            info!("Creating channel");
            let channel = connection.create_channel().await?;
            Ok(channel)
        }
        Err(err) => Err(err),
    }
}

pub async fn declare_queue(channel: &Channel, name: &str) {
    channel
        .queue_declare(
            name,
            QueueDeclareOptions {
                passive: true,
                durable: true,
                auto_delete: false,
                exclusive: false,
                nowait: false,
            },
            FieldTable::default(),
        )
        .await
        .expect("Failed to declare queue");
}

pub async fn consume_queue(channel: &Channel, queue_name: &str) {
    let mut consumer = channel
        .basic_consume(
            queue_name,
            "orders_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to start consumer");

    println!("Waiting for messages...");

    while let Some(result) = consumer.next().await {
        match result {
            Ok(delivery) => {
                let data = std::str::from_utf8(&delivery.data).unwrap();
                println!("Received message: {}", data);

                // Acknowledge the message
                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("Failed to ack");
            }
            Err(e) => {
                eprintln!("Error receiving message: {:?}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let channel = create_channel().await.expect("Failed to create channel");
    channel
        .queue_declare(
            "customer_queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Queue declare failed");

    // Bind queue to exchange
    channel
        .queue_bind(
            "customer_queue",
            "customer_exchange",
            "customer.*",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Queue bind failed");

    // Start consuming
    let mut consumer = channel
        .basic_consume(
            "customer_queue",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Consumer failed");

    println!("📡 Waiting for messages...");

    while let Some(delivery) = consumer.next().await {
        match delivery {
            Ok(delivery) => {
                let msg = String::from_utf8_lossy(&delivery.data);
                let routing_key = delivery.routing_key.clone();
                println!("Routing key is: {}", routing_key);
                println!("📨 Received: {}", msg);
                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("Ack failed");
            }
            Err(e) => eprintln!("❌ Error: {:?}", e),
        }
    }
}
