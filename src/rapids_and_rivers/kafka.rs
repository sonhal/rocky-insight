use std::process::exit;
use std::sync::Arc;
use rdkafka::{ClientConfig, ClientContext, Message};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{BaseConsumer, Consumer, ConsumerContext, DefaultConsumerContext, StreamConsumer};
use rdkafka::error::{KafkaError, KafkaResult};
use rdkafka::util::Timeout;
use rdkafka::util::Timeout::After;

pub(crate) type RapidConsumer = StreamConsumer<DefaultConsumerContext>;

const GROUP_ID: &str = "cg-rocky-insight-2";

pub fn create_consumer(bootstrap_servers: &str, topic: &str) -> RapidConsumer {
    let consumer: RapidConsumer = ClientConfig::new()
        .set("group.id", GROUP_ID)
        .set("bootstrap.servers", bootstrap_servers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&[topic]).expect("Error while subscribing");
    return consumer;
}

