use std::sync::Arc;
use serde_json::Value;
use crate::rapids_and_rivers::river::{MessageLister, River};
use crate::store::RockyStore;

struct ContainerListener {
    store: RockyStore
}

impl ContainerListener {
    pub fn new(store: &RockyStore) -> ContainerListener {
        Self {
            store: store.clone()
        }
    }

    pub fn new_box(store: &RockyStore) -> Box<ContainerListener> {
        Box::new(Self::new(store))
    }
}

impl MessageLister for ContainerListener {

    fn on_message(&mut self, message: &Value) {
        self.store.store("containers-status", &message.to_string());
    }
}

pub fn container_river(store: &RockyStore) -> River {
    let mut container_river = River::new();
    container_river.validate(Box::new(
        | msg | msg["name"] == "docker_containers"
    ));
    container_river.register(ContainerListener::new_box(store));
    return container_river;
}
