use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, Instant},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RateLimitDecision {
    pub allowed: bool,
    pub remaining: u32,
}

#[derive(Debug)]
struct RateLimitBucket {
    window_started_at: Instant,
    attempts: u32,
}

#[derive(Debug)]
pub struct InMemoryRateLimiter {
    max_attempts: u32,
    window: Duration,
    buckets: Mutex<HashMap<String, RateLimitBucket>>,
}

impl InMemoryRateLimiter {
    pub fn new(max_attempts: u32, window: Duration) -> Self {
        Self {
            max_attempts,
            window,
            buckets: Mutex::new(HashMap::new()),
        }
    }

    pub fn check(&self, key: impl Into<String>) -> RateLimitDecision {
        let now = Instant::now();
        let key = key.into();
        let mut buckets = self
            .buckets
            .lock()
            .expect("rate limiter mutex should not be poisoned");
        let bucket = buckets.entry(key).or_insert_with(|| RateLimitBucket {
            window_started_at: now,
            attempts: 0,
        });

        if now.duration_since(bucket.window_started_at) >= self.window {
            bucket.window_started_at = now;
            bucket.attempts = 0;
        }

        if bucket.attempts >= self.max_attempts {
            return RateLimitDecision {
                allowed: false,
                remaining: 0,
            };
        }

        bucket.attempts += 1;
        RateLimitDecision {
            allowed: true,
            remaining: self.max_attempts.saturating_sub(bucket.attempts),
        }
    }

    pub fn clear(&self) {
        self.buckets
            .lock()
            .expect("rate limiter mutex should not be poisoned")
            .clear();
    }
}

impl Default for InMemoryRateLimiter {
    fn default() -> Self {
        Self::new(5, Duration::from_secs(60))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn denies_after_configured_attempts_in_window() {
        let limiter = InMemoryRateLimiter::new(2, Duration::from_secs(60));

        assert!(limiter.check("login:client:user@example.com").allowed);
        assert!(limiter.check("login:client:user@example.com").allowed);
        assert!(!limiter.check("login:client:user@example.com").allowed);
    }

    #[test]
    fn tracks_keys_independently() {
        let limiter = InMemoryRateLimiter::new(1, Duration::from_secs(60));

        assert!(limiter.check("login:client:a@example.com").allowed);
        assert!(!limiter.check("login:client:a@example.com").allowed);
        assert!(limiter.check("login:client:b@example.com").allowed);
    }
}
