use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex, time::Instant};

#[derive(Clone, Copy)]
pub struct Config {
    pub capacity: u32,
    pub refill_rate: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            capacity: 5,
            refill_rate: 1.0,
        }
    }
}

#[derive(Debug)]
pub enum RateLimitError {
    Exceeded,
}
pub struct RateLimiter {
    buckets: HashMap<String, TokenBucket>,
}

impl RateLimiter {
    fn new() -> Self {
        Self {
            buckets: HashMap::new(),
        }
    }

    pub fn check(&mut self, key: &str, capacity: u32, refill_rate: f64) -> bool {
        let bucket = self.buckets.entry(key.to_string()).or_insert_with(|| {
            TokenBucket::new(Config {
                capacity,
                refill_rate,
            })
        });

        // Update config if it changed
        bucket.config.capacity = capacity;
        bucket.config.refill_rate = refill_rate;

        bucket.check()
    }
    pub fn try_consume(
        &mut self,
        key: &str,
        capacity: u32,
        refill_rate: f64,
    ) -> Result<(), RateLimitError> {
        let bucket = self.buckets.entry(key.to_string()).or_insert_with(|| {
            TokenBucket::new(Config {
                capacity,
                refill_rate,
            })
        });

        // Update config if it changed
        bucket.config.capacity = capacity;
        bucket.config.refill_rate = refill_rate;

        if bucket.try_consume() {
            Ok(())
        } else {
            Err(RateLimitError::Exceeded)
        }
    }
}

pub struct TokenBucket {
    config: Config,
    tokens: f64,
    last_refill: Instant,
}

impl TokenBucket {
    fn new(config: Config) -> Self {
        Self {
            config: config,
            tokens: config.capacity as f64,
            last_refill: Instant::now(),
        }
    }
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens += elapsed * self.config.refill_rate;
        self.tokens = self.tokens.min(self.config.capacity as f64);
        self.last_refill = now;
    }

    fn check(&mut self) -> bool {
        self.refill();
        if self.tokens >= 1.0 { true } else { false }
    }
    fn try_consume(&mut self) -> bool {
        self.refill();
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

pub static RATE_LIMITER: Lazy<Mutex<RateLimiter>> = Lazy::new(|| Mutex::new(RateLimiter::new()));
