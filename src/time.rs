use std::time::{SystemTime, UNIX_EPOCH};

pub type NanoTime = u128;
pub type SecTime = u64;

pub fn now_nano() -> NanoTime {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

pub const fn sec_to_nano(sec: SecTime) -> NanoTime {
    (sec as NanoTime) * 1_000_000_000
}

pub const fn nano_to_sec(nano: NanoTime) -> SecTime {
    (nano / 1_000_000_000) as SecTime
}
