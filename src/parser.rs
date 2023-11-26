use std::str::FromStr;

use chrono::{Duration, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use regex::Regex;

use crate::constants::{*};
use crate::re::{DATE_TIME_REGEX, TIME_REGEX};

pub fn parse_timestamp_from_str(input: &str) -> Option<i64> {
    let input = input.trim();

    if let Some(timestamp) = parse_date_or_time(input) {
        return Some(timestamp);
    }

    if input.starts_with('+') || input.starts_with('-') {
        if let Some(timestamp) = parse_multiple_offsets(input) {
            return Some(timestamp);
        }
    }

    None
}


pub fn parse_date_or_time(date: &str) -> Option<i64> {
    if DATE_TIME_REGEX.is_match(date) {
        for format in &DATE_TIME_FORMATS {
            let from = NaiveDateTime::parse_from_str(date, format);

            if from.is_ok() {
                let date_time = Local.from_local_datetime(&from.unwrap()).unwrap();
                return Some(date_time.timestamp());
            }
        }
    }

    // get current date and append time to it
    if TIME_REGEX.is_match(date) {
        for format in &TIME_FORMATS {
            match NaiveTime::parse_from_str(date, format) {
                Ok(dt) => {
                    let now = Local::now().date_naive();
                    let naive_date_time = NaiveDateTime::new(now, dt);

                    let date_time = Local.from_local_datetime(&naive_date_time).unwrap();

                    return Some(date_time.timestamp());
                }
                Err(_) => {}
            }
        }
    }


    None
}

pub fn parse_multiple_offsets(date_time: &str) -> Option<i64> {
    let re = Regex::new(r"([+-]\d+[smhd])").unwrap();
    let offsets = re.captures_iter(date_time).collect::<Vec<_>>();

    let mut total_duration = Duration::seconds(0);

    for offset in offsets {
        let offset_str = &offset[1];
        let (sign, rest) = offset_str.split_at(1);

        let (value, unit) = rest.split_at(rest.len() - 1);

        let value = match i64::from_str(value) {
            Ok(v) => v,
            Err(_) => continue
        };

        let duration = match unit {
            "s" => Duration::seconds(value),
            "m" => Duration::minutes(value),
            "h" => Duration::hours(value),
            "d" => Duration::days(value),
            _ => continue
        };

        total_duration = if sign == "+" {
            total_duration + duration
        } else {
            total_duration - duration
        };
    }

    let dt = Utc::now() + total_duration;

    Some(dt.timestamp())
}