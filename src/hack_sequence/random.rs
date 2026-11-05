use std::cell::Cell;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::Step;

thread_local! {
    static STATE: Cell<u64> = Cell::new(seed());
}

fn seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x9e37_79b9_7f4a_7c15)
        | 1
}

/// Small xorshift64 PRNG so the demo needs no external crates.
fn next_u64() -> u64 {
    STATE.with(|state| {
        let mut x = state.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        state.set(x);
        x
    })
}

fn random_below(n: u64) -> u64 {
    next_u64() % n
}

fn random_f64() -> f64 {
    (next_u64() >> 11) as f64 / (1u64 << 53) as f64
}

fn pick<T: Copy>(items: &[T]) -> T {
    items[random_below(items.len() as u64) as usize]
}

pub fn rand_delay(step: &Step) -> Duration {
    let delta = step.max_delay.saturating_sub(step.min_delay);
    if delta.is_zero() {
        step.min_delay
    } else {
        step.min_delay + Duration::from_nanos(next_u64() % delta.as_nanos() as u64)
    }
}

pub fn random_ip() -> String {
    format!(
        "{}.{}.{}.{}",
        random_below(256),
        random_below(256),
        random_below(256),
        random_below(256)
    )
}

pub fn random_port() -> u16 {
    1024 + random_below(64511) as u16
}

pub fn random_hex(length: usize) -> String {
    const CHARS: &[u8] = b"0123456789abcdef";
    (0..length)
        .map(|_| CHARS[random_below(CHARS.len() as u64) as usize] as char)
        .collect()
}

pub fn random_token() -> String {
    random_hex(32)
}

pub fn random_mac() -> String {
    format!(
        "{}:{}:{}:{}:{}:{}",
        random_hex(2),
        random_hex(2),
        random_hex(2),
        random_hex(2),
        random_hex(2),
        random_hex(2)
    )
}

pub fn mask_account(account: &str) -> String {
    let (local, domain) = match account.split_once('@') {
        Some((local, domain)) => (local, Some(domain)),
        None => (account, None),
    };
    let visible: String = local.chars().take(2).collect();
    match domain {
        Some(domain) => format!("{visible}***@{domain}"),
        None => format!("{visible}***"),
    }
}

const FIRST_NAMES: &[&str] = &[
    "Alex", "Jordan", "Morgan", "Casey", "Riley", "Sam", "Taylor", "Avery", "Quinn", "Jamie",
];

const LAST_NAMES: &[&str] = &[
    "Miller", "Hayes", "Brooks", "Reed", "Shaw", "Cole", "Diaz", "Novak", "Frost", "Vale",
];

const PLATFORMS: &[&str] = &[
    "GitHub",
    "Reddit",
    "X",
    "Discord",
    "Steam",
    "Mastodon",
    "Hacker News",
    "Twitch",
];

const CITIES: &[&str] = &[
    "Reykjavik, IS",
    "Zurich, CH",
    "Oslo, NO",
    "Lisbon, PT",
    "Montreal, CA",
    "Austin, US",
    "Tallinn, EE",
    "Wellington, NZ",
];

const DEVICES: &[&str] = &[
    "iPhone 15",
    "Pixel 9",
    "Windows 11 desktop",
    "MacBook Air",
    "Linux workstation",
    "Android tablet",
];

const TIMEZONES: &[&str] = &[
    "UTC-08:00",
    "UTC-05:00",
    "UTC+00:00",
    "UTC+01:00",
    "UTC+02:00",
    "UTC+09:00",
];

pub fn random_fake_name() -> String {
    format!("{} {}", pick(FIRST_NAMES), pick(LAST_NAMES))
}

pub fn random_platforms(count: usize) -> Vec<&'static str> {
    let mut items = PLATFORMS.to_vec();
    for i in (1..items.len()).rev() {
        let j = random_below((i + 1) as u64) as usize;
        items.swap(i, j);
    }
    items.truncate(count.min(items.len()));
    items
}

pub fn random_city() -> &'static str {
    pick(CITIES)
}

pub fn random_device() -> &'static str {
    pick(DEVICES)
}

pub fn random_timezone() -> &'static str {
    pick(TIMEZONES)
}

pub fn random_confidence() -> f64 {
    70.0 + random_f64() * 29.5
}

pub fn random_count(max: u64) -> u64 {
    random_below(max.max(1))
}

pub fn random_date() -> String {
    let year = 2012 + random_below(12);
    let month = 1 + random_below(12);
    let day = 1 + random_below(28);
    format!("{year:04}-{month:02}-{day:02}")
}

#[allow(dead_code)]
pub fn random_dump_size() -> String {
    format!("{:.1} GB", 0.5 + random_f64() * 240.0)
}
