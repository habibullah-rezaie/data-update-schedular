use std::io::Error;

use chrono::{NaiveTime, Utc};

/// Get the difference of a time in during the day from now
///
/// * `time_seconds` - The time of day in seconds
/// NOTE: time_seconds cannot be more than 87400
pub fn get_diff_from_now_in_secs(time_seconds: u32) -> std::result::Result<i64, Error> {
    let now = NaiveTime::from_num_seconds_from_midnight_opt(time_seconds, 0);

    if now.is_none() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Invalid `time_seconds` provided",
        ));
    }
    let diff = NaiveTime::signed_duration_since(now.unwrap(), Utc::now().time()).num_seconds();

    Ok(diff)
}
