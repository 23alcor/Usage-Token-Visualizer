use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    thread,
    time::Duration,
};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuotaSnapshot {
    provider: String,
    pool_id: String,
    used_percentage: Option<f64>,
    resets_at: Option<String>,
    observed_at: String,
    source: String,
    quality: String,
}

#[derive(Debug, Deserialize)]
struct ClaudePayload { rate_limits: Option<RateLimits> }

#[derive(Debug, Deserialize)]
struct RateLimits { five_hour: Option<Limit>, seven_day: Option<Limit> }

#[derive(Debug, Deserialize)]
struct Limit { used_percentage: Option<f64>, resets_at: Option<serde_json::Value> }

#[tauri::command]
fn collect_claude_statusline(payload: String) -> Result<Vec<QuotaSnapshot>, String> {
    collect_and_store(&payload)
}

fn collect_and_store(payload: &str) -> Result<Vec<QuotaSnapshot>, String> {
    let parsed: ClaudePayload = serde_json::from_str(&payload)
        .map_err(|_| "The status-line input is not valid JSON.".to_owned())?;
    let limits = parsed.rate_limits;
    let observed_at = Utc::now().to_rfc3339();
    let snapshots = vec![
        snapshot("five-hour", limits.as_ref().and_then(|value| value.five_hour.as_ref()), &observed_at),
        snapshot("seven-day", limits.as_ref().and_then(|value| value.seven_day.as_ref()), &observed_at),
    ];
    append_to_local_history(&snapshots)?;
    Ok(snapshots)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClaudeTrackingSetup {
    status: String,
    message: String,
}

#[tauri::command]
fn setup_claude_code_tracking() -> Result<ClaudeTrackingSetup, String> {
    let claude_directory = claude_directory()?;
    fs::create_dir_all(&claude_directory).map_err(|error| error.to_string())?;
    let bridge_path = claude_directory.join("token-usage-statusline.sh");
    write_status_line_bridge(&bridge_path)?;

    let settings_path = claude_directory.join("settings.json");
    let mut settings = read_settings(&settings_path)?;
    let bridge_command = bridge_path.to_string_lossy().to_string();

    if let Some(existing) = settings.get("statusLine") {
        if existing.get("command").and_then(|value| value.as_str()) == Some(bridge_command.as_str()) {
            return Ok(ClaudeTrackingSetup {
                status: "ready".to_owned(),
                message: "Automatic Claude Code tracking is already enabled.".to_owned(),
            });
        }
        return Ok(ClaudeTrackingSetup {
            status: "manualConfigurationRequired".to_owned(),
            message: format!(
                "Claude Code already has a custom status line. It was left unchanged. Add this bridge to your existing status-line script: {}",
                bridge_path.display()
            ),
        });
    }

    settings["statusLine"] = serde_json::json!({
        "type": "command",
        "command": bridge_command,
        "refreshInterval": 60
    });
    fs::write(&settings_path, serde_json::to_vec_pretty(&settings).map_err(|error| error.to_string())?)
        .map_err(|error| error.to_string())?;

    Ok(ClaudeTrackingSetup {
        status: "installed".to_owned(),
        message: "Automatic Claude Code tracking is enabled. Send a Claude Code message to record your first reading.".to_owned(),
    })
}

fn claude_directory() -> Result<PathBuf, String> {
    dirs::home_dir()
        .map(|directory| directory.join(".claude"))
        .ok_or("Could not locate your home directory.".to_owned())
}

fn read_settings(path: &Path) -> Result<serde_json::Value, String> {
    if !path.exists() {
        return Ok(serde_json::json!({}));
    }
    let contents = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&contents)
        .map_err(|_| format!("Could not read existing Claude Code settings at {}. The file was not changed.", path.display()))?;
    if !settings.is_object() {
        return Err(format!("Claude Code settings at {} must contain a JSON object.", path.display()));
    }
    Ok(settings)
}

fn write_status_line_bridge(path: &Path) -> Result<(), String> {
    let bridge = r#"#!/bin/sh
# Installed by Token Usage. Claude Code sends status-line JSON on stdin.
curl --silent --fail --output /dev/null --connect-timeout 1 --max-time 2 \
  --request POST --header 'Content-Type: application/json' \
  --data-binary @- http://127.0.0.1:16472/claude-code
if [ $? -eq 0 ]; then
  printf 'Token Usage tracking active'
else
  printf 'Token Usage: open the app to track usage'
fi
"#;
    fs::write(path, bridge).map_err(|error| error.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn start_claude_status_line_listener(app: AppHandle) -> Result<(), String> {
    let listener = TcpListener::bind("127.0.0.1:16472")
        .map_err(|error| format!("Could not start the local Claude Code listener: {error}"))?;
    thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            handle_status_line_request(stream, &app);
        }
    });
    Ok(())
}

