use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> String {
    let current = SystemTime::now();
    let since_epoch = current
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let mut timestamp = since_epoch.as_secs();

    // apply UTC+8 (8 hours * 3600 seconds)
    timestamp += 8 * 3600;

    let hours = (timestamp % 86400) / 3600;
    let minutes = (timestamp % 3600) / 60;
    let seconds = timestamp % 60;

    let is_pm = hours >= 12;
    let display_hour = if hours == 0 {
        12
    } else if hours > 12 {
        hours - 12
    } else {
        hours
    };

    let stamp = format!(
        "<dimmed>{:02}:{:02}:{:02} {}: </>",
        display_hour,
        minutes,
        seconds,
        if is_pm { "PM" } else { "AM" }
    );

    crate::formatter::colorize_string(stamp)
}
