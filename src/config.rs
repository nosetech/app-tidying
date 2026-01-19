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

/// デフォルト設定ファイルのパスを取得
/// macOS の標準に従い、~/Library/Application Support/biz.nosetech.apptidying/settings.json を返す
#[allow(dead_code)]
pub fn get_default_config_path() -> Result<PathBuf, AppConfigError> {
    let home = dirs::home_dir().ok_or_else(|| AppConfigError {
        message: "ホームディレクトリの取得に失敗しました".to_string(),
    })?;
    Ok(home.join("Library/Application Support/biz.nosetech.apptidying/settings.json"))
}

/// 設定ディレクトリを取得
/// デフォルト設定ファイルパスの親ディレクトリを返す
#[allow(dead_code)]
pub fn get_config_dir() -> Result<PathBuf, AppConfigError> {
    let default_path = get_default_config_path()?;
    default_path
        .parent()
        .map(|p| p.to_path_buf())
        .ok_or_else(|| AppConfigError {
            message: "設定ディレクトリの取得に失敗しました".to_string(),
        })
}

/// 設定ファイルの検証結果として、ディスプレイ外の座標やサイズが大きすぎる場合などの警告を格納する構造体
#[derive(Debug, Clone)]
pub struct ValidationWarning {
    /// ディスプレイ名
    pub display_name: String,
    /// アプリケーション名
    pub app_name: String,
    /// 警告メッセージ
    pub message: String,
}

#[allow(dead_code)]
pub fn parse_config_from_json(json_str: &str) -> Result<AppConfig, AppConfigError> {
    let config: AppConfig = serde_json::from_str(json_str).map_err(|e| AppConfigError {
        message: format!("JSON パースエラー: {}", e),
    })?;

    validate_config_syntax(&config)?;
    Ok(config)
}

#[allow(dead_code)]
pub fn load_config_file(path: &PathBuf) -> Result<AppConfig, AppConfigError> {
    let content = fs::read_to_string(path).map_err(|e| AppConfigError {
        message: format!("ファイル読み込みエラー ({}): {}", path.display(), e),
    })?;

    parse_config_from_json(&content)
}

/// デフォルト設定ファイルから設定を読み込む
#[allow(dead_code)]
pub fn load_default_config() -> Result<AppConfig, AppConfigError> {
    let config_path = get_default_config_path()?;
    load_config_file(&config_path)
}

/// 設定をファイルに保存する
#[allow(dead_code)]
pub fn save_config_file(config: &AppConfig, path: &PathBuf) -> Result<(), AppConfigError> {
    // 親ディレクトリが存在しない場合は作成
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| AppConfigError {
            message: format!(
                "ディレクトリの作成に失敗しました ({}): {}",
                parent.display(),
                e
            ),
        })?;
    }

    // JSONに変換（整形あり）
    let json_str = serde_json::to_string_pretty(config).map_err(|e| AppConfigError {
        message: format!("JSON シリアライズエラー: {}", e),
    })?;

    // ファイルに書き込み
    fs::write(path, json_str).map_err(|e| AppConfigError {
        message: format!("ファイル書き込みエラー ({}): {}", path.display(), e),
    })?;

    Ok(())
}

