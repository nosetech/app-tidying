use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[allow(dead_code)]
const SUPPORTED_VERSION: &str = "1.0";

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionValue {
    Left,
    Right,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalPositionValue {
    Top,
    Bottom,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeValue {
    Half,
    Third,
    Max,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    #[serde(default)]
    pub x: serde_json::Value,
    #[serde(default)]
    pub y: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Size {
    #[serde(default)]
    pub width: serde_json::Value,
    #[serde(default)]
    pub height: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppWindowConfig {
    pub app: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub position: Option<Position>,
    #[serde(default)]
    pub size: Option<Size>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DisplayConfig {
    pub name: String,
    pub windows: Vec<AppWindowConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LayoutConfig {
    pub name: String,
    pub displays: Vec<DisplayConfig>,
}

fn default_notification_info() -> String {
    "notification".to_string()
}

fn default_notification_warn() -> String {
    "notification".to_string()
}

fn default_notification_error() -> String {
    "dialog".to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationConfig {
    #[serde(default = "default_notification_info")]
    pub info: String,
    #[serde(default = "default_notification_warn")]
    pub warn: String,
    #[serde(default = "default_notification_error")]
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub version: String,
    pub layouts: Vec<LayoutConfig>,
    #[serde(default)]
    pub notification: Option<NotificationConfig>,
    #[serde(default)]
    pub timeout: Option<u64>,
}

#[derive(Debug)]
pub struct AppConfigError {
    pub message: String,
}

impl std::fmt::Display for AppConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppConfigError {}

#[allow(dead_code)]
pub fn get_config_dir() -> Result<PathBuf, AppConfigError> {
    let home = dirs::home_dir().ok_or_else(|| AppConfigError {
        message: "ホームディレクトリの取得に失敗しました".to_string(),
    })?;
    Ok(home.join(".config/apptidying"))
}

#[allow(dead_code)]
pub fn parse_config_from_json(json_str: &str) -> Result<AppConfig, AppConfigError> {
    let config: AppConfig = serde_json::from_str(json_str).map_err(|e| AppConfigError {
        message: format!("JSON パースエラー: {}", e),
    })?;

    validate_config(&config)?;
    Ok(config)
}

#[allow(dead_code)]
pub fn load_config_file(path: &PathBuf) -> Result<AppConfig, AppConfigError> {
    let content = fs::read_to_string(path).map_err(|e| AppConfigError {
        message: format!("ファイル読み込みエラー ({}): {}", path.display(), e),
    })?;

    parse_config_from_json(&content)
}

#[allow(dead_code)]
pub fn load_default_config() -> Result<AppConfig, AppConfigError> {
    let config_dir = get_config_dir()?;
    let config_path = config_dir.join("settings.json");

    load_config_file(&config_path)
}

#[allow(dead_code)]
fn validate_config(config: &AppConfig) -> Result<(), AppConfigError> {
    // バージョンチェック
    if config.version != SUPPORTED_VERSION {
        return Err(AppConfigError {
            message: format!(
                "サポートされていないバージョン: {}（サポート: {}）",
                config.version, SUPPORTED_VERSION
            ),
        });
    }

    // レイアウトが空でないかチェック
    if config.layouts.is_empty() {
        return Err(AppConfigError {
            message: "layouts フィールドが空です".to_string(),
        });
    }

    // 各レイアウトのディスプレイをチェック
    for layout in &config.layouts {
        if layout.displays.is_empty() {
            return Err(AppConfigError {
                message: format!("レイアウト '{}' のディスプレイが空です", layout.name),
            });
        }

        // 各ディスプレイのウィンドウをチェック
        for display in &layout.displays {
            if display.windows.is_empty() {
                return Err(AppConfigError {
                    message: format!("ディスプレイ '{}' のウィンドウが空です", display.name),
                });
            }

            // ウィンドウの座標・サイズをチェック
            for window in &display.windows {
                validate_window_config(window, &display.name)?;
            }
        }
    }

    // 通知設定の検証
    if let Some(ref notification) = config.notification {
        validate_notification_config(notification)?;
    }

    Ok(())
}

#[allow(dead_code)]
fn validate_window_config(
    window: &AppWindowConfig,
    display_name: &str,
) -> Result<(), AppConfigError> {
    // 座標が指定されている場合のバリデーション
    if let Some(ref position) = window.position {
        validate_position(position).map_err(|e| AppConfigError {
            message: format!(
                "ディスプレイ '{}' のアプリ '{}' のウィンドウ設定でエラー: {}",
                display_name, window.app, e.message
            ),
        })?;
    }

    // サイズが指定されている場合のバリデーション
    if let Some(ref size) = window.size {
        validate_size(size).map_err(|e| AppConfigError {
            message: format!(
                "ディスプレイ '{}' のアプリ '{}' のウィンドウ設定でエラー: {}",
                display_name, window.app, e.message
            ),
        })?;
    }

    Ok(())
}

#[allow(dead_code)]
fn validate_value(
    value: &serde_json::Value,
    field_name: &str,
    allowed_strings: &[&str],
    min_numeric: i64,
) -> Result<(), AppConfigError> {
    match value {
        serde_json::Value::String(s) => {
            if !allowed_strings.contains(&s.as_str()) {
                return Err(AppConfigError {
                    message: format!(
                        "無効な {} 値: '{}' ({} を指定)",
                        field_name,
                        s,
                        allowed_strings.join(", ")
                    ),
                });
            }
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i < min_numeric {
                    return Err(AppConfigError {
                        message: if min_numeric == 0 {
                            format!("{} が負です", field_name)
                        } else {
                            format!("{} は正の数値である必要があります", field_name)
                        },
                    });
                }
            }
        }
        serde_json::Value::Null => {
            // null は許可
        }
        _ => {
            return Err(AppConfigError {
                message: format!("{} は文字列または数値である必要があります", field_name),
            });
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_position(position: &Position) -> Result<(), AppConfigError> {
    validate_value(&position.x, "x", &["left", "right"], 0)?;
    validate_value(&position.y, "y", &["top", "bottom"], 0)?;
    Ok(())
}

#[allow(dead_code)]
fn validate_size(size: &Size) -> Result<(), AppConfigError> {
    validate_value(&size.width, "width", &["half", "third", "max"], 1)?;
    validate_value(&size.height, "height", &["half", "third", "max"], 1)?;
    Ok(())
}

#[allow(dead_code)]
fn validate_notification_config(notification: &NotificationConfig) -> Result<(), AppConfigError> {
    let valid_values = ["notification", "dialog", "none"];

    if !valid_values.contains(&notification.info.as_str()) {
        return Err(AppConfigError {
            message: format!(
                "無効な notification.info 値: '{}' (notification, dialog または none を指定)",
                notification.info
            ),
        });
    }

    if !valid_values.contains(&notification.warn.as_str()) {
        return Err(AppConfigError {
            message: format!(
                "無効な notification.warn 値: '{}' (notification, dialog または none を指定)",
                notification.warn
            ),
        });
    }

    if !valid_values.contains(&notification.error.as_str()) {
        return Err(AppConfigError {
            message: format!(
                "無効な notification.error 値: '{}' (notification, dialog または none を指定)",
                notification.error
            ),
        });
    }

    Ok(())
}
