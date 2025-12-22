#![allow(dead_code)]

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Uuid(String);

impl Uuid {
    pub fn new_v4() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or_default();
        let value = (nanos ^ counter).to_be_bytes();
        let hex = value.iter().map(|b| format!("{b:02x}")).collect::<String>();
        Self(format!("{hex}-{:016x}", counter))
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