/// 設定ファイルの構文チェックのみを実行する
/// バージョンチェック、構造の妥当性、パターン値の検証などを行う
#[allow(dead_code)]
pub fn validate_config_syntax(config: &AppConfig) -> Result<(), AppConfigError> {
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
                message: "レイアウトのディスプレイが空です".to_string(),
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

/// ウィンドウの座標・サイズがディスプレイの境界内に収まっているかを検証
/// ディスプレイ外の座標や、画面より大きいサイズが設定されている場合は警告を返す
#[allow(dead_code)]
fn validate_display_bounds(
    window: &AppWindowConfig,
    display_info: &crate::applescript::DisplayInfo,
    display_name: &str,
) -> Option<ValidationWarning> {
    // 座標をピクセル単位で計算してチェック
    if let Some(ref position) = window.position {
        // サイズを計算（デフォルトはディスプレイサイズ）
        let window_width = if let Some(ref size) = window.size {
            match calculate_size_for_validation(&size.width, display_info.width) {
                Ok(w) => w,
                Err(_) => return None, // エラーは構文チェックで処理済み
            }
        } else {
            display_info.width
        };

        let window_height = if let Some(ref size) = window.size {
            match calculate_size_for_validation(&size.height, display_info.height) {
                Ok(h) => h,
                Err(_) => return None, // エラーは構文チェックで処理済み
            }
        } else {
            display_info.height
        };

        // 座標を計算
        let (x, y) = match calculate_position_for_validation(
            position,
            display_info.width,
            display_info.height,
            window_width,
            window_height,
        ) {
            Ok((x, y)) => (x, y),
            Err(_) => return None, // エラーは構文チェックで処理済み
        };

        // ウィンドウの右端がディスプレイを超えるかチェック
        if x + window_width > display_info.width {
            return Some(ValidationWarning {
                display_name: display_name.to_string(),
                app_name: window.app.clone(),
                message: format!(
                    "ウィンドウの右端 ({}) がディスプレイの幅 ({}) を超えています",
                    x + window_width,
                    display_info.width
                ),
            });
        }

        // ウィンドウの下端がディスプレイを超えるかチェック
        if y + window_height > display_info.height {
            return Some(ValidationWarning {
                display_name: display_name.to_string(),
                app_name: window.app.clone(),
                message: format!(
                    "ウィンドウの下端 ({}) がディスプレイの高さ ({}) を超えています",
                    y + window_height,
                    display_info.height
                ),
            });
        }
    }

    None
}

/// ディスプレイ名が実際に接続されているかを検証
#[allow(dead_code)]
fn validate_display_exists(
    display_name: &str,
    connected_displays: &[crate::applescript::DisplayInfo],
) -> bool {
    connected_displays
        .iter()
        .any(|display| display.name == display_name)
}

/// 境界値チェックを実行（ディスプレイ情報が必要）
/// 警告リストを返す（エラーではなく警告）
#[allow(dead_code)]
pub fn validate_config_bounds(
    config: &AppConfig,
    connected_displays: &[crate::applescript::DisplayInfo],
) -> Result<Vec<ValidationWarning>, AppConfigError> {
    let mut warnings = Vec::new();

    // 最初のレイアウトをチェック
    let layout = config.layouts.first().ok_or_else(|| AppConfigError {
        message: "レイアウトが定義されていません".to_string(),
    })?;

    for display_config in &layout.displays {
        // ディスプレイが接続されているかチェック
        if !validate_display_exists(&display_config.name, connected_displays) {
            for window in &display_config.windows {
                warnings.push(ValidationWarning {
                    display_name: display_config.name.clone(),
                    app_name: window.app.clone(),
                    message: format!(
                        "ディスプレイ '{}' が接続されていません",
                        display_config.name
                    ),
                });
            }
            continue;
        }

        // ディスプレイ情報を取得
        if let Some(display_info) = connected_displays
            .iter()
            .find(|d| d.name == display_config.name)
        {
            for window in &display_config.windows {
                if let Some(warning) =
                    validate_display_bounds(window, display_info, &display_config.name)
                {
                    warnings.push(warning);
                }
            }
        }
    }

    Ok(warnings)
}

/// 構文チェックと境界値チェックの両方を実行
/// connected_displays が指定されていない場合は構文チェックのみを実行
#[allow(dead_code)]
pub fn validate_config(
    config: &AppConfig,
    connected_displays: Option<&[crate::applescript::DisplayInfo]>,
) -> Result<Vec<ValidationWarning>, AppConfigError> {
    // 構文チェックを実行
    validate_config_syntax(config)?;

    // 境界値チェックを実行
    if let Some(displays) = connected_displays {
        validate_config_bounds(config, displays)
    } else {
        Ok(Vec::new())
    }
}

// =============================================================================
// Helper Functions for Validation
// =============================================================================

/// サイズ値をピクセル単位で計算（検証用）
fn calculate_size_for_validation(
    value: &serde_json::Value,
    display_size: i32,
) -> Result<i32, AppConfigError> {
    match value {
        serde_json::Value::String(s) => match s.as_str() {
            "half" => Ok(display_size / 2),
            "third" => Ok(display_size / 3),
            "max" => Ok(display_size),
            _ => Err(AppConfigError {
                message: format!("無効なサイズ値: '{}' (half, third, max を指定)", s),
            }),
        },
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i <= 0 {
                    Err(AppConfigError {
                        message: "サイズは正の値である必要があります".to_string(),
                    })
                } else {
                    Ok(i as i32)
                }
            } else {
                Err(AppConfigError {
                    message: "サイズは整数である必要があります".to_string(),
                })
            }
        }
        _ => Err(AppConfigError {
            message: "サイズは文字列または数値である必要があります".to_string(),
        }),
    }
}

