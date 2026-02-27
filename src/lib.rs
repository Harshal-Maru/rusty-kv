use std::collections::HashMap;

pub struct KvStore {
    entries: HashMap<String, String>,
}

impl KvStore {
    // Create a new, empty Key-Value Store
    pub fn new() -> Self {
        KvStore {
            entries: HashMap::new(),
        }
    }

    // Set a key value pair
    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.entries.insert(key, value)
    }

    // Get a key value pair, given the key
    pub fn get(&self, key: String) -> Option<String> {
        self.entries.get(&key).cloned()
    }

    // Remove a key value pair, given the key and return the value
    pub fn remove(&mut self, key: String) -> Option<String> {
        self.entries.remove(&key)
    }
}


#[cfg(test)]
mod tests {
    // Import everything from the outer module (your KvStore)
    use super::*;

    // The #[test] annotation marks a function as a unit test.
    #[test]
    fn test_set_and_get() {
        let mut store = KvStore::new();
        
        // We use .to_string() to convert string literals (&str) into owned Strings.
        store.set("key1".to_string(), "value1".to_string());
        store.set("key2".to_string(), "value1".to_string());
        store.set("key2".to_string(), "value2".to_string());
        store.set("Name".to_string(), "Harshal".to_string());

        // assert_eq! checks if both sides are equal. If not, the test fails.
        assert_eq!(store.get("key1".to_string()), Some("value1".to_string()));
        assert_eq!(store.get("key2".to_string()), Some("value2".to_string()));
        assert_eq!(store.get("Name".to_string()), Some("Harshal".to_string()));
        assert_eq!(store.remove("key1".to_string()), Some("value1".to_string()));
        assert_eq!(store.remove("key1".to_string()), None);
    }
}