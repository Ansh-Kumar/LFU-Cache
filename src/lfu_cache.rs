use std::collections::HashMap;

pub struct LFUCache {
    capacity: i64,
    kvs: HashMap<String, i64>,
    frequency_count: HashMap<String, i64>,
    frequency_list: HashMap<i64, Vec<String>>,
}

impl LFUCache {
    pub fn new(capacity: i64) -> Self {
        LFUCache {
            capacity: capacity,
            kvs: HashMap::new(),
            frequency_count: HashMap::new(),
            frequency_list: HashMap::new(),
        }
    }

    fn update_frequency(&mut self, key: &String) {
        // get old frequency
        let old_freq = self.frequency_count.get(key).copied().unwrap_or(0);
        let new_freq = old_freq + 1;

        // update frequency count
        self.frequency_count.insert(key.clone(), new_freq);

        // update frequency list
        if let Some(freq_vec) = self.frequency_list.get_mut(&old_freq) {
            if let Some(pos) = freq_vec.iter().position(|x| x == key) {
                freq_vec.remove(pos);
            }
            if freq_vec.is_empty() {
                self.frequency_list.remove(&old_freq);
            }
        }

        self.frequency_list
            .entry(new_freq)
            .or_insert_with(Vec::new)
            .push(key.clone());
    }

    pub fn get(&mut self, key: &String) -> Option<&i64> {
        if self.kvs.contains_key(key) {
            self.update_frequency(key);
            self.kvs.get(key)
        } else {
            None
        }
    }

    pub fn push(&mut self, key: String, value: i64) {
        if self.capacity == 0 {
            return;
        }
        if self.kvs.len() as i64 == self.capacity {
            self.evict();
        }
        self.kvs.insert(key.clone(), value);
        self.frequency_count.insert(key.clone(), 1);
        self.frequency_list
            .entry(1)
            .or_insert_with(Vec::new)
            .push(key);
    }

    fn evict(&mut self) {
        if let Some((&min_freq, keys)) =
            self.frequency_list.iter_mut().min_by_key(|(&freq, _)| freq)
        {
            if let Some(evict_key) = keys.first() {
                self.kvs.remove(evict_key);
                self.frequency_count.remove(evict_key);
                keys.remove(0);
                if keys.is_empty() {
                    self.frequency_list.remove(&min_freq);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_get() {
        let mut cache = LFUCache::new(2);

        cache.push("key1".to_string(), 1);
        assert_eq!(cache.get(&String::from("key1")), Some(1).as_ref());

        cache.push("key2".to_string(), 2);
        assert_eq!(cache.get(&String::from("key2")), Some(2).as_ref());
    }

    #[test]
    fn test_eviction() {
        let mut cache = LFUCache::new(2);

        cache.push("key1".to_string(), 1);
        cache.push("key2".to_string(), 2);

        // This should cause "key1" to be evicted because "key1" and "key2" have the same frequency, but "key1" was inserted first.
        cache.push("key3".to_string(), 3);

        assert_eq!(cache.get(&String::from("key1")), None);
        assert_eq!(cache.get(&String::from("key2")), Some(2).as_ref());
        assert_eq!(cache.get(&String::from("key3")), Some(3).as_ref());
    }

    #[test]
    fn test_frequency_update() {
        let mut cache = LFUCache::new(2);

        cache.push("key1".to_string(), 1);
        cache.push("key2".to_string(), 2);

        // Access "key1" to increase its frequency
        assert_eq!(cache.get(&String::from("key1")), Some(1).as_ref());

        // This should cause "key2" to be evicted because "key1" has a higher frequency.
        cache.push("key3".to_string(), 3);

        assert_eq!(cache.get(&String::from("key1")), Some(1).as_ref());
        assert_eq!(cache.get(&String::from("key2")), None);
        assert_eq!(cache.get(&String::from("key3")), Some(3).as_ref());
    }

    #[test]
    fn test_update_existing_key() {
        let mut cache = LFUCache::new(2);

        cache.push("key1".to_string(), 1);
        cache.push("key1".to_string(), 10);

        assert_eq!(cache.get(&String::from("key1")), Some(10).as_ref());
    }

    #[test]
    fn test_zero_capacity() {
        let mut cache = LFUCache::new(0);

        cache.push("key1".to_string(), 1);
        assert_eq!(cache.get(&String::from("key1")), None);
    }
}
