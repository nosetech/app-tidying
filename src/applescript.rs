use serde_json::{json, Value};
use std::process::Command;

#[derive(Debug)]
pub struct AppLaunchError {
    pub message: String,
}

impl std::fmt::Display for AppLaunchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppLaunchError {}

#[derive(Debug, Clone)]
pub struct AppLaunchResult {
    pub status: String,
    pub message: String,
    pub process_id: Option<i32>,
    pub was_already_running: bool,
}

impl AppLaunchResult {
    pub fn to_json(&self) -> Value {
        let mut obj = json!({
            "status": self.status,
            "message": self.message,
            "was_already_running": self.was_already_running,
        });

        if let Some(pid) = self.process_id {
            obj["process_id"] = Value::Number(pid.into());
        }

        obj
    }
}

/// Check if an application is already running
fn is_app_running(app_name: &str) -> Result<bool, AppLaunchError> {
    let script = format!(
        r#"
tell application "System Events"
    try
        application process "{}" exists
    on error
        false
    end try
end tell
"#,
        escape_applescript_string(app_name)
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| AppLaunchError {
            message: format!("Failed to execute osascript: {}", e),
        })?;

    if !output.status.success() {
        return Err(AppLaunchError {
            message: format!(
                "Failed to check if app is running: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    let result = String::from_utf8_lossy(&output.stdout);
    Ok(result.trim() == "true")
}

/// Get the process ID of a running application
fn get_app_process_id(app_name: &str) -> Result<Option<i32>, AppLaunchError> {
    let script = format!(
        r#"
tell application "System Events"
    try
        unix id of application process "{}"
    on error
        ""
    end try
end tell
"#,
        escape_applescript_string(app_name)
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| AppLaunchError {
            message: format!("Failed to execute osascript: {}", e),
        })?;

    if !output.status.success() {
        return Ok(None);
    }

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if result.is_empty() {
        return Ok(None);
    }

    result.parse::<i32>().map(Some).map_err(|_| AppLaunchError {
        message: format!("Failed to parse process ID: {}", result),
    })
}

/// Launch an application
fn launch_app(app_name: &str) -> Result<(), AppLaunchError> {
    let script = format!(
        r#"
tell application "{}"
    launch
    activate
end tell
"#,
        escape_applescript_string(app_name)
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| AppLaunchError {
            message: format!("Failed to execute osascript: {}", e),
        })?;

    if !output.status.success() {
        return Err(AppLaunchError {
            message: format!(
                "Failed to launch app: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    Ok(())
}

/// Escape special characters in AppleScript strings
pub fn escape_applescript_string(s: &str) -> String {
    s.replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
}

/// Launch or activate an application
pub fn launch_or_activate_app(
    app_name: &str,
    timeout_ms: u64,
) -> Result<AppLaunchResult, AppLaunchError> {
    // Check if app is already running
    let was_already_running = is_app_running(app_name)?;

    if was_already_running {
        // Get the process ID
        let process_id = get_app_process_id(app_name)?;

        return Ok(AppLaunchResult {
            status: "success".to_string(),
            message: format!("アプリケーション '{}' は既に起動しています", app_name),
            process_id,
            was_already_running: true,
        });
    }

    // Launch the app
    launch_app(app_name)?;

    // Wait for the app to fully launch
    let timeout_secs = timeout_ms / 1000;
    std::thread::sleep(std::time::Duration::from_secs(timeout_secs));

    // Get the process ID
    let process_id = get_app_process_id(app_name)?;

    Ok(AppLaunchResult {
        status: "success".to_string(),
        message: format!("アプリケーション '{}' を起動しました", app_name),
        process_id,
        was_already_running: false,
    })
}
