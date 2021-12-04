use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, String>>>;


#[derive(Clone)]
pub struct RockyStore {
    db: Db
}

impl RockyStore {

    pub fn new() -> RockyStore {
        RockyStore {
            db: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn store(&mut self, key: &str, json: &str) -> Option<String> {
        let mut lock = self.db.lock().unwrap();
        lock.insert(key.to_string(), json.to_string())
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        let mut lock = self.db.lock().unwrap();
        lock.get(key).cloned()
    }
}

#[cfg(test)]
mod test {
    use crate::store::RockyStore;

    #[test]
    fn store_json() {
        let key = "containers";
        let value ="{ \"key\": \"value\"}";
        let mut store = RockyStore::new();
        store.store(key, value);
        let result = store.get(key);
        assert_eq!(value, result.unwrap())
    }

}
