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

// =============================================================================
// Display Information
// =============================================================================

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DisplayInfo {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub origin_x: i32,
    pub origin_y: i32,
}

impl DisplayInfo {
    #[allow(dead_code)]
    pub fn to_json(&self) -> Value {
        json!({
            "name": self.name,
            "width": self.width,
            "height": self.height,
            "origin_x": self.origin_x,
            "origin_y": self.origin_y,
        })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DisplayError {
    pub message: String,
}

impl std::fmt::Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DisplayError {}

/// Get display information using JXA and AppleScript
#[allow(dead_code)]
pub fn get_display_info(display_name: Option<&str>) -> Result<DisplayInfo, DisplayError> {
    // Get all displays using JXA
    let jxa_script = r#"
ObjC.import('AppKit')

const screens = $.NSScreen.screens
let targetDisplay = null

if (screens.count === 0) {
    "error: no displays found"
} else {
    let searchName = null
    if (ObjC.unwrap(arguments[0]) !== null) {
        searchName = ObjC.unwrap(arguments[0])
    }

    for (let i = 0; i < screens.count; i++) {
        const screen = screens.objectAtIndex(i)
        const displayName = ObjC.unwrap(screen.localizedName) || "Unknown"

        if (searchName === null || displayName === searchName) {
            const frame = screen.frame
            const result = {
                name: displayName,
                width: Math.round(frame.size.width),
                height: Math.round(frame.size.height),
                origin_x: Math.round(frame.origin.x),
                origin_y: Math.round(frame.origin.y)
            }
            targetDisplay = result
            break
        }
    }

    if (targetDisplay === null && searchName !== null) {
        // Display with specified name not found, use main display
        const mainScreen = $.NSScreen.mainScreen
        const displayName = ObjC.unwrap(mainScreen.localizedName) || "Main"
        const frame = mainScreen.frame
        const result = {
            name: displayName,
            width: Math.round(frame.size.width),
            height: Math.round(frame.size.height),
            origin_x: Math.round(frame.origin.x),
            origin_y: Math.round(frame.origin.y)
        }
        targetDisplay = result
    } else if (targetDisplay === null) {
        // Use main display if no specific name requested
        const mainScreen = $.NSScreen.mainScreen
        const displayName = ObjC.unwrap(mainScreen.localizedName) || "Main"
        const frame = mainScreen.frame
        const result = {
            name: displayName,
            width: Math.round(frame.size.width),
            height: Math.round(frame.size.height),
            origin_x: Math.round(frame.origin.x),
            origin_y: Math.round(frame.origin.y)
        }
        targetDisplay = result
    }

    JSON.stringify(targetDisplay)
}
"#;

    let output = Command::new("osascript")
        .arg("-l")
        .arg("JavaScript")
        .arg("-e")
        .arg(jxa_script)
        .arg(display_name.unwrap_or(""))
        .output()
        .map_err(|e| DisplayError {
            message: format!("osascriptの実行に失敗しました: {}", e),
        })?;

    if !output.status.success() {
        return Err(DisplayError {
            message: format!(
                "ディスプレイ情報取得に失敗しました: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let json_value: serde_json::Value =
        serde_json::from_str(&result_str).map_err(|e| DisplayError {
            message: format!("ディスプレイ情報のパースに失敗しました: {}", e),
        })?;

    let display_info = DisplayInfo {
        name: json_value["name"]
            .as_str()
            .ok_or_else(|| DisplayError {
                message: "ディスプレイ名の取得に失敗しました".to_string(),
            })?
            .to_string(),
        width: json_value["width"].as_i64().ok_or_else(|| DisplayError {
            message: "ディスプレイ幅の取得に失敗しました".to_string(),
        })? as i32,
        height: json_value["height"].as_i64().ok_or_else(|| DisplayError {
            message: "ディスプレイ高さの取得に失敗しました".to_string(),
        })? as i32,
        origin_x: json_value["origin_x"]
            .as_i64()
            .ok_or_else(|| DisplayError {
                message: "ディスプレイ原点X座標の取得に失敗しました".to_string(),
            })? as i32,
        origin_y: json_value["origin_y"]
            .as_i64()
            .ok_or_else(|| DisplayError {
                message: "ディスプレイ原点Y座標の取得に失敗しました".to_string(),
            })? as i32,
    };

    Ok(display_info)
}

// =============================================================================
// Window Resize
// =============================================================================

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WindowResizeResult {
    pub status: String,
    pub message: String,
    pub new_position: Option<(i32, i32)>,
    pub new_size: Option<(i32, i32)>,
}

impl WindowResizeResult {
    #[allow(dead_code)]
    pub fn to_json(&self) -> Value {
        let mut obj = json!({
            "status": self.status,
            "message": self.message,
        });

        if let Some((x, y)) = self.new_position {
            obj["new_position"] = json!({"x": x, "y": y});
        }

        if let Some((width, height)) = self.new_size {
            obj["new_size"] = json!({"width": width, "height": height});
        }

        obj
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct WindowResizeError {
    pub message: String,
}

impl std::fmt::Display for WindowResizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for WindowResizeError {}

/// Resize and move a window
#[allow(dead_code)]
pub fn resize_window(
    app_name: &str,
    window_title: Option<&str>,
    position: Option<(i32, i32)>,
    size: Option<(i32, i32)>,
) -> Result<WindowResizeResult, WindowResizeError> {
    // Build AppleScript to resize window
    let mut script = format!(
        r#"
tell application "System Events"
    try
        tell process "{}"
"#,
        escape_applescript_string(app_name)
    );

    // Select window by title or use first window
    if let Some(title) = window_title {
        script.push_str(&format!(
            r#"
            set targetWindow to first window whose name contains "{}"
"#,
            escape_applescript_string(title)
        ));
    } else {
        script.push_str(
            r#"
            set targetWindow to window 1
"#,
        );
    }

    // Set position if provided
    if let Some((x, y)) = position {
        script.push_str(&format!(
            r#"
            set position of targetWindow to {{{}, {}}}
"#,
            x, y
        ));
    }

    // Set size if provided
    if let Some((width, height)) = size {
        script.push_str(&format!(
            r#"
            set size of targetWindow to {{{}, {}}}
"#,
            width, height
        ));
    }

    script.push_str(
        r#"
        end tell
        return "success"
    on error errMsg
        return "error: " & errMsg
    end try
end tell
"#,
    );

    let output = run_osascript(&script).map_err(|e| WindowResizeError { message: e.message })?;

    if !output.status.success() {
        return Err(WindowResizeError {
            message: format!(
                "ウィンドウのリサイズに失敗しました: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if result_str.contains("error") {
        return Err(WindowResizeError {
            message: result_str,
        });
    }

    Ok(WindowResizeResult {
        status: "success".to_string(),
        message: "ウィンドウをリサイズしました".to_string(),
        new_position: position,
        new_size: size,
    })
}

// =============================================================================
// Window Information
// =============================================================================

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WindowInfo {
    pub title: String,
    pub position: (i32, i32),
    pub size: (i32, i32),
    pub minimized: bool,
    pub visible: bool,
}

impl WindowInfo {
    #[allow(dead_code)]
    pub fn to_json(&self) -> Value {
        json!({
            "title": self.title,
            "position": {
                "x": self.position.0,
                "y": self.position.1
            },
            "size": {
                "width": self.size.0,
                "height": self.size.1
            },
            "minimized": self.minimized,
            "visible": self.visible
        })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct WindowInfoError {
    pub message: String,
}

impl std::fmt::Display for WindowInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for WindowInfoError {}

/// Get window information for a specific window
#[allow(dead_code)]
pub fn get_window_info(
    app_name: &str,
    window_title: Option<&str>,
) -> Result<WindowInfo, WindowInfoError> {
    let mut script = format!(
        r#"
tell application "System Events"
    tell process "{}"
        try
"#,
        escape_applescript_string(app_name)
    );

    // Select window by title or use first window
    if let Some(title) = window_title {
        script.push_str(&format!(
            r#"
            set targetWindow to first window whose name contains "{}"
"#,
            escape_applescript_string(title)
        ));
    } else {
        script.push_str(
            r#"
            set targetWindow to window 1
"#,
        );
    }

    script.push_str(
        r#"
            set winPos to position of targetWindow
            set winSize to size of targetWindow
            set winTitle to title of targetWindow

            try
                set winMinimized to miniaturized of targetWindow
            on error
                set winMinimized to false
            end try

            try
                set winVisible to visible of targetWindow
            on error
                set winVisible to true
            end try

            return winTitle & "|" & (item 1 of winPos) & "," & (item 2 of winPos) & "|" & (item 1 of winSize) & "," & (item 2 of winSize) & "|" & winMinimized & "|" & winVisible
        on error errMsg
            return "error: " & errMsg
        end try
    end tell
end tell
"#,
    );

    let output = run_osascript(&script).map_err(|e| WindowInfoError { message: e.message })?;

    if !output.status.success() {
        return Err(WindowInfoError {
            message: format!(
                "ウィンドウ情報の取得に失敗しました: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if result_str.starts_with("error:") {
        return Err(WindowInfoError {
            message: result_str,
        });
    }

    // Parse the result
    let parts: Vec<&str> = result_str.split('|').collect();
    if parts.len() < 5 {
        return Err(WindowInfoError {
            message: "ウィンドウ情報の解析に失敗しました".to_string(),
        });
    }

    let title = parts[0].to_string();

    // Parse position
    let pos_parts: Vec<&str> = parts[1].split(',').collect();
    if pos_parts.len() != 2 {
        return Err(WindowInfoError {
            message: "ウィンドウ位置の解析に失敗しました".to_string(),
        });
    }
    let position_x = pos_parts[0].parse::<i32>().map_err(|_| WindowInfoError {
        message: "ウィンドウのx座標が無効です".to_string(),
    })?;
    let position_y = pos_parts[1].parse::<i32>().map_err(|_| WindowInfoError {
        message: "ウィンドウのy座標が無効です".to_string(),
    })?;

    // Parse size
    let size_parts: Vec<&str> = parts[2].split(',').collect();
    if size_parts.len() != 2 {
        return Err(WindowInfoError {
            message: "ウィンドウサイズの解析に失敗しました".to_string(),
        });
    }
    let width = size_parts[0].parse::<i32>().map_err(|_| WindowInfoError {
        message: "ウィンドウの幅が無効です".to_string(),
    })?;
    let height = size_parts[1].parse::<i32>().map_err(|_| WindowInfoError {
        message: "ウィンドウの高さが無効です".to_string(),
    })?;

    // Parse minimized state
    let minimized = parts[3].parse::<bool>().unwrap_or(false);

    // Parse visible state
    let visible = parts[4].parse::<bool>().unwrap_or(true);

    Ok(WindowInfo {
        title,
        position: (position_x, position_y),
        size: (width, height),
        minimized,
        visible,
    })
}
