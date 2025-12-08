use chrono::{NaiveDate, NaiveDateTime, Utc, Duration, Datelike, Local, TimeZone};
use regex::Regex;

pub fn parse_to_unix(input: &str) -> Option<i64> {
    let now = Local::now();
    
    // relative durations
    let re_relative = Regex::new(r"(?i)(?:(\d+)y)?(?:(\d+)m)?(?:(\d+)w)?(?:(\d+)d)?(?:(\d+)h)?(?:(\d+)min)?$").unwrap();
    if let Some(caps) = re_relative.captures(input) {
        let mut duration = Duration::seconds(0);
        if let Some(y) = caps.get(1) { duration += Duration::days(y.as_str().parse::<i64>().unwrap() * 365); }
        if let Some(m) = caps.get(2) { duration += Duration::days(m.as_str().parse::<i64>().unwrap() * 30); }
        if let Some(w) = caps.get(3) { duration += Duration::days(w.as_str().parse::<i64>().unwrap() * 7); }
        if let Some(d) = caps.get(4) { duration += Duration::days(d.as_str().parse::<i64>().unwrap()); }
        if let Some(h) = caps.get(5) { duration += Duration::hours(h.as_str().parse::<i64>().unwrap()); }
        if let Some(min) = caps.get(6) { duration += Duration::minutes(min.as_str().parse::<i64>().unwrap()); }
        if duration != Duration::seconds(0) {
            return Some((now + duration).timestamp());
        }
    }

    // time only: HH:MM (today, local)
    let re_time = Regex::new(r"^(\d{2}):(\d{2})$").unwrap();
    if let Some(caps) = re_time.captures(input) {
        let hour = caps.get(1).unwrap().as_str().parse::<u32>().ok()?;
        let minute = caps.get(2).unwrap().as_str().parse::<u32>().ok()?;
        let date = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())?;
        let datetime = NaiveDateTime::new(date, chrono::NaiveTime::from_hms_opt(hour, minute, 0)?);
        let local_dt = Local.from_local_datetime(&datetime).single()?;
        return Some(local_dt.timestamp());
    }

    // absolute date/time format: DDMM[YY][-HH:MM], local
    let re_abs = Regex::new(r"^(\d{2})(\d{2})(\d{2})?(?:-(\d{2}):(\d{2}))?$").unwrap();
    if let Some(caps) = re_abs.captures(input) {
        let day = caps.get(1).unwrap().as_str().parse::<u32>().ok()?;
        let month = caps.get(2).unwrap().as_str().parse::<u32>().ok()?;
        let year = if let Some(y) = caps.get(3) {
            2000 + y.as_str().parse::<i32>().ok()?  // assuming 2000+
        } else {
            now.year()
        };
        let hour = caps.get(4).map_or(0, |h| h.as_str().parse::<u32>().unwrap());
        let minute = caps.get(5).map_or(0, |m| m.as_str().parse::<u32>().unwrap());

        let date = NaiveDate::from_ymd_opt(year, month, day)?;
        let datetime = NaiveDateTime::new(date, chrono::NaiveTime::from_hms_opt(hour, minute, 0)?);
        let local_dt = Local.from_local_datetime(&datetime).single()?;
        return Some(local_dt.timestamp());
    }

    None
}

pub fn unix_to_relative(unix_time: i64) -> String {
    let now = Utc::now().timestamp();
    let mut seconds = unix_time - now;
    let negative = seconds < 0;
    if negative {
        seconds = -seconds;
    }

    let units = [
        ("y", 365 * 24 * 3600),
        ("mo", 30 * 24 * 3600),
        ("w", 7 * 24 * 3600),
        ("d", 24 * 3600),
        ("h", 3600),
        ("m", 60),
        ("s", 1),
    ];

    let mut values: Vec<(i64, &str)> = units
        .iter()
        .map(|&(name, unit_sec)| {
            let val = seconds / unit_sec;
            seconds %= unit_sec;
            (val, name)
        })
        .collect();

    let overflow_map = |unit: &str| match unit {
        "s" => 60,
        "m" => 60,
        "h" => 24,
        "d" => 7,
        "w" => 4,
        "mo" => 12,
        _ => 0
    };

    loop {
        let mut changed = false;
        for i in (1..values.len()).rev() {
            let (val, unit) = values[i];
            let overflow = overflow_map(unit);
            if overflow > 0 && val >= overflow {
                let carry = val / overflow;
                values[i].0 %= overflow;
                values[i - 1].0 += carry;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    let result: Vec<String> = values
        .iter()
        .filter(|&&(v, _)| v > 0)
        .take(2)
        .map(|&(v, u)| format!("{}{}", v, u))
        .collect();

    let output = if result.is_empty() { "0s".to_string() } else { result.join(" ") };

    if negative {
        format!("Overdue {}", output)
    } else {
        output
    }
}
