use chrono::{TimeZone, Utc};

pub fn format_prev_time(timestamp: i64) -> String {
    let now = Utc::now();

    let target_time = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .unwrap_or(Utc::now());

    let duration = now.signed_duration_since(target_time);

    if duration.num_seconds() < 60 {
        return format!("{}초 전", duration.num_seconds());
    } else if duration.num_minutes() < 60 {
        return format!("{}분 전", duration.num_minutes());
    } else if duration.num_hours() < 24 {
        return format!("{}시간 전", duration.num_hours());
    } else if duration.num_days() < 30 {
        return format!("{}일 전", duration.num_days());
    } else if duration.num_days() < 365 {
        let months = duration.num_days() / 30;
        return format!("{}개월 전", months);
    } else {
        let years = duration.num_days() / 365;
        return format!("{}년 전", years);
    }
}

pub fn formatted_timestamp(timestamp: i64) -> String {
    let datetime = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .expect("Invalid timestamp");

    datetime.format("%-m월 %-d일 %Y년").to_string()
}
