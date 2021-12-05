use rdkafka::consumer::{Consumer, ConsumerContext, DefaultConsumerContext, StreamConsumer};
use rdkafka::ClientConfig;

pub(crate) type RapidConsumer = StreamConsumer<DefaultConsumerContext>;

const GROUP_ID: &str = "cg-rocky-insight-1";

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

    consumer
        .subscribe(&[topic])
        .expect("Error while subscribing");
    return consumer;
}

pub fn create_ssl_consumer(
    bootstrap_servers: &str,
    topic: &str,
    ca_location: &str,
    cert_location: &str,
    key_location: &str,
    key_password: &str,
) -> RapidConsumer {
    let consumer: RapidConsumer = ClientConfig::new()
        .set("group.id", GROUP_ID)
        .set("bootstrap.servers", bootstrap_servers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .set("security.protocol", "SSL")
        .set("ssl.ca.location", ca_location)
        .set("ssl.certificate.location", cert_location)
        .set("ssl.key.location", key_location)
        .set("ssl.key.password", key_password)
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[topic])
        .expect("Error while subscribing");
    return consumer;
}