fn handle_status_line_request(mut stream: TcpStream, app: &AppHandle) {
    let result = read_http_body(&mut stream).and_then(|payload| collect_and_store(&payload));
    match result {
        Ok(snapshots) => {
            let _ = app.emit("claude-quota-updated", &snapshots);
            let _ = stream.write_all(b"HTTP/1.1 204 No Content\r\nConnection: close\r\n\r\n");
        }
        Err(error) => {
            let body = error.as_bytes();
            let header = format!("HTTP/1.1 400 Bad Request\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len());
            let _ = stream.write_all(header.as_bytes());
            let _ = stream.write_all(body);
        }
    }
}

fn read_http_body(stream: &mut TcpStream) -> Result<String, String> {
    stream.set_read_timeout(Some(Duration::from_secs(2))).map_err(|error| error.to_string())?;
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 4096];
    let header_end;
    loop {
        let read = stream.read(&mut chunk).map_err(|error| error.to_string())?;
        if read == 0 { return Err("The local status-line request was empty.".to_owned()); }
        buffer.extend_from_slice(&chunk[..read]);
        if let Some(position) = buffer.windows(4).position(|window| window == b"\r\n\r\n") {
            header_end = position + 4;
            break;
        }
        if buffer.len() > 16_384 { return Err("The local status-line request header is too large.".to_owned()); }
    }

    let headers = std::str::from_utf8(&buffer[..header_end]).map_err(|_| "The local status-line request headers are invalid.".to_owned())?;
    if !headers.starts_with("POST /claude-code ") { return Err("The local listener only accepts Claude Code status-line updates.".to_owned()); }
    let content_length = headers.lines()
        .find_map(|line| line.strip_prefix("Content-Length: ").or_else(|| line.strip_prefix("content-length: ")))
        .and_then(|value| value.trim().parse::<usize>().ok())
        .ok_or("The local status-line request is missing Content-Length.".to_owned())?;
    if content_length > 1_000_000 { return Err("The local status-line payload is too large.".to_owned()); }
    while buffer.len() < header_end + content_length {
        let read = stream.read(&mut chunk).map_err(|error| error.to_string())?;
        if read == 0 { return Err("The local status-line request ended before its payload.".to_owned()); }
        buffer.extend_from_slice(&chunk[..read]);
    }
    String::from_utf8(buffer[header_end..header_end + content_length].to_vec())
        .map_err(|_| "The local status-line payload is not UTF-8.".to_owned())
}

fn snapshot(pool_id: &str, limit: Option<&Limit>, observed_at: &str) -> QuotaSnapshot {
    let used_percentage = limit.and_then(|value| value.used_percentage);
    QuotaSnapshot {
        provider: "anthropic".to_owned(), pool_id: pool_id.to_owned(), used_percentage,
        resets_at: limit.and_then(|value| normalize_reset(value.resets_at.as_ref())),
        observed_at: observed_at.to_owned(), source: "claude-code-statusline".to_owned(),
        quality: if used_percentage.is_some() { "official" } else { "unavailable" }.to_owned(),
    }
}

fn normalize_reset(value: Option<&serde_json::Value>) -> Option<String> {
    match value? {
        serde_json::Value::String(value) => DateTime::parse_from_rfc3339(value).ok().map(|date| date.with_timezone(&Utc).to_rfc3339()),
        serde_json::Value::Number(value) => value.as_f64().and_then(|time| {
            let seconds = if time > 10_000_000_000.0 { time / 1_000.0 } else { time };
            DateTime::from_timestamp(seconds as i64, 0).map(|date| date.to_rfc3339())
        }),
        _ => None,
    }
}

fn history_path() -> Result<PathBuf, String> {
    let directory = dirs::data_local_dir().ok_or("Could not locate the local application-data directory.")?.join("Token Usage");
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    Ok(directory.join("claude-code-snapshots.json"))
}

fn append_to_local_history(snapshots: &[QuotaSnapshot]) -> Result<(), String> {
    let path = history_path()?;
    let mut history: Vec<QuotaSnapshot> = fs::read_to_string(&path).ok()
        .and_then(|contents| serde_json::from_str(&contents).ok()).unwrap_or_default();
    history.extend_from_slice(snapshots);
    fs::write(path, serde_json::to_vec_pretty(&history).map_err(|error| error.to_string())?)
        .map_err(|error| error.to_string())
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            start_claude_status_line_listener(app.handle().clone()).map_err(std::io::Error::other)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![collect_claude_statusline, setup_claude_code_tracking])
        .run(tauri::generate_context!())
        .expect("error while running Token Usage");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_provider_reported_usage_and_reset() {
        let payload: ClaudePayload = serde_json::from_str(
            r#"{"rate_limits":{"five_hour":{"used_percentage":35,"resets_at":"2026-10-02T03:00:00Z"}}}"#,
        ).unwrap();
        let snapshot = snapshot("five-hour", payload.rate_limits.unwrap().five_hour.as_ref(), "2026-10-01T00:00:00Z");

        assert_eq!(snapshot.used_percentage, Some(35.0));
        assert_eq!(snapshot.resets_at.as_deref(), Some("2026-10-02T03:00:00+00:00"));
        assert_eq!(snapshot.quality, "official");
    }

    #[test]
    fn missing_limit_is_unavailable() {
        let snapshot = snapshot("seven-day", None, "2026-10-01T00:00:00Z");

        assert_eq!(snapshot.used_percentage, None);
        assert_eq!(snapshot.quality, "unavailable");
    }

    #[test]
    fn parses_claude_code_epoch_reset_times() {
        let reset = normalize_reset(Some(&serde_json::json!(1_735_689_600_i64)));

        assert_eq!(reset.as_deref(), Some("2025-01-01T00:00:00+00:00"));
    }
}
