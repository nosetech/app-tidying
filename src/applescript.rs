use serde_json::{json, Value};
use std::process::Command;

#[derive(Debug)]
#[allow(dead_code)]
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
#[allow(dead_code)]
pub struct AppLaunchResult {
    pub status: String,
    pub message: String,
    pub process_id: Option<i32>,
    pub was_already_running: bool,
}

impl AppLaunchResult {
    #[allow(dead_code)]
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

/// Common function to execute osascript
fn run_osascript(script: &str) -> Result<std::process::Output, AppLaunchError> {
    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| AppLaunchError {
            message: format!("osascriptの実行に失敗しました: {}", e),
        })
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

    let output = run_osascript(&script)?;

    if !output.status.success() {
        return Err(AppLaunchError {
            message: format!(
                "アプリの起動状態確認に失敗しました: {}",
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

    let output = run_osascript(&script)?;

    if !output.status.success() {
        log::warn!(
            "プロセスID取得に失敗しました (アプリ: {}): {}",
            app_name,
            String::from_utf8_lossy(&output.stderr)
        );
        return Ok(None);
    }

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if result.is_empty() {
        return Ok(None);
    }

    result.parse::<i32>().map(Some).map_err(|_| AppLaunchError {
        message: format!("プロセスIDのパースに失敗しました: {}", result),
    })
}

/// Launch an application
fn launch_app(app_name: &str) -> Result<(), AppLaunchError> {
    let script = format!(
        r#"
tell application "{}"
    launch
    activate
    reopen
end tell
"#,
        escape_applescript_string(app_name)
    );

    let output = run_osascript(&script)?;

    if !output.status.success() {
        return Err(AppLaunchError {
            message: format!(
                "アプリの起動に失敗しました: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    Ok(())
}

/// Activate an already running application (bring it to foreground with windows visible)
fn activate_app(app_name: &str) -> Result<(), AppLaunchError> {
    let script = format!(
        r#"
tell application "{}"
    activate
    reopen
end tell
"#,
        escape_applescript_string(app_name)
    );

    let output = run_osascript(&script)?;

    if !output.status.success() {
        return Err(AppLaunchError {
            message: format!(
                "アプリの活性化に失敗しました: {}",
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
#[allow(dead_code)]
pub fn launch_or_activate_app(
    app_name: &str,
    timeout_ms: u64,
) -> Result<AppLaunchResult, AppLaunchError> {
    // Check if app is already running
    let was_already_running = is_app_running(app_name)?;

    if was_already_running {
        // Activate the already running application (bring it to foreground with windows visible)
        activate_app(app_name)?;

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

    // Wait for the app to fully launch (using milliseconds for accurate timing)
    std::thread::sleep(std::time::Duration::from_millis(timeout_ms));

    // Get the process ID
    let process_id = get_app_process_id(app_name)?;

    Ok(AppLaunchResult {
        status: "success".to_string(),
        message: format!("アプリケーション '{}' を起動しました", app_name),
        process_id,
        was_already_running: false,
    })
}
