//! ユーティリティ関数
//!
//! AppleScript 文字列エスケープ、ウィンドウ情報パース、
//! ウィンドウタイプ判定などのユーティリティ関数を提供します。

use crate::applescript::window::{WindowInfo, WindowInfoError};

/// AppleScript 文字列をエスケープする
///
/// AppleScript 文字列リテラル内で特殊文字をエスケープします。
///
/// # Arguments
/// * `s` - エスケープ対象の文字列
///
/// # Returns
/// エスケープ処理済みの文字列
///
/// # Examples
/// ```
/// use apptidying::applescript::escape_applescript_string;
///
/// assert_eq!(escape_applescript_string("Hello"), "Hello");
/// assert_eq!(escape_applescript_string("Hello \"World\""), "Hello \\\"World\\\"");
/// assert_eq!(escape_applescript_string("Line1\nLine2"), "Line1\\nLine2");
/// ```
pub fn escape_applescript_string(s: &str) -> String {
    s.replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
}

/// ウィンドウの分類タイプ
///
/// ウィンドウが管理対象かシステムUI要素かを分類します。
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum WindowType {
    /// 通常のアプリケーションウィンドウ（管理対象）
    Regular,
    /// システムウィンドウ（除外対象）
    System,
}

/// システムアプリケーションかどうかを判定
///
/// 指定されたアプリケーション名が macOS のシステムアプリケーションであるかを判定します。
///
/// # Arguments
/// * `app_name` - チェック対象のアプリケーション名
///
/// # Returns
/// * `true` - macOS システムアプリケーション
/// * `false` - それ以外のアプリケーション
///
/// # システムアプリケーション一覧
/// 37個の macOS システムアプリケーションを認識します：
/// - コアアプリ: Finder, Mail, Safari, Calendar, Notes, Maps, Messages, Contacts
/// - ユーティリティ: Disk Utility, Terminal, Console, Activity Monitor
/// - メディア: Music, TV, News, Podcasts, Weather, Stocks, Home
/// - iWork: Keynote, Numbers, Pages
/// - 開発: Xcode
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

/// ウィンドウが除外対象かどうかを判定
///
/// ウィンドウがシステムUI要素で、ウィンドウ管理の対象外かどうかを判定します。
/// アプリケーション名またはウィンドウタイトルに基づいて判定します。
///
/// # Arguments
/// * `app_name` - アプリケーション名
/// * `window_title` - ウィンドウタイトル
///
/// # Returns
/// * `true` - ウィンドウは除外対象（管理しない）
/// * `false` - ウィンドウは管理可能
///
/// # 除外アプリケーション
/// - Dock, Menubar, WindowManager, LoginWindow, SystemUIServer
/// - ControlCenter, NotificationCenter, Spotlight
/// - Finder Sync UI, Quick Look, Accessibility Inspector
///
/// # 除外ウィンドウタイトルパターン
/// - "Menu", "Dock", "Notification", "Spotlight", "Control Center", "Accessibility Inspector" を含む
///
/// # Examples
/// ```
/// use apptidying::applescript::is_excluded_window;
///
/// assert!(is_excluded_window("Dock", ""));
/// assert!(is_excluded_window("Finder", "Menu"));
/// assert!(!is_excluded_window("Finder", "Documents"));
/// ```
pub fn is_excluded_window(app_name: &str, window_title: &str) -> bool {
    // システムUIプロセスを除外
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

    // システムUIウィンドウのタイトルパターンを除外
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

/// ウィンドウタイプを分類
///
/// アプリケーション名とウィンドウタイトルに基づいて、
/// ウィンドウが管理対象かシステムUI要素かを分類します。
///
/// # Arguments
/// * `app_name` - アプリケーション名
/// * `window_title` - ウィンドウタイトル
///
/// # Returns
/// * `WindowType::System` - ウィンドウは管理対象外
/// * `WindowType::Regular` - ウィンドウは管理可能
///
/// # Examples
/// ```
/// use apptidying::applescript::{classify_window, WindowType};
///
/// // 通常の管理可能なウィンドウ
/// let result = classify_window("Finder", "Documents");
/// assert!(matches!(result, WindowType::Regular));
///
/// // システムウィンドウ（除外対象）
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

/// AppleScript 出力から単一ウィンドウ情報をパース
///
/// AppleScript から返されたウィンドウ情報文字列を解析して WindowInfo を生成します。
///
/// # Format
/// `title|x,y|w,h|minimized|visible`
///
/// # Arguments
/// * `entry` - パース対象のウィンドウ情報文字列
///
/// # Returns
/// * `Ok(WindowInfo)` - パース成功
/// * `Err(WindowInfoError)` - パース失敗
///
/// # Examples
/// ```ignore
/// use apptidying::applescript::parse_single_window;
///
/// let result = parse_single_window("Main Window|0,25|1440,900|false|true")?;
/// assert_eq!(result.title, "Main Window");
/// assert_eq!(result.position, (0, 25));
/// assert_eq!(result.size, (1440, 900));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn parse_single_window(entry: &str) -> Result<WindowInfo, WindowInfoError> {
    let parts: Vec<&str> = entry.split('|').collect();
    if parts.len() < 5 {
        return Err(WindowInfoError {
            message: format!("ウィンドウ情報の形式が不正です: {}", entry),
        });
    }

    let title = parts[0].to_string();

    // 位置をパース
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

    // サイズをパース
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

    // 最小化状態をパース
    let minimized = parts[3].parse::<bool>().map_err(|_| WindowInfoError {
        message: format!("ウィンドウの最小化状態が無効です: {}", parts[3]),
    })?;

    // 表示状態をパース
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

/// AppleScript 出力からウィンドウリストをパース
///
/// AppleScript から返されたウィンドウリスト文字列を解析して Vec<WindowInfo> を生成します。
///
/// # Format
/// AppleScript はカンマ区切りのウィンドウエントリを返します。各エントリは以下の形式です：
/// `title|x,y|w,h|minimized|visible`
///
/// # Example Output
/// ```text
/// Main Window|0,25|1440,900|false|true,Settings|200,100|800,600|false|true
/// ```
///
/// **注意**: サイズと位置（例：`800,600`）とエントリセパレータの両方がカンマを使用するため、
/// パイプ文字（`|`）を数えることでパースを実現しています。4個のパイプを見た後のカンマが
/// エントリセパレータとなります。
///
/// # Arguments
/// * `result_str` - パース対象のウィンドウリスト文字列
///
/// # Returns
/// * `Ok(Vec<WindowInfo>)` - パース成功。空の結果は空のベクトルを返す
/// * `Err(WindowInfoError)` - 重大なパース失敗
pub fn parse_window_list(result_str: &str) -> Result<Vec<WindowInfo>, WindowInfoError> {
    // 空の結果はウィンドウなしを意味する
    if result_str.is_empty() {
        return Ok(vec![]);
    }

    let mut windows = Vec::new();
    let mut current_entry = String::new();
    let mut pipe_count = 0;

    for char in result_str.chars() {
        if char == '|' {
            // パイプをカウント（効率化のため）
            pipe_count += 1;
            current_entry.push(char);
        } else if char == ',' && pipe_count == 4 {
            // このカンマはエントリセパレータ（4個のパイプを見た）
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

    // 最後のエントリを忘れずに
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
