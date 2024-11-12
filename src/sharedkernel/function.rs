use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_now() -> u64 {
    let start_time: Result<std::time::Duration, std::time::SystemTimeError> =
        SystemTime::now().duration_since(UNIX_EPOCH);

    match start_time {
        Ok(duration) => duration.as_secs(),
        Err(_) => 0u64,
    }
}
