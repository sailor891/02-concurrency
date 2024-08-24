use anyhow::Result;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Metrics {
    pub data: Arc<Mutex<HashMap<String, i64>>>,
}
impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
impl Metrics {
    pub fn add(&self, key: &str) {
        let mut bind = self.data.lock().unwrap();
        let cnt = bind.entry(key.to_string()).or_insert(0);
        *cnt += 1;
    }
    pub fn get(&self, key: &str) -> Result<i64> {
        let bind = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        match bind.get(key) {
            Some(&value) => Ok(value),
            None => Err(anyhow::anyhow!("key not found")),
        }
    }
    pub fn get_all(&self) -> Result<HashMap<String, i64>> {
        let bind = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        Ok(bind.clone())
    }
}
