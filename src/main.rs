#[derive(Copy, Clone)]
struct KeyValuePair {
    key: i32,
    value: i32,
}

struct MyHashMap {
    table: Vec<Option<Vec<KeyValuePair>>>,
}

impl KeyValuePair {
    fn new(key: i32, value: i32) -> Self {
        Self { key, value }
    }
}

impl MyHashMap {
    fn new() -> Self {
        let table_ = vec![None; 100];
        Self { table: table_ }
    }

    fn put(&mut self, key: i32, value: i32) {
        let index = self.hash(key);

        if let Some(bucket) = &mut self.table[index] {
            // Check for existing entry with the same key and update it
            for entry in bucket.iter_mut() {
                if entry.key == key {
                    entry.value = value;
                    return;
                }
            }

            // No existing entry with the same key, add a new one to the bucket
            bucket.push(KeyValuePair::new(key, value));
        } else {
            // No collision, create a new bucket and add the entry
            let mut bucket = Vec::new();
            bucket.push(KeyValuePair::new(key, value));
            self.table[index] = Some(bucket);
        }
    }

    fn get(&self, key: i32) -> i32 {
        let index = self.hash(key);

        if let Some(bucket) = &self.table[index] {
            for entry in bucket.iter() {
                if entry.key == key {
                    return entry.value;
                }
            }
        }

        -1
    }

    fn remove(&mut self, key: i32) {
        let index = self.hash(key);

        if let Some(bucket) = &mut self.table[index] {
            bucket.retain(|entry| entry.key != key);
        }
    }

    fn hash(&self, num: i32) -> usize {
        let mut hash_value = 0u64;
        let prime: u64 = 31; // Choose a prime number

        for (i, byte) in num.to_ne_bytes().iter().enumerate() {
            hash_value += (*byte as u64) * prime.pow(i as u32);
        }

        (hash_value % self.table.len() as u64) as usize
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_get() {
        let mut map = MyHashMap::new();
        map.put(1, 10);
        map.put(2, 20);
        map.put(3, 30);

        assert_eq!(map.get(1), 10);
        assert_eq!(map.get(2), 20);
        assert_eq!(map.get(3), 30);
        assert_eq!(map.get(4), -1); // Key not found
    }

    #[test]
    fn test_put_update() {
        let mut map = MyHashMap::new();
        map.put(1, 10);
        map.put(2, 20);
        map.put(3, 30);

        assert_eq!(map.get(1), 10);
        assert_eq!(map.get(2), 20);
        assert_eq!(map.get(3), 30);

        map.put(1, 100); // Update the value associated with key 1
        assert_eq!(map.get(1), 100);
    }

    #[test]
    fn test_remove() {
        let mut map = MyHashMap::new();
        map.put(1, 10);
        map.put(2, 20);
        map.put(3, 30);

        assert_eq!(map.get(1), 10);
        assert_eq!(map.get(2), 20);
        assert_eq!(map.get(3), 30);

        map.remove(2); // Remove key 2
        assert_eq!(map.get(2), -1); // Key 2 should not be found
    }
}
