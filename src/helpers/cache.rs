use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::Instant;

// Define a generic type that represents the cache
struct Cache<T> {
    data: HashMap<u64, (Instant, T)>,
    capacity: usize,
}

impl<T> Cache<T> {
    // Create a new Cache with a given capacity
    fn new(capacity: usize) -> Self {
        Self {
            data: HashMap::new(),
            capacity,
        }
    }

    // Get the value from the cache if it exists and is still valid (not expired)
    fn get(&self, key: &u64) -> Option<&T> {
        if let Some((instant, value)) = self.data.get(key) {
            // Check if the value is still valid (not expired)
            if instant.elapsed().as_secs() < 60 {
                return Some(value);
            }
        }
        None
    }

    // Insert a value into the cache with the given key
    fn insert(&mut self, key: u64, value: T) {
        // Check if the cache is full, and if so, remove the oldest entry
        if self.data.len() >= self.capacity {
            let oldest_key = *self.data.keys().min().unwrap();
            self.data.remove(&oldest_key);
        }

        // Insert the new value into the cache with the current timestamp
        let instant = Instant::now();
        self.data.insert(key, (instant, value));
    }
}

// Cache wrapper function
pub fn cache<T, F>(func: F) -> impl FnMut() -> T
where
    T: Clone,
    F: Fn() -> T + Hash,
{
    // Create a new cache with a capacity of 10 entries
    let mut cache: Cache<T> = Cache::new(10);

    // Return a closure that acts as the cached version of the original function
    move || {
        // Generate a unique key for the function using its name and arguments
        let mut hasher = DefaultHasher::new();
        func.hash(&mut hasher);
        let key = hasher.finish();

        // Check if the value is already in the cache and return it if found
        if let Some(cached_value) = cache.get(&key) {
            return cached_value.clone();
        }

        // If the value is not in the cache, call the original function and store the result in the cache
        let result = func();
        cache.insert(key, result.clone());
        result
    }
}
