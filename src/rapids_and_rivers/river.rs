use serde_json::Value;


pub trait MessageLister {
    fn on_message(&mut self, message: &Value);
}

pub type PacketValidation = Box<dyn Fn(&serde_json::Value) -> bool  + Send + Sync>;


pub struct  River {
    validations: Vec<PacketValidation>,
    listeners: Vec<Box<dyn MessageLister + Send + Sync>>
}

impl River {

    pub fn new() -> River {
        River { validations: vec![], listeners: vec![] }
    }

    pub fn validate(&mut self, validation: PacketValidation) {
        self.validations.push(validation)
    }

    pub fn register(&mut self, listener: Box<dyn MessageLister + Send + Sync>) {
        self.listeners.push(listener)
    }

    pub(crate) fn handle(&mut self, message: &Value) -> bool {
        if self.validations.iter().all(|v| v(message)) {
            self.listeners.iter_mut().for_each(| listener| listener.on_message(message));
            return true;
        }
        return false;
    }
}



#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
    use serde_json::Value;
    use crate::rapids_and_rivers::river::{MessageLister, River};

    struct TestListener {
        received: Arc<Mutex<Vec<Value>>>
    }

    impl TestListener {
        fn new() -> TestListener {
            TestListener { received: Arc::new(Mutex::new(vec![]))}
        }
    }

    impl MessageLister for TestListener {
        fn on_message(&mut self, message: &Value) {
            self.received.lock().unwrap().push(message.clone())
        }
    }

    #[test]
    fn filter_message() {
        let mut tester = Box::new(TestListener::new());
        let received = Arc::clone(&tester.received);

        let mut river = River::new();

        river.validate(Box::new(| msg | msg["@type"].is_string()));
        river.register(tester);

        let accepted_message: Value = "{\"@type\": \"some-event\"}".parse().unwrap();
        let filtered_message: Value = "{\"field\": \"data\"}".parse().unwrap();
        river.handle(&accepted_message);
        river.handle(&filtered_message);

        let result = received.lock().unwrap();
        assert_eq!(result.len(), 1)
    }
}
