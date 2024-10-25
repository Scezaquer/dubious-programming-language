use std::collections::HashMap;

pub struct ImmutableMap<K, V> {
    map: HashMap<K, V>,
}

impl<K: std::cmp::Eq + std::hash::Hash, V> ImmutableMap<K, V> {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), &str> {
        if self.map.contains_key(&key) {
            Err("Cannot modify an existing key")
        } else {
            self.map.insert(key, value);
            Ok(())
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
}
