mod random;
mod sequence;

pub use sequence::{fake_account_demo, fake_deanon_sequence, fake_hack_sequence};

use std::time::Duration;

pub struct Step {
    pub message: String,
    pub min_delay: Duration,
    pub max_delay: Duration,
}

impl Step {
    fn new(message: impl Into<String>, min_ms: u64, max_ms: u64) -> Self {
        Self {
            message: message.into(),
            min_delay: Duration::from_millis(min_ms),
            max_delay: Duration::from_millis(max_ms),
        }
    }
}
