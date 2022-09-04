use chrono::{NaiveTime, Utc};

pub fn diference_in_secs_from_now(secs: usize) -> usize {
    let now = (Utc::now().timestamp() / 1000) as usize;

    return now - secs;
}
