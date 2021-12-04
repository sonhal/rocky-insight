use std::sync::Arc;
use rdkafka::consumer::{DefaultConsumerContext, StreamConsumer};
use rdkafka::Message;
use serde_json::Value;
use crate::rapids_and_rivers::river::River;
use crate::rapids_and_rivers::kafka::{create_consumer, RapidConsumer};

pub struct Rapid {
    consumer: RapidConsumer,
    running: bool,
    rivers: Vec<River>,
}

impl Rapid {
    pub fn new(bootstrap_servers: &str, topic: &str) -> Rapid {
        Rapid {
            consumer: create_consumer(bootstrap_servers, topic),
            running: false,
            rivers: vec![],
        }
    }

    pub fn register(&mut self, river: River) {
        self.rivers.push(river)
    }

    pub async fn start(&mut self) -> Result<(), &'static str> {
        if self.running { return Err("Already running"); }
        self.running = true;

        loop {
            match self.consumer.recv().await {
                Err(e) => {
                    warn!("Kafka error: {}", e);
                    continue;
                }
                Ok(m) => {
                    let message = match m.payload_view::<str>() {
                        None => continue,
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            warn!("Error while deserializing message payload: {:?}", e);
                            continue;
                        }
                    };
                    debug!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), message, m.topic(), m.partition(), m.offset(), m.timestamp());

                    let parsed_message: Value = serde_json::from_str(message).expect("Error parsing JSON data");
                    for (i, river) in &mut self.rivers.iter_mut().enumerate() {
                        debug!("passing msg = {}, to river index = {}", parsed_message, i);
                        river.handle(&parsed_message);
                    }
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use log::LevelFilter;
    use rdkafka::ClientConfig;
    use rdkafka::producer::{FutureProducer, FutureRecord};
    use serde_json::Value;
    use crate::rapids_and_rivers::rapid::Rapid;
    use crate::rapids_and_rivers::river::{MessageLister, River};
    use testcontainers::{clients, images::kafka};
    use testcontainers::core::env::command;

    #[test]
    fn build() {
        let mut rapid = Rapid::new("test", "test");
        let river = River::new();
        rapid.register(river);
    }


    struct TestApp {
        received: Arc<Mutex<Vec<Value>>>
    }

    impl MessageLister for TestApp {
        fn on_message(&mut self, message: &Value) {
            self.received.lock().unwrap().push(message.clone());
        }
    }

    #[tokio::test]
    async fn receives_message() {
        env_logger::builder().filter_level(LevelFilter::Info).try_init();
        let docker = clients::Cli::docker();
        let kafka_node = docker.run(kafka::Kafka::default());
        let topic = "test-topic";

        let bootstrap_servers = format!("localhost:{}", kafka_node.get_host_port(kafka::KAFKA_PORT));

        let producer = ClientConfig::new()
            .set("bootstrap.servers", &bootstrap_servers)
            .set("message.timeout.ms", "5000")
            .create::<FutureProducer>()
            .expect("Failed to create Kafka FutureProducer");

        let expected = vec![
            "{\"@type\": \"cool-type\"}".to_string(),
            "{\"field\": \"value\"}".to_string(),
        ];

        for (i, message) in expected.iter().enumerate() {
            debug!("Sending message {}", message);
            producer
                .send(
                    FutureRecord::to(topic)
                        .payload(message)
                        .key(&format!("Key {}", i)),
                    Duration::from_secs(0),
                )
                .await
                .unwrap();
        }

        let mut rapid = Rapid::new(&bootstrap_servers, topic);
        let mut test_app = TestApp { received: Arc::new(Mutex::new(vec![] )) };
        let result = Arc::clone(&test_app.received);


        debug!("Starting rapid task");
        let rapid_handle = tokio::spawn(async move {
            let mut river = River::new();
            river.validate( Box::new(| msg | msg["@type"] == "cool-type" ));
            river.register(Box::new(test_app));
            rapid.register(river);
            rapid.start().await
        });

        debug!("Sleeping task");
        tokio::time::sleep(Duration::from_secs(6)).await;
        debug!(" docker = {:#?} ", docker);
        kafka_node.stop();

        assert_eq!(result.lock().unwrap().len(), 1);

        rapid_handle.abort();
    }
}