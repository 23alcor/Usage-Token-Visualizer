use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

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
        .invoke_handler(tauri::generate_handler![collect_claude_statusline])
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
}
