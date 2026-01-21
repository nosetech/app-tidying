//! アプリケーション操作
//!
//! アプリケーションの起動・活性化、プロセスID取得、実行中アプリ一覧取得などを行います。

use serde_json::{json, Value};

use crate::applescript::osascript::run_osascript;
use crate::applescript::utils::escape_applescript_string;

/// アプリケーション起動エラー
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

/// アプリケーション起動結果
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AppLaunchResult {
    /// 実行結果のステータス（"success" など）
    pub status: String,
    /// 結果メッセージ
    pub message: String,
    /// プロセスID（取得できた場合）
    pub process_id: Option<i32>,
    /// アプリケーションが既に起動していたかどうか
    pub was_already_running: bool,
}

impl AppLaunchResult {
    /// JSON オブジェクトに変換
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

/// 実行中のアプリケーション情報
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AppInfo {
    /// アプリケーション名
    pub name: String,
    /// プロセスID（取得できた場合）
    pub process_id: Option<i32>,
}

impl AppInfo {
    /// JSON オブジェクトに変換
    #[allow(dead_code)]
    pub fn to_json(&self) -> Value {
        let mut obj = json!({
            "name": self.name,
        });

        if let Some(pid) = self.process_id {
            obj["process_id"] = Value::Number(pid.into());
        }

        obj
    }
}

/// 実行中アプリケーション一覧取得エラー
#[derive(Debug)]
#[allow(dead_code)]
pub struct RunningAppsError {
    pub message: String,
}

impl std::fmt::Display for RunningAppsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RunningAppsError {}

/// アプリケーションが既に起動しているかを確認
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

/// 実行中アプリケーションのプロセスIDを取得
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

/// アプリケーションを起動
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

/// 起動済みアプリケーションを活性化（フォアグラウンドに表示）
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

/// アプリケーションを起動または活性化
///
/// アプリケーションが未起動の場合は起動し、既に起動している場合は活性化（フォアグラウンドに表示）します。
///
/// # Arguments
/// * `app_name` - アプリケーション名（例: "Google Chrome", "Safari"）
/// * `timeout_ms` - アプリケーション起動待機時間（ミリ秒）
///
/// # Returns
/// * `Ok(AppLaunchResult)` - 処理成功
/// * `Err(AppLaunchError)` - 失敗
///
/// # Examples
/// ```ignore
/// use apptidying::applescript::launch_or_activate_app;
///
/// let result = launch_or_activate_app("Safari", 3000)?;
/// println!("Status: {}", result.status);
/// println!("Message: {}", result.message);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[allow(dead_code)]
pub fn launch_or_activate_app(
    app_name: &str,
    timeout_ms: u64,
) -> Result<AppLaunchResult, AppLaunchError> {
    // アプリが既に起動しているかを確認
    let was_already_running = is_app_running(app_name)?;

    if was_already_running {
        // 起動済みアプリケーションを活性化（フォアグラウンドに表示）
        activate_app(app_name)?;

        // プロセスIDを取得
        let process_id = get_app_process_id(app_name)?;

        return Ok(AppLaunchResult {
            status: "success".to_string(),
            message: format!("アプリケーション '{}' は既に起動しています", app_name),
            process_id,
            was_already_running: true,
        });
    }

    // アプリケーションを起動
    launch_app(app_name)?;

    // アプリケーション起動完了を待機（ミリ秒単位で精密なタイミング）
    std::thread::sleep(std::time::Duration::from_millis(timeout_ms));

    // プロセスIDを取得
    let process_id = get_app_process_id(app_name)?;

    Ok(AppLaunchResult {
        status: "success".to_string(),
        message: format!("アプリケーション '{}' を起動しました", app_name),
        process_id,
        was_already_running: false,
    })
}

/// 実行中のアプリケーション一覧を取得
///
/// macOS で現在実行中のアプリケーション一覧を取得します。
///
/// # Returns
/// * `Ok(Vec<AppInfo>)` - 実行中のアプリケーション一覧
/// * `Err(RunningAppsError)` - 失敗
///
/// # Examples
/// ```ignore
/// use apptidying::applescript::get_running_applications;
///
/// let apps = get_running_applications()?;
/// for app in apps {
///     println!("{}: {:?}", app.name, app.process_id);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[allow(dead_code)]
pub fn get_running_applications() -> Result<Vec<AppInfo>, RunningAppsError> {
    let script = r#"
tell application "System Events"
    set appList to {}
    set procList to (name of every process whose background only is false)
    repeat with procName in procList
        try
            set procId to unix id of application process procName
            set end of appList to procName & "|" & procId
        on error
            set end of appList to procName & "|"
        end try
    end repeat
    return appList
end tell
"#;

    let output = run_osascript(script).map_err(|e| RunningAppsError { message: e.message })?;

    if !output.status.success() {
        return Err(RunningAppsError {
            message: format!(
                "実行中アプリケーション一覧の取得に失敗しました: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    let result_str = String::from_utf8_lossy(&output.stdout);
    let mut apps = Vec::new();

    // AppleScript は単一行のカンマ区切りリストを返す
    // 各エントリをトリムして処理
    let entries: Vec<&str> = result_str.split(',').collect();

    for entry in entries {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }

        // "app_name|process_id" または "app_name|" の形式をパース
        if let Some(pipe_pos) = entry.rfind('|') {
            let app_name = entry[..pipe_pos].to_string();
            let pid_str = &entry[pipe_pos + 1..];

            let process_id = if pid_str.is_empty() {
                None
            } else {
                pid_str.parse::<i32>().ok()
            };

            apps.push(AppInfo {
                name: app_name,
                process_id,
            });
        }
    }

    if apps.is_empty() {
        return Err(RunningAppsError {
            message: "実行中のアプリケーションが見つかりません".to_string(),
        });
    }

    Ok(apps)
}
