use dashmap::DashMap;
use std::{fmt, sync::Arc};

// concurrent map metrics
#[derive(Clone)]
pub struct CmapMetrics {
    pub data: Arc<DashMap<String, i64>>,
}
impl CmapMetrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }
}
impl Default for CmapMetrics {
    fn default() -> Self {
        Self::new()
    }
}
impl CmapMetrics {
    pub fn add(&self, key: &str) {
        let mut cnt = self.data.entry(key.to_string()).or_insert(0);
        *cnt += 1;
    }
}
impl fmt::Display for CmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for entry in self.data.iter() {
            s += &format!("{}:{}\n", entry.key(), entry.value());
        }
        write!(f, "{}", s)?;
        Ok(())
    }
}
