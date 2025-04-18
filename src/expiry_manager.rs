use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type Timestamp = u64;

pub struct ExpiryManager {
    // Maps expiration timestamps to sets of keys
    expires: BTreeMap<Timestamp, HashSet<String>>,
    // Maps keys to their expiration timestamps
    key_to_expiry: HashMap<String, Timestamp>,
}

impl ExpiryManager {
    pub fn new() -> Self {
        ExpiryManager {
            expires: BTreeMap::new(),
            key_to_expiry: HashMap::new(),
        }
    }

    fn get_now_ms() -> Timestamp {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_millis() as Timestamp
    }

    pub fn set_expiry(&mut self, key: &str, ttl_ms: u64) {
        let expiry_time = Self::get_now_ms() + ttl_ms;

        // Remove old expiry if it exists
        if let Some(&old_expiry) = self.key_to_expiry.get(key) {
            if let Some(keys) = self.expires.get_mut(&old_expiry) {
                keys.remove(key);
                if keys.is_empty() {
                    self.expires.remove(&old_expiry);
                }
            }
        }

        // Add new expiry
        self.key_to_expiry.insert(key.to_string(), expiry_time);
        self.expires
            .entry(expiry_time)
            .or_default()
            .insert(key.to_string());
    }

    pub fn remove_expiry(&mut self, key: &str) {
        if let Some(&expiry_time) = self.key_to_expiry.get(key) {
            if let Some(keys) = self.expires.get_mut(&expiry_time) {
                keys.remove(key);
                if keys.is_empty() {
                    self.expires.remove(&expiry_time);
                }
            }
            self.key_to_expiry.remove(key);
        }
    }

    pub fn is_expired(&self, key: &str) -> bool {
        if let Some(&expiry_time) = self.key_to_expiry.get(key) {
            Self::get_now_ms() >= expiry_time
        } else {
            false
        }
    }

    pub fn cleanup_expired_keys<F>(&mut self, mut remove_key: F)
    where
        F: FnMut(&str),
    {
        let now = Self::get_now_ms();
        let expired_keys: Vec<Timestamp> = self
            .expires
            .range(..=now)
            .map(|(&timestamp, _)| timestamp)
            .collect();

        for timestamp in expired_keys {
            if let Some(keys) = self.expires.remove(&timestamp) {
                for key in keys {
                    self.key_to_expiry.remove(&key);
                    remove_key(&key); // Call the callback to remove the key from Memory
                }
            }
        }
    }
}
