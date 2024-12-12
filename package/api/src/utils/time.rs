use chrono::{DateTime, FixedOffset, MappedLocalTime, TimeZone, Utc};

pub fn convert_rfc3339_to_timestamp_millis(iso_str: &str) -> Option<i64> {
    DateTime::parse_from_rfc3339(iso_str)
        .ok()
        .map(|dt| dt.with_timezone(&Utc).timestamp_millis())
}

pub fn convert_utc_timestamp_to_datetime(
    timestamp: i64,
    adjusted_day: Option<u8>,
) -> Option<String> {
    let kst_offset = FixedOffset::east_opt(9 * 3600).unwrap(); // UTC+9
    let datetime = match kst_offset.timestamp_millis_opt(timestamp) {
        MappedLocalTime::Single(dt) => dt,
        _ => return None,
    };

    let datetime = if let Some(days) = adjusted_day {
        datetime + chrono::Duration::days(days as i64)
    } else {
        datetime
    };
    Some(datetime.format("%y-%m-%d").to_string())
}
