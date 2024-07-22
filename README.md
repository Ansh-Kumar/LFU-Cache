# LFU Cache

I am learning Rust and I thought creating a really basic LFU cache would help.

## How It Works

`LFUCache::new(capacity)` - creates a new LFU Cache of size capacity

`LFUCache::push(key, value)` - pushes a new key value to the cache

`LFECache::get(key)` - gets the value if in the cache, None otherwise
