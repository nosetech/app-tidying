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

    /// JSONオブジェクトからDisplayInfoを生成する
    pub(crate) fn from_json_value(value: &serde_json::Value) -> Result<Self, DisplayError> {
        Ok(DisplayInfo {
            name: value["name"]
                .as_str()
                .ok_or_else(|| DisplayError {
                    message: "ディスプレイ名の取得に失敗しました".to_string(),
                })?
                .to_string(),
            width: value["width"].as_i64().ok_or_else(|| DisplayError {
                message: "ディスプレイ幅の取得に失敗しました".to_string(),
            })? as i32,
            height: value["height"].as_i64().ok_or_else(|| DisplayError {
                message: "ディスプレイ高さの取得に失敗しました".to_string(),
            })? as i32,
            origin_x: value["origin_x"].as_i64().ok_or_else(|| DisplayError {
                message: "ディスプレイ原点X座標の取得に失敗しました".to_string(),
            })? as i32,
            origin_y: value["origin_y"].as_i64().ok_or_else(|| DisplayError {
                message: "ディスプレイ原点Y座標の取得に失敗しました".to_string(),
            })? as i32,
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

/// 指定されたディスプレイ情報を取得する
///
/// 指定されたディスプレイ名に一致するディスプレイを返します。
/// 見つからない場合やdisplay_nameがNoneの場合は、メインディスプレイを返します。
#[allow(dead_code)]
pub fn get_display_info(display_name: Option<&str>) -> Result<DisplayInfo, DisplayError> {
    // すべての接続ディスプレイを取得
    let all_displays = get_all_connected_displays()?;

    if all_displays.is_empty() {
        return Err(DisplayError {
            message: "接続されているディスプレイが見つかりません".to_string(),
        });
    }

    // 指定されたディスプレイ名を検索
    if let Some(name) = display_name {
        if !name.is_empty() {
            if let Some(display) = all_displays.iter().find(|d| d.name == name) {
                return Ok(display.clone());
            }
        }
    }

    // 見つからない場合またはdisplay_nameがNoneの場合は、メインディスプレイ（最初のディスプレイ）を返す
    Ok(all_displays.into_iter().next().unwrap())
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

    // Reuse parse_single_window for consistent parsing
    parse_single_window(&result_str)
}

// =============================================================================
// Running Applications
// =============================================================================

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AppInfo {
    pub name: String,
    pub process_id: Option<i32>,
}

impl AppInfo {
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

/// Get list of running applications
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

    // AppleScript returns a comma-separated list on a single line
    // Split by comma and trim each entry
    let entries: Vec<&str> = result_str.split(',').collect();

    for entry in entries {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }

        // Parse the entry with format "app_name|process_id" or "app_name|"
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

// =============================================================================
// Get All Windows for an Application
// =============================================================================

/// Parse a single window entry from AppleScript output
/// Format: title|x,y|w,h|minimized|visible
#[allow(dead_code)]
pub fn parse_single_window(entry: &str) -> Result<WindowInfo, WindowInfoError> {
    let parts: Vec<&str> = entry.split('|').collect();
    if parts.len() < 5 {
        return Err(WindowInfoError {
            message: format!("ウィンドウ情報の形式が不正です: {}", entry),
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
    let minimized = parts[3].parse::<bool>().map_err(|_| WindowInfoError {
        message: format!("ウィンドウの最小化状態が無効です: {}", parts[3]),
    })?;

    // Parse visible state
    let visible = parts[4].parse::<bool>().map_err(|_| WindowInfoError {
        message: format!("ウィンドウの表示状態が無効です: {}", parts[4]),
    })?;

    Ok(WindowInfo {
        title,
        position: (position_x, position_y),
        size: (width, height),
        minimized,
        visible,
    })
}

/// Parse window list from AppleScript output
/// AppleScript returns comma-separated window entries, each with format:
/// title|x,y|w,h|minimized|visible
///
/// Example output from AppleScript:
/// "Main Window|0,25|1440,900|false|true,Settings|200,100|800,600|false|true"
///
/// Note: Since both the size/position (e.g., "800,600") and the entry separator
/// use commas, we parse by counting pipe characters (|) to determine when a comma
/// is an entry separator (after 4 pipes) vs. part of the data.
#[allow(dead_code)]
pub fn parse_window_list(result_str: &str) -> Result<Vec<WindowInfo>, WindowInfoError> {
    // Empty result means no windows
    if result_str.is_empty() {
        return Ok(vec![]);
    }

    let mut windows = Vec::new();
    let mut current_entry = String::new();
    let mut pipe_count = 0;

    for char in result_str.chars() {
        if char == '|' {
            // Count pipes as we encounter them for efficiency
            pipe_count += 1;
            current_entry.push(char);
        } else if char == ',' && pipe_count == 4 {
            // This comma is the entry separator (we've seen 4 pipes)
            let entry = current_entry.trim();
            if !entry.is_empty() {
                match parse_single_window(entry) {
                    Ok(window_info) => windows.push(window_info),
                    Err(e) => {
                        log::warn!("ウィンドウ情報のパースに失敗: {} - エントリ: {}", e, entry);
                    }
                }
            }
            current_entry.clear();
            pipe_count = 0;
        } else {
            current_entry.push(char);
        }
    }

    // Don't forget the last entry
    let entry = current_entry.trim();
    if !entry.is_empty() {
        match parse_single_window(entry) {
            Ok(window_info) => windows.push(window_info),
            Err(e) => {
                log::warn!("ウィンドウ情報のパースに失敗: {} - エントリ: {}", e, entry);
            }
        }
    }

    Ok(windows)
}

// =============================================================================
// System Window Detection
// =============================================================================

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum WindowType {
    /// Regular application window that can be managed
    Regular,
    /// System window that should be excluded (menu bar, dock, etc.)
    System,
}

/// Check if an application is a system application (Finder, Mail, Safari, etc.)
///
/// # Arguments
/// * `app_name` - The name of the application to check
///
/// # Returns
/// * `true` if the application is a macOS system application
/// * `false` otherwise
///
/// # System Apps List
/// This function recognizes 37 macOS system applications including:
/// - Core apps: Finder, Mail, Safari, Calendar, Notes, Maps, Messages, Contacts
/// - Utilities: Disk Utility, Terminal, Console, Activity Monitor
/// - Media: Music, TV, News, Podcasts, Weather, Stocks, Home
/// - iWork: Keynote, Numbers, Pages
/// - Development: Xcode
///
/// # Examples
/// ```
/// use apptidying::applescript::is_system_app;
///
/// assert!(is_system_app("Finder"));
/// assert!(is_system_app("Mail"));
/// assert!(is_system_app("Safari"));
/// assert!(!is_system_app("Google Chrome"));
/// assert!(!is_system_app("Visual Studio Code"));
/// ```
#[allow(dead_code)]
pub fn is_system_app(app_name: &str) -> bool {
    const SYSTEM_APPS: &[&str] = &[
        "Finder",
        "Mail",
        "Safari",
        "Calendar",
        "Notes",
        "Maps",
        "Messages",
        "Contacts",
        "Reminders",
        "Stocks",
        "Weather",
        "Podcasts",
        "News",
        "Home",
        "Music",
        "TV",
        "Books",
        "Dictionary",
        "Thesaurus",
        "Migration Assistant",
        "Photo Booth",
        "Preview",
        "TextEdit",
        "System Preferences",
        "System Settings",
        "Disk Utility",
        "Terminal",
        "Console",
        "Activity Monitor",
        "Bluetooth Screen Lock",
        "App Store",
        "iBooks",
        "Keynote",
        "Numbers",
        "Pages",
        "FileMerge",
        "Xcode",
    ];

    SYSTEM_APPS.contains(&app_name)
}

/// Check if a window should be excluded from management
///
/// Returns `true` if the window is a system UI element that should not be managed.
/// Checks both the application name and window title against known exclusion patterns.
///
/// # Arguments
/// * `app_name` - The name of the application
/// * `window_title` - The title of the window
///
/// # Returns
/// * `true` if the window is a system UI element that should be excluded
/// * `false` if the window can be managed
///
/// # Excluded Applications
/// - Dock, Menubar, WindowManager, LoginWindow, SystemUIServer
/// - ControlCenter, NotificationCenter, Spotlight
/// - Finder Sync UI, Quick Look, Accessibility Inspector
///
/// # Excluded Window Title Patterns
/// - Titles containing: "Menu", "Dock", "Notification", "Spotlight", "Control Center", "Accessibility Inspector"
///
/// # Examples
/// ```
/// use apptidying::applescript::is_excluded_window;
///
/// assert!(is_excluded_window("Dock", ""));
/// assert!(is_excluded_window("Finder", "Menu"));
/// assert!(!is_excluded_window("Finder", "Documents"));
/// ```
#[allow(dead_code)]
pub fn is_excluded_window(app_name: &str, window_title: &str) -> bool {
    // Exclude system UI processes
    const EXCLUDED_APP_NAMES: &[&str] = &[
        "Dock",
        "Menubar",
        "WindowManager",
        "LoginWindow",
        "SystemUIServer",
        "ControlCenter",
        "NotificationCenter",
        "Spotlight",
        "Finder Sync UI",
        "Quick Look",
        "Accessibility Inspector",
    ];

    if EXCLUDED_APP_NAMES.contains(&app_name) {
        return true;
    }

    // Exclude specific window titles that are system UI elements
    const EXCLUDED_WINDOW_TITLE_PATTERNS: &[&str] = &[
        "Menu",
        "Dock",
        "Notification",
        "Spotlight",
        "Control Center",
        "Accessibility Inspector",
    ];

    if EXCLUDED_WINDOW_TITLE_PATTERNS
        .iter()
        .any(|&pattern| window_title.contains(pattern))
    {
        return true;
    }

    false
}

/// Classify a window type based on application name and window properties
///
/// Determines whether a window should be managed or excluded based on its
/// application and title. This is a convenience function that wraps
/// `is_excluded_window()` and returns the appropriate `WindowType`.
///
/// # Arguments
/// * `app_name` - The name of the application
/// * `window_title` - The title of the window
///
/// # Returns
/// * `WindowType::System` if the window should be excluded from management
/// * `WindowType::Regular` if the window can be managed
///
/// # Examples
/// ```
/// use apptidying::applescript::{classify_window, WindowType};
///
/// // Regular manageable window
/// let result = classify_window("Finder", "Documents");
/// assert!(matches!(result, WindowType::Regular));
///
/// // System window that should be excluded
/// let result = classify_window("Dock", "");
/// assert!(matches!(result, WindowType::System));
/// ```
#[allow(dead_code)]
pub fn classify_window(app_name: &str, window_title: &str) -> WindowType {
    if is_excluded_window(app_name, window_title) {
        WindowType::System
    } else {
        WindowType::Regular
    }
}

/// Get all windows for a specific application
#[allow(dead_code)]
pub fn get_all_windows(app_name: &str) -> Result<Vec<WindowInfo>, WindowInfoError> {
    let script = format!(
        r#"
tell application "System Events"
    tell process "{}"
        try
            set windowList to every window
            set windowDataList to {{}}

            repeat with win in windowList
                try
                    set winTitle to title of win
                    set winPos to position of win
                    set winSize to size of win

                    try
                        set winMinimized to miniaturized of win
                    on error
                        set winMinimized to false
                    end try

                    try
                        set winVisible to visible of win
                    on error
                        set winVisible to true
                    end try

                    set windowData to winTitle & "|" & (item 1 of winPos) & "," & (item 2 of winPos) & "|" & (item 1 of winSize) & "," & (item 2 of winSize) & "|" & winMinimized & "|" & winVisible
                    set end of windowDataList to windowData
                on error
                    -- Skip this window
                end try
            end repeat

            return windowDataList
        on error errMsg
            return "error: " & errMsg
        end try
    end tell
end tell
"#,
        escape_applescript_string(app_name)
    );

    let output = run_osascript(&script).map_err(|e| WindowInfoError { message: e.message })?;

    if !output.status.success() {
        return Err(WindowInfoError {
            message: format!(
                "ウィンドウ一覧の取得に失敗しました: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Check for errors
    if result_str.starts_with("error:") {
        return Err(WindowInfoError {
            message: result_str,
        });
    }

    // Parse the results
    parse_window_list(&result_str)
}

// =============================================================================
// Get all connected displays
// =============================================================================

/// Get information about all connected displays
#[allow(dead_code)]
pub fn get_all_connected_displays() -> Result<Vec<DisplayInfo>, DisplayError> {
    let jxa_script = r#"
ObjC.import('AppKit')

const screens = $.NSScreen.screens
let displays = []

if (screens.count === 0) {
    JSON.stringify([])
} else {
    for (let i = 0; i < screens.count; i++) {
        const screen = screens.objectAtIndex(i)
        const displayName = ObjC.unwrap(screen.localizedName) || "Unknown"
        const frame = screen.frame

        const display = {
            name: displayName,
            width: Math.round(frame.size.width),
            height: Math.round(frame.size.height),
            origin_x: Math.round(frame.origin.x),
            origin_y: Math.round(frame.origin.y)
        }
        displays.push(display)
    }

    JSON.stringify(displays)
}
"#;

    let output = Command::new("osascript")
        .arg("-l")
        .arg("JavaScript")
        .arg("-e")
        .arg(jxa_script)
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

    let json_array: serde_json::Value =
        serde_json::from_str(&result_str).map_err(|e| DisplayError {
            message: format!("ディスプレイ情報のパースに失敗しました: {}", e),
        })?;

    let displays_array = json_array.as_array().ok_or_else(|| DisplayError {
        message: "ディスプレイ情報が配列形式ではありません".to_string(),
    })?;

    let mut displays = Vec::new();

    for display_value in displays_array {
        let display_info = DisplayInfo::from_json_value(display_value)?;
        displays.push(display_info);
    }

    Ok(displays)
}
