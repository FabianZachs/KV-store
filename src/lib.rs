use std::collections::HashMap;

/// The `KvStore` stores String key/value pairs.
/// This is not disk persistant.
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates a new `KvStore`.
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Sets a String key to a String value.
    pub fn set(&mut self, k: String, v: String) {
        self.map.insert(k, v);
    }

    /// Returns the String value corresponding to a key.
    ///
    /// Returns `None` if no such key has been  stored.
    pub fn get(&self, k: String) -> Option<String> {
        Some(String::from(self.map.get(&k)?))
    }

    /// Remmoves a key-value entry via the key
    pub fn remove(&mut self, k: String) {
        self.map.remove(&k);
    }
}
