use crate::messaging::connection::{create_channel, declare_exchange};
use crate::models::customer::Customer;
use crate::models::messages::CustomerCreated;
use lapin::options::ExchangeDeclareOptions;
use lapin::{
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind, options::*,
    types::FieldTable,
};

use actix::prelude::*;
use log::info;
use serde::Serialize;
use tokio_amqp::*;

pub struct Publisher {
    pub channel: Channel,
    pub exchange: String,
}

impl Actor for Publisher {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
pub struct Publish<T: Serialize + Send + 'static> {
    pub routing_key: String,
    pub payload: T,
}

impl<T> Handler<Publish<T>> for Publisher
where
    T: Serialize + Send + 'static,
{
    type Result = ResponseActFuture<Self, Result<(), ()>>;

    fn handle(&mut self, msg: Publish<T>, _ctx: &mut Context<Self>) -> Self::Result {
        let channel = self.channel.clone();
        let exchange = self.exchange.clone();

        Box::pin(
            async move {
                let data = serde_json::to_vec(&msg.payload).map_err(|_| ())?;
                channel
                    .basic_publish(
                        &exchange,
                        &msg.routing_key,
                        BasicPublishOptions::default(),
                        &data,
                        BasicProperties::default(),
                    )
                    .await
                    .map_err(|_| ())?
                    .await
                    .map_err(|_| ())?;
                Ok(())
            }
            .into_actor(self),
        )
    }
}
