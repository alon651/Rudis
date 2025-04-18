use std::collections::HashMap;

pub struct Memory {
    pub data: HashMap<String, String>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }
}
