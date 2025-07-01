use crate::models::messages::CustomerCreated;
use lapin::{
    BasicProperties, Channel, Connection, ConnectionProperties, options::*, types::FieldTable,
};
use serde::Serialize;
use tokio_amqp::LapinTokioExt;
use tokio_amqp::*;
