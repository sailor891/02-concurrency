use dashmap::DashMap;
use std::{fmt, sync::Arc};

#[derive(Clone)]
pub struct Metrics {
    pub data: Arc<DashMap<String, i64>>,
}
impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
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
        let mut cnt = self.data.entry(key.to_string()).or_insert(0);
        *cnt += 1;
    }
}
impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for entry in self.data.iter() {
            s += &format!("{}:{}\n", entry.key(), entry.value());
        }
        write!(f, "{}", s)?;
        Ok(())
    }
}
