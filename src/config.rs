use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const SUPPORTED_VERSION: &str = "1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionValue {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalPositionValue {
    Top,
    Bottom,
}

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationConfig {
    #[serde(default = "default_notification_level")]
    pub info: String,
    #[serde(default = "default_notification_level")]
    pub warn: String,
    #[serde(default = "default_notification_level")]
    pub error: String,
}

fn default_notification_level() -> String {
    "notification".to_string()
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

pub fn get_config_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Failed to get home directory");
    home.join(".config/apptidying")
}

pub fn parse_config_from_json(json_str: &str) -> Result<AppConfig, AppConfigError> {
    let config: AppConfig = serde_json::from_str(json_str)
        .map_err(|e| AppConfigError {
            message: format!("JSON パースエラー: {}", e),
        })?;

    validate_config(&config)?;
    Ok(config)
}

pub fn load_config_file(path: &PathBuf) -> Result<AppConfig, AppConfigError> {
    let content = fs::read_to_string(path).map_err(|e| AppConfigError {
        message: format!("ファイル読み込みエラー ({}): {}", path.display(), e),
    })?;

    parse_config_from_json(&content)
}

pub fn load_default_config() -> Result<AppConfig, AppConfigError> {
    let config_dir = get_config_dir();
    let config_path = config_dir.join("settings.json");

    load_config_file(&config_path)
}

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
                message: format!(
                    "レイアウト '{}' のディスプレイが空です",
                    layout.name
                ),
            });
        }

        // 各ディスプレイのウィンドウをチェック
        for display in &layout.displays {
            if display.windows.is_empty() {
                return Err(AppConfigError {
                    message: format!(
                        "ディスプレイ '{}' のウィンドウが空です",
                        display.name
                    ),
                });
            }

            // ウィンドウの座標・サイズをチェック
            for window in &display.windows {
                validate_window_config(window)?;
            }
        }
    }

    // 通知設定の検証
    if let Some(ref notification) = config.notification {
        validate_notification_config(notification)?;
    }

    Ok(())
}

fn validate_window_config(window: &AppWindowConfig) -> Result<(), AppConfigError> {
    // 座標が指定されている場合のバリデーション
    if let Some(ref position) = window.position {
        validate_position(position)?;
    }

    // サイズが指定されている場合のバリデーション
    if let Some(ref size) = window.size {
        validate_size(size)?;
    }

    Ok(())
}

fn validate_position(position: &Position) -> Result<(), AppConfigError> {
    // x 値のチェック
    match &position.x {
        serde_json::Value::String(s) => {
            if !["left", "right"].contains(&s.as_str()) {
                return Err(AppConfigError {
                    message: format!("無効な x 値: '{}' (left または right を指定)", s),
                });
            }
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i < 0 {
                    return Err(AppConfigError {
                        message: "x 座標が負です".to_string(),
                    });
                }
            }
        }
        serde_json::Value::Null => {
            // null は許可
        }
        _ => {
            return Err(AppConfigError {
                message: "x は文字列または数値である必要があります".to_string(),
            });
        }
    }

    // y 値のチェック
    match &position.y {
        serde_json::Value::String(s) => {
            if !["top", "bottom"].contains(&s.as_str()) {
                return Err(AppConfigError {
                    message: format!("無効な y 値: '{}' (top または bottom を指定)", s),
                });
            }
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i < 0 {
                    return Err(AppConfigError {
                        message: "y 座標が負です".to_string(),
                    });
                }
            }
        }
        serde_json::Value::Null => {
            // null は許可
        }
        _ => {
            return Err(AppConfigError {
                message: "y は文字列または数値である必要があります".to_string(),
            });
        }
    }

    Ok(())
}

fn validate_size(size: &Size) -> Result<(), AppConfigError> {
    // width のチェック
    match &size.width {
        serde_json::Value::String(s) => {
            if !["half", "third", "max"].contains(&s.as_str()) {
                return Err(AppConfigError {
                    message: format!(
                        "無効な width 値: '{}' (half, third または max を指定)",
                        s
                    ),
                });
            }
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i <= 0 {
                    return Err(AppConfigError {
                        message: "width は正の数値である必要があります".to_string(),
                    });
                }
            }
        }
        serde_json::Value::Null => {
            // null は許可
        }
        _ => {
            return Err(AppConfigError {
                message: "width は文字列または数値である必要があります".to_string(),
            });
        }
    }

    // height のチェック
    match &size.height {
        serde_json::Value::String(s) => {
            if !["half", "third", "max"].contains(&s.as_str()) {
                return Err(AppConfigError {
                    message: format!(
                        "無効な height 値: '{}' (half, third または max を指定)",
                        s
                    ),
                });
            }
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i <= 0 {
                    return Err(AppConfigError {
                        message: "height は正の数値である必要があります".to_string(),
                    });
                }
            }
        }
        serde_json::Value::Null => {
            // null は許可
        }
        _ => {
            return Err(AppConfigError {
                message: "height は文字列または数値である必要があります".to_string(),
            });
        }
    }

    Ok(())
}

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
