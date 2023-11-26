use std::collections::HashMap;

use chrono::{DateTime, Duration, Local, TimeZone};

pub fn generate_discord_timestamp(timestamp: i64, format: &str) -> Option<String> {
    let mut generated = String::new();
    generated.push_str("<t:");
    generated.push_str(&timestamp.to_string());

    if !format.is_empty() {
        generated.push_str(":");
        generated.push_str(format);
    }

    generated.push_str(">");

    Some(generated)
}

pub fn generate_date_time_formats<'a>(timestamp: i64, twelve_hours_clock: Option<bool>) -> HashMap<String, String> {
    let date_time = Local.timestamp_opt(timestamp, 0).unwrap();
    let twelve_hours_clock = twelve_hours_clock.unwrap_or(false);

    let time_format = if twelve_hours_clock {
        "%I:%M %p"
    } else {
        "%H:%M"
    };

    let time_format_seconds = if twelve_hours_clock {
        "%I:%M:%S %p"
    } else {
        "%H:%M:%S"
    };

    let formats = [
        ("", &*format!("%B %d, %Y {}", time_format)),
        ("D", "%B %d, %Y"),
        ("f", &*format!("%B %e, %Y {}", time_format)),
        ("F", &*format!("%A, %e %B, %Y {}", time_format)),
        ("d", "%m/%d/%Y"),
        ("T", time_format_seconds),
        ("t", time_format),
        ("R", &relative_date_string(date_time)),
    ];

    let mut results = HashMap::new();

    for (key, format) in formats {
        let date_str = date_time.format(format).to_string();
        results.insert(date_str, key.to_string());
    }

    results
}

pub fn relative_date_string(date_time: DateTime<Local>) -> String {
    let now = Local::now();
    let now_date = now.date_naive();
    let date_time_date = date_time.date_naive();
    let duration = now.signed_duration_since(date_time);

    // Check the sign of the duration
    let in_future = duration.num_seconds() < 0;

    // remove sign from duration
    let duration = Duration::seconds(duration.num_seconds().abs());

    let date_string = if now_date == date_time_date {
        "today".to_string()
    } else if now_date.pred_opt().unwrap() == date_time_date {
        "yesterday".to_string()
    } else if now_date.succ_opt().unwrap() == date_time_date {
        "tomorrow".to_string()
    } else {
        date_time.format("%B %e, %Y").to_string()
    };

    let time_string = if duration.num_seconds() < 60 {
        format!("{} seconds", duration.num_seconds())
    } else if duration.num_minutes() < 60 {
        format!("{} minutes", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{} hours", duration.num_hours())
    } else if duration.num_days() < 7 {
        format!("{} days", duration.num_days())
    } else if duration.num_weeks() < 4 {
        format!("{} weeks", duration.num_weeks())
    } else if duration.num_weeks() < 52 {
        format!("{} months", duration.num_weeks() / 4)
    } else {
        return format!("{}", date_string);
    };

    if in_future {
        format!("in {}", time_string)
    } else {
        format!("{} ago", time_string)
    }
}