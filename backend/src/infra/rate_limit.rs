use std::{collections::HashMap, time::{Duration, Instant}};

use tokio::sync::Mutex;

#[derive(Debug)]
pub struct RateLimiter {
    max_per_minute: u32,
    window: Duration,
    records: Mutex<HashMap<String, Vec<Instant>>>,
}

impl RateLimiter {
    pub fn new(max_per_minute: u32) -> Self {
        Self {
            max_per_minute,
            window: Duration::from_secs(60),
            records: Mutex::new(HashMap::new()),
        }
    }

    pub async fn allow(&self, user_id: &str) -> bool {
        let mut records = self.records.lock().await;
        let now = Instant::now();

        let timestamps = records.entry(user_id.to_string()).or_default();
        timestamps.retain(|timestamp| now.duration_since(*timestamp) <= self.window);

        if timestamps.len() >= self.max_per_minute as usize {
            return false;
        }

        timestamps.push(now);
        true
    }
}