/// 座標値をピクセル単位で計算（検証用）
fn calculate_position_for_validation(
    position: &Position,
    display_width: i32,
    display_height: i32,
    window_width: i32,
    window_height: i32,
) -> Result<(i32, i32), AppConfigError> {
    let x = calculate_x_for_validation(&position.x, display_width, window_width)?;
    let y = calculate_y_for_validation(&position.y, display_height, window_height)?;

    Ok((x, y))
}

/// X座標を計算（検証用）
fn calculate_x_for_validation(
    value: &serde_json::Value,
    display_width: i32,
    window_width: i32,
) -> Result<i32, AppConfigError> {
    match value {
        serde_json::Value::String(s) => match s.as_str() {
            "left" => Ok(0),
            "right" => Ok(display_width - window_width),
            _ => Err(AppConfigError {
                message: format!("無効な x 値: '{}'", s),
            }),
        },
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i < 0 {
                    Err(AppConfigError {
                        message: "x が負です".to_string(),
                    })
                } else {
                    Ok(i as i32)
                }
            } else {
                Err(AppConfigError {
                    message: "x は整数である必要があります".to_string(),
                })
            }
        }
        _ => Err(AppConfigError {
            message: "x は文字列または数値である必要があります".to_string(),
        }),
    }
}

/// Y座標を計算（検証用）
fn calculate_y_for_validation(
    value: &serde_json::Value,
    display_height: i32,
    window_height: i32,
) -> Result<i32, AppConfigError> {
    match value {
        serde_json::Value::String(s) => match s.as_str() {
            "top" => Ok(25), // メニューバーの高さは25pxと想定
            "bottom" => Ok(display_height - window_height),
            _ => Err(AppConfigError {
                message: format!("無効な y 値: '{}'", s),
            }),
        },
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i < 0 {
                    Err(AppConfigError {
                        message: "y が負です".to_string(),
                    })
                } else {
                    Ok(i as i32)
                }
            } else {
                Err(AppConfigError {
                    message: "y は整数である必要があります".to_string(),
                })
            }
        }
        _ => Err(AppConfigError {
            message: "y は文字列または数値である必要があります".to_string(),
        }),
    }
}

// =============================================================================
// Pattern Calculation Functions
// =============================================================================

/// Parse position value to absolute coordinates
/// Returns (x, y) coordinates
#[allow(dead_code, clippy::too_many_arguments)]
pub fn parse_position_value(
    value: &serde_json::Value,
    display_width: i32,
    display_height: i32,
    window_width: i32,
    window_height: i32,
    field_name: &str,
) -> Result<(i32, i32), AppConfigError> {
    match value {
        serde_json::Value::Object(obj) => {
            let x = obj.get("x").ok_or_else(|| AppConfigError {
                message: format!("{} に x フィールドが見つかりません", field_name),
            })?;

            let y = obj.get("y").ok_or_else(|| AppConfigError {
                message: format!("{} に y フィールドが見つかりません", field_name),
            })?;

            let x_val = parse_x_value(x, display_width, window_width)?;
            let y_val = parse_y_value(y, display_height, window_height)?;

            Ok((x_val, y_val))
        }
        _ => Err(AppConfigError {
            message: format!("{} はオブジェクトである必要があります", field_name),
        }),
    }
}

