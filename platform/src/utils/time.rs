use chrono::{Datelike, Local, TimeZone, Utc};

pub fn convert_timestamp_to_fmt_string(timestamp: i64, format: &str) -> String {
    let date_time = Utc.timestamp_millis_opt(timestamp).unwrap();
    let local_date_time = date_time.with_timezone(&Local);
    local_date_time.format(format).to_string()
}

pub fn convert_timestamp_to_separate_string(timestamp: i64) -> (i32, u32, u32) {
    let date_time = Utc.timestamp_millis_opt(timestamp).unwrap();
    let local_date_time = date_time.with_timezone(&Local);

    (
        local_date_time.year(),
        local_date_time.month(),
        local_date_time.day(),
    )
}
