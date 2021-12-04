use std::sync::Arc;
use serde_json::Value;
use crate::rapids_and_rivers::river::{MessageLister, River};
use crate::store::RockyStore;

struct KafkaDiskUseListener {
    store: RockyStore
}

impl KafkaDiskUseListener {
    pub fn new(store: &RockyStore) -> KafkaDiskUseListener {
        Self {
            store: store.clone()
        }
    }

    pub fn new_box(store: &RockyStore) -> Box<KafkaDiskUseListener> {
        Box::new(Self::new(store))
    }
}

impl MessageLister for KafkaDiskUseListener {

    fn on_message(&mut self, message: &Value) {
        self.store.store("kafka-disk-status", &message.to_string());
    }
}

pub fn kafka_disk_use_river(store: &RockyStore) -> River {
    let mut kafka_disk_use_river = River::new();
    kafka_disk_use_river.validate(Box::new(
        | msg | msg["name"] == "kafka_disk_use"
    ));
    kafka_disk_use_river.register(KafkaDiskUseListener::new_box(store));
    return kafka_disk_use_river;
}