/// Parse x coordinate value
fn parse_x_value(
    value: &serde_json::Value,
    display_width: i32,
    window_width: i32,
) -> Result<i32, AppConfigError> {
    match value {
        serde_json::Value::String(s) => match s.as_str() {
            "left" => Ok(0),
            "right" => Ok(display_width - window_width),
            _ => Err(AppConfigError {
                message: format!("無効な x 値: '{}' (left, right を指定)", s),
            }),
        },
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i < 0 {
                    Err(AppConfigError {
                        message: "x が負です".to_string(),
                    })
                } else {
                    Ok(i as i32)
                }
            } else {
                Err(AppConfigError {
                    message: "x は整数である必要があります".to_string(),
                })
            }
        }
        _ => Err(AppConfigError {
            message: "x は文字列または数値である必要があります".to_string(),
        }),
    }
}

/// Parse y coordinate value
fn parse_y_value(
    value: &serde_json::Value,
    display_height: i32,
    window_height: i32,
) -> Result<i32, AppConfigError> {
    match value {
        serde_json::Value::String(s) => match s.as_str() {
            "top" => Ok(25), // メニューバーの高さは25pxと想定
            "bottom" => Ok(display_height - window_height),
            _ => Err(AppConfigError {
                message: format!("無効な y 値: '{}' (top, bottom を指定)", s),
            }),
        },
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i < 0 {
                    Err(AppConfigError {
                        message: "y が負です".to_string(),
                    })
                } else {
                    Ok(i as i32)
                }
            } else {
                Err(AppConfigError {
                    message: "y は整数である必要があります".to_string(),
                })
            }
        }
        _ => Err(AppConfigError {
            message: "y は文字列または数値である必要があります".to_string(),
        }),
    }
}

/// Parse size value to absolute dimensions
/// Returns (width, height) dimensions
#[allow(dead_code)]
pub fn parse_size_value(
    value: &serde_json::Value,
    display_width: i32,
    display_height: i32,
    field_name: &str,
) -> Result<(i32, i32), AppConfigError> {
    match value {
        serde_json::Value::Object(obj) => {
            let width = obj.get("width").ok_or_else(|| AppConfigError {
                message: format!("{} に width フィールドが見つかりません", field_name),
            })?;

            let height = obj.get("height").ok_or_else(|| AppConfigError {
                message: format!("{} に height フィールドが見つかりません", field_name),
            })?;

            let width_val = parse_width_value(width, display_width)?;
            let height_val = parse_height_value(height, display_height)?;

            if width_val <= 0 || height_val <= 0 {
                return Err(AppConfigError {
                    message: "ウィンドウサイズは正の値である必要があります".to_string(),
                });
            }

            Ok((width_val, height_val))
        }
        _ => Err(AppConfigError {
            message: format!("{} はオブジェクトである必要があります", field_name),
        }),
    }
}

/// Parse width value
fn parse_width_value(value: &serde_json::Value, display_width: i32) -> Result<i32, AppConfigError> {
    match value {
        serde_json::Value::String(s) => match s.as_str() {
            "half" => Ok(display_width / 2),
            "third" => Ok(display_width / 3),
            "max" => Ok(display_width),
            _ => Err(AppConfigError {
                message: format!("無効な width 値: '{}' (half, third, max を指定)", s),
            }),
        },
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i <= 0 {
                    Err(AppConfigError {
                        message: "width は正の値である必要があります".to_string(),
                    })
                } else {
                    Ok(i as i32)
                }
            } else {
                Err(AppConfigError {
                    message: "width は整数である必要があります".to_string(),
                })
            }
        }
        _ => Err(AppConfigError {
            message: "width は文字列または数値である必要があります".to_string(),
        }),
    }
}

/// Parse height value
fn parse_height_value(
    value: &serde_json::Value,
    display_height: i32,
) -> Result<i32, AppConfigError> {
    match value {
        serde_json::Value::String(s) => match s.as_str() {
            "half" => Ok(display_height / 2),
            "third" => Ok(display_height / 3),
            "max" => Ok(display_height),
            _ => Err(AppConfigError {
                message: format!("無効な height 値: '{}' (half, third, max を指定)", s),
            }),
        },
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i <= 0 {
                    Err(AppConfigError {
                        message: "height は正の値である必要があります".to_string(),
                    })
                } else {
                    Ok(i as i32)
                }
            } else {
                Err(AppConfigError {
                    message: "height は整数である必要があります".to_string(),
                })
            }
        }
        _ => Err(AppConfigError {
            message: "height は文字列または数値である必要があります".to_string(),
        }),
    }
}
