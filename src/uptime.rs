use std::fs;

pub fn get_uptime() -> String {
    let Ok(content) = fs::read_to_string("/proc/uptime") else {
        return String::from("Unknown");
    };

    let Some(first_field) = content.split_whitespace().next() else {
        return String::from("Unknown");
    };

    let Ok(total_seconds) = first_field.parse::<f64>() else {
        return String::from("Unknown");
    };

    let total_seconds = total_seconds as u64;

    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;

    if days > 0 {
        format!("{days}d {hours}h {minutes}m")
    } else if hours > 0 {
        format!("{hours}h {minutes}m")
    } else {
        format!("{minutes}m")
    }
}
