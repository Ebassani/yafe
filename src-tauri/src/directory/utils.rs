use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn system_time_to_u64(time: SystemTime) -> Option<u64> {
    time.duration_since(UNIX_EPOCH).ok().map(|d| d.as_secs())
}