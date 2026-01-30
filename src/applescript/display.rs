//! ディスプレイ情報取得
//!
//! macOS の接続ディスプレイ情報を JXA で取得します。
//! 各ディスプレイの名前、解像度、原点座標を取得します。

use serde_json::{json, Value};

use crate::applescript::osascript::run_jxa;

/// ディスプレイ情報構造体
///
/// macOS のディスプレイの物理的な情報を保持します。
#[derive(Debug, Clone)]
pub struct DisplayInfo {
    /// ディスプレイ名（例: "Built-in", "Enhanced"）
    pub name: String,
    /// ディスプレイの幅（ピクセル）
    pub width: i32,
    /// ディスプレイの高さ（ピクセル）
    pub height: i32,
    /// ディスプレイの原点X座標
    pub origin_x: i32,
    /// ディスプレイの原点Y座標
    pub origin_y: i32,
}

impl DisplayInfo {
    /// JSON オブジェクトに変換
    ///
    /// 注意: このメソッドは将来のJSON出力機能用に残されています。
    #[allow(dead_code)] // 将来の拡張用に残す（JSON出力機能）
    pub fn to_json(&self) -> Value {
        json!({
            "name": self.name,
            "width": self.width,
            "height": self.height,
            "origin_x": self.origin_x,
            "origin_y": self.origin_y,
        })
    }

    /// JSON value から DisplayInfo を生成
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

/// ディスプレイ情報取得エラー
#[derive(Debug)]
pub struct DisplayError {
    pub message: String,
}

impl std::fmt::Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DisplayError {}

/// すべての接続ディスプレイ情報を取得
///
/// JXA を使用して macOS の NSScreen API からすべての接続ディスプレイ情報を取得します。
/// 各ディスプレイの名前、解像度（幅・高さ）、原点座標を取得します。
///
/// # Returns
/// * `Ok(Vec<DisplayInfo>)` - ディスプレイ情報のベクトル
/// * `Err(DisplayError)` - 失敗（ディスプレイなし、パース失敗など）
///
/// # Examples
/// ```ignore
/// use apptidying::applescript::get_all_connected_displays;
///
/// let displays = get_all_connected_displays()?;
/// for display in displays {
///     println!("{}: {}x{} at ({}, {})",
///              display.name, display.width, display.height,
///              display.origin_x, display.origin_y);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn get_all_connected_displays() -> Result<Vec<DisplayInfo>, DisplayError> {
    let jxa_script = r#"
ObjC.import('AppKit')

const screens = $.NSScreen.screens
let displays = []

if (screens.count === 0) {
    "error: ディスプレイが接続されていません"
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

    let output = run_jxa(jxa_script).map_err(|e| DisplayError { message: e.message })?;

    if !output.status.success() {
        return Err(DisplayError {
            message: format!(
                "ディスプレイ情報取得に失敗しました: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // エラー結果をチェック
    if result_str.starts_with("error:") {
        return Err(DisplayError {
            message: result_str,
        });
    }

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

/// 指定されたディスプレイ情報を取得
///
/// 指定されたディスプレイ名に一致するディスプレイを返します。
/// 指定されたディスプレイ名が見つからない場合はエラーを返します。
/// display_name が None の場合は、メインディスプレイ（最初のディスプレイ）を返します。
///
/// # Arguments
/// * `display_name` - 取得するディスプレイ名（オプション）
///
/// # Returns
/// * `Ok(DisplayInfo)` - ディスプレイ情報
/// * `Err(DisplayError)` - ディスプレイが見つからない等のエラー
///
/// # Examples
/// ```ignore
/// use apptidying::applescript::get_display_info;
///
/// // メインディスプレイを取得
/// let main_display = get_display_info(None)?;
/// println!("Main display: {}", main_display.name);
///
/// // 特定のディスプレイを取得
/// let display = get_display_info(Some("Built-in"))?;
/// println!("Display: {}x{}", display.width, display.height);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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
            } else {
                // ディスプレイ名が指定されたが見つからない場合はエラーを返す
                return Err(DisplayError {
                    message: format!("指定されたディスプレイ '{}' が見つかりません", name),
                });
            }
        }
    }

    // display_name が None または空の場合は、メインディスプレイ（最初のディスプレイ）を返す
    Ok(all_displays.into_iter().next().unwrap())
}
