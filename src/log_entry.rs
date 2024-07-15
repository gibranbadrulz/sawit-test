use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub service_name: String,
    pub status_code: u16,
    pub response_time_ms: u32,
    pub user_id: String,
    pub transaction_id: String,
    pub additional_info: String,
}

/// function to parse a log line into a LogEntry struct
pub fn parse_log_line(line: &str, timestamp: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 8 {
        return None;
    }

    let response_time_ms_str = parts[4].trim_end_matches("ms");
    let response_time_ms = response_time_ms_str.parse().ok()?;

    Some(LogEntry {
        timestamp: timestamp.to_string(),
        service_name: parts[2].to_string(),
        status_code: parts[3].parse().ok()?,
        response_time_ms, // Store as u32
        user_id: parts[5].to_string(),
        transaction_id: parts[6].to_string(),
        additional_info: parts[7..].join(" "),
    })
}
