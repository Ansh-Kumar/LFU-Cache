use lfu_cache::lfu_cache::LFUCache;
fn main() {
    println!("Running LFU Cache Simulation");

    let mut cache = LFUCache::new(3);

    cache.push("key1".to_string(), 1);
    cache.push("key2".to_string(), 2);
    cache.push("key3".to_string(), 3);

    // should evict key1
    cache.push("key4".to_string(), 4);

    assert_eq!(cache.get(&String::from("key1")), None);

    cache.get(&String::from("key2"));

    // should evict key3
    cache.push("key5".to_string(), 5);

    assert_eq!(cache.get(&String::from("key2")), Some(2).as_ref());
    assert_eq!(cache.get(&String::from("key3")), None);
}
