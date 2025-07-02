use crate::models::messages::CustomerCreated;
use lapin::options::ExchangeDeclareOptions;
use lapin::{
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind, options::*,
    types::FieldTable,
};

use log::info;
use serde::Serialize;
use tokio_amqp::*;

async fn create_msg_connection() -> Result<Connection, lapin::Error> {
    let connection_url = std::env::var("RABBITMQ_URL")
        .unwrap_or_else(|_| "amqp://rabbitmq:rabbitmq@localhost:5672/%2f".to_string());

    let connection_properties = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    let connection = Connection::connect(&connection_url, connection_properties).await?;

    Ok(connection)
}

async fn create_channel() -> Result<Channel, lapin::Error> {
    match create_msg_connection().await {
        Ok(connection) => {
            info!("Creating channel");
            let channel = connection.create_channel().await?;
            Ok(channel)
        }
        Err(err) => Err(err),
    }
}

async fn declare_exchange(channel: &Channel, name: &str, kind: ExchangeKind) {
    channel
        .exchange_declare(
            name,
            kind,
            ExchangeDeclareOptions {
                passive: false,
                durable: true,
                auto_delete: false,
                internal: false,
                nowait: false,
            },
            FieldTable::default(),
        )
        .await
        .expect("Failed to declare exchange");
}

pub async fn publish_customer_created(customer: CustomerCreated) {
    let payload = serde_json::to_vec(&customer).unwrap();
    info!("Publishing customer created message");
    if let Ok(channel) = create_channel().await {
        declare_exchange(&channel, "customer_exchange", ExchangeKind::Direct).await;
        channel
            .basic_publish(
                "customer_exchange", // exchange name
                "customer.created",  // routing key
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await
            .unwrap()
            .await
            .unwrap();
    }
}
