use anyhow::Result;
use core::fmt;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

#[derive(Debug, Clone)]
pub struct AtomicMetrics {
    pub data: Arc<HashMap<&'static str, AtomicI64>>,
}
impl AtomicMetrics {
    pub fn new(metrics_name: &[&'static str]) -> Self {
        let data = metrics_name
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        Self {
            data: Arc::new(data),
        }
    }
    pub fn add(&self, key: &str) -> Result<()> {
        let count = self
            .data
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("metrics not found"))?;
        count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}
impl fmt::Display for AtomicMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (key, count) in self.data.iter() {
            s.push_str(&format!("{}: {}\n", key, count.load(Ordering::Relaxed)));
        }
        write!(f, "{}", s)
    }
}
