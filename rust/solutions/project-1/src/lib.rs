use std::collections::HashMap;

#[derive(Debug)]
pub struct KvStore {
  store: HashMap<String, String>
}

impl KvStore {
  pub fn set(&mut self, key: String, value: String) {
    self.store.insert(key, value);
  }

  pub fn get(&self, key: String) -> Option<String>{
    self.store.get(&key).cloned()
  }

  pub fn remove(&mut self, key: String) {
    self.store.remove(&key);
  }

  pub fn new() -> KvStore {
    KvStore {
      store: HashMap::new()
    }
  }
}
