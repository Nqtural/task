use chrono::{
    DateTime, Datelike, Duration, Local, NaiveDate, NaiveTime, TimeZone, Timelike, Utc,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Time {
    epoch: i64,
}

impl Time {
    pub fn new(epoch: i64) -> Self {
        Self { epoch }
    }

    pub fn epoch(self) -> i64 {
        self.epoch
    }

    pub fn from_str(input: &str) -> Option<Self> {
        let now = Local::now();

        parse_relative_duration(input, now)
            .or_else(|| parse_relative_clock(input, now))
            .or_else(|| parse_absolute_datetime(input, now))
            .map(|dt| Self::new(dt.with_timezone(&Utc).timestamp()))
    }

    pub fn format_relative(self) -> String {
        format_relative(self.epoch)
    }
}

fn parse_relative_duration(input: &str, base: DateTime<Local>) -> Option<DateTime<Local>> {
    let mut chars = input.chars().peekable();
    let mut years = 0;
    let mut months = 0;
    let mut seconds = 0i64;

    while chars.peek().is_some() {
        let mut num = 0i64;
        while let Some(c) = chars.peek().and_then(|c| c.to_digit(10)) {
            num = num * 10 + c as i64;
            chars.next();
        }

        match chars.next()? {
            'y' => {
                years += num;
                continue;
            }
            'm' => {
                if chars.peek() == Some(&'o') {
                    chars.next();
                    months += num;
                    continue;
                } else {
                    seconds += num * 60;
                    continue;
                }
            }
            'w' => {
                seconds += num * 7 * 24 * 3600;
                continue;
            }
            'd' => {
                seconds += num * 24 * 3600;
                continue;
            }
            'h' => {
                seconds += num * 3600;
                continue;
            }
            's' => {
                seconds += num;
                continue;
            }
            _ => return None,
        };
    }

    let mut dt = base;
    if years != 0 || months != 0 {
        dt = add_years_months(dt, years, months)?;
    }

    Some(dt + Duration::seconds(seconds))
}

fn add_years_months(
    dt: DateTime<Local>,
    years: i64,
    months: i64,
) -> Option<DateTime<Local>> {
    let mut year = dt.year() + years as i32;
    let mut month = dt.month() as i64 + months;

    while month > 12 {
        month -= 12;
        year += 1;
    }
    while month < 1 {
        month += 12;
        year -= 1;
    }

    let day = dt.day().min(days_in_month(year, month as u32));
    let date = NaiveDate::from_ymd_opt(year, month as u32, day)?;
    let time = dt.time();

    Local.from_local_datetime(&date.and_time(time)).single()
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let next = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }
    .unwrap();

    (next - Duration::days(1)).day()
}

fn parse_relative_clock(input: &str, now: DateTime<Local>) -> Option<DateTime<Local>> {
    let (h, m) = input.split_once(':')?;
    let hour: u32 = h.parse().ok()?;
    let minute: u32 = m.parse().ok()?;

    let today = now.date_naive();
    let time = NaiveTime::from_hms_opt(hour, minute, 0)?;

    let today_dt = Local.from_local_datetime(&today.and_time(time)).single()?;

    if today_dt > now {
        Some(today_dt)
    } else {
        Some(today_dt + Duration::days(1))
    }
}

fn parse_absolute_datetime(input: &str, now: DateTime<Local>) -> Option<DateTime<Local>> {
    let (date_part, time_part) = input.split_once('-').unwrap_or((input, "00:00"));

    if date_part.len() != 4 && date_part.len() != 6 {
        return None;
    }

    let day: u32 = date_part[0..2].parse().ok()?;
    let month: u32 = date_part[2..4].parse().ok()?;
    let year: i32 = if date_part.len() == 6 {
        2000 + date_part[4..6].parse::<i32>().ok()?
    } else {
        now.year()
    };

    let (h, m) = time_part.split_once(':')?;
    let hour: u32 = h.parse().ok()?;
    let minute: u32 = m.parse().ok()?;

    let date = NaiveDate::from_ymd_opt(year, month, day)?;
    let time = NaiveTime::from_hms_opt(hour, minute, 0)?;

    Local.from_local_datetime(&date.and_time(time)).single()
}

fn format_relative(epoch: i64) -> String {
    let now = Local::now();
    let target = Local.timestamp_opt(epoch, 0).single().unwrap();

    let mut past = false;
    let mut start = now;
    let mut end = target;
    if target < now {
        past = true;
        start = target;
        end = now;
    }

    let mut years = end.year() - start.year();
    let mut months = end.month() as i32 - start.month() as i32;
    let mut days = end.day() as i32 - start.day() as i32;
    let mut hours = end.hour() as i32 - start.hour() as i32;
    let mut minutes = end.minute() as i32 - start.minute() as i32;
    let mut seconds = end.second() as i32 - start.second() as i32;

    // normalize negative units
    if seconds < 0 {
        seconds += 60;
        minutes -= 1;
    }
    if minutes < 0 {
        minutes += 60;
        hours -= 1;
    }
    if hours < 0 {
        hours += 24;
        days -= 1;
    }
    if days < 0 {
        let prev_month = if end.month() == 1 { 12 } else { end.month() - 1 };
        let prev_year = if prev_month == 12 { end.year() - 1 } else { end.year() };
        days += days_in_month(prev_year, prev_month) as i32;
        months -= 1;
    }
    if months < 0 {
        months += 12;
        years -= 1;
    }

    let mut parts = Vec::new();
    if years > 0 {
        parts.push(format!("{years}y"));
    }
    if months > 0 {
        parts.push(format!("{months}mo"));
    }
    if days > 0 {
        parts.push(format!("{days}d"));
    }
    if hours > 0 {
        parts.push(format!("{hours}h"));
    }
    if minutes > 0 {
        parts.push(format!("{minutes}m"));
    }
    if seconds > 0 {
        parts.push(format!("{seconds}s"));
    }

    if parts.is_empty() {
        parts.push("0s".to_string());
    }

    let out: String = parts.iter().take(2).cloned().collect();
    if past {
        format!("Overdue {out}")
    } else {
        out
    }
}
