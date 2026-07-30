use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// =============================================================================
// 定数定義
// =============================================================================

/// macOS メニューバーの標準高さ（ピクセル）
const MACOS_MENU_BAR_HEIGHT: i32 = 25;

/// ウィンドウの位置情報
///
/// X座標（x）とY座標（y）を指定します。
/// 値は以下のいずれかの形式で指定できます：
/// - 文字列: `"left"`, `"right"`, `"top"`, `"bottom"`
/// - 数値: ピクセル単位の絶対座標
/// - `null`（またはキー自体を省略）: 未指定。`x`/`y` を片方だけ指定した場合、
///   未指定側は変更されず現在のウィンドウ位置が維持される（Issue #120。
///   [`fill_absent_position`] を参照）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    /// X座標値（`"left"`, `"right"`、0以上の数値、または未指定を表す `null`）
    #[serde(default)]
    pub x: serde_json::Value,
    /// Y座標値（`"top"`, `"bottom"`、0以上の数値、または未指定を表す `null`）
    #[serde(default)]
    pub y: serde_json::Value,
}

/// ウィンドウのサイズ情報
///
/// 幅（width）と高さ（height）を指定します。
/// 値は以下のいずれかの形式で指定できます：
/// - 文字列: `"half"`（1/2）, `"third"`（1/3）, `"max"`（フル）
/// - 数値: ピクセル単位のサイズ
/// - `null`（またはキー自体を省略）: 未指定。`width`/`height` を片方だけ指定した場合、
///   未指定側は変更されず現在のウィンドウサイズが維持される（Issue #120。
///   [`fill_absent_size`] を参照）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Size {
    /// 幅（`"half"`, `"third"`, `"max"`、1以上の数値、または未指定を表す `null`）
    #[serde(default)]
    pub width: serde_json::Value,
    /// 高さ（`"half"`, `"third"`, `"max"`、1以上の数値、または未指定を表す `null`）
    #[serde(default)]
    pub height: serde_json::Value,
}

/// アプリケーションウィンドウの設定
///
/// layout.json の各ウィンドウ設定を表す構造体です。
/// アプリケーション名、位置、サイズを指定します。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppWindowConfig {
    /// アプリケーション名（macOS での表示名、例: `"Google Chrome"`, `"Terminal"`）
    pub app: String,
    /// ウィンドウの位置（左上座標）。オプション
    #[serde(default)]
    pub position: Option<Position>,
    /// ウィンドウのサイズ（幅と高さ）。オプション
    #[serde(default)]
    pub size: Option<Size>,
    /// OS標準タイリング指定（layout.json version 2.0、Issue #121/#122）
    ///
    /// `"left"`, `"right"`, `"top"`, `"bottom"`, `"top-left"`, `"top-right"`,
    /// `"bottom-left"`, `"bottom-right"`, `"full-screen"` のいずれかを指定します。
    /// `position`/`size` とは相互排他（同時指定はバリデーションエラー）。
    /// version 1.0 の layout.json では指定できません。詳細は
    /// [`parse_tile_keyword`] を参照
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tiling: Option<String>,
}

/// ディスプレイの設定
///
/// ディスプレイごとのウィンドウ設定をまとめた構造体です。
/// ディスプレイ名と、そこに配置するウィンドウのリストを指定します。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DisplayConfig {
    /// ディスプレイ名（例: `"Built-in"`, `"Enhanced"`, `"External Display"`）
    pub name: String,
    /// このディスプレイに配置するウィンドウの設定リスト
    pub windows: Vec<AppWindowConfig>,
}

/// ウィンドウレイアウト設定
///
/// 複数のディスプレイにおけるウィンドウ配置をまとめた構造体です。
/// 各ディスプレイの設定をリストで保持します。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LayoutConfig {
    /// ディスプレイの設定リスト
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

/// 通知設定
///
/// アプリケーション実行時の通知方式をレベル別に指定します。
/// settings.json の `notification` フィールドで設定します。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationConfig {
    /// INFO レベル通知方式（`"notification"`, `"dialog"`, `"none"`）
    #[serde(default = "default_notification_info")]
    pub info: String,
    /// WARN レベル通知方式（`"notification"`, `"dialog"`, `"none"`）
    #[serde(default = "default_notification_warn")]
    pub warn: String,
    /// ERROR レベル通知方式（`"notification"`, `"dialog"`, `"none"`）
    #[serde(default = "default_notification_error")]
    pub error: String,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        NotificationConfig {
            info: default_notification_info(),
            warn: default_notification_warn(),
            error: default_notification_error(),
        }
    }
}

fn default_rotation_type() -> String {
    "size".to_string()
}

fn default_max_size_mb() -> u64 {
    10
}

fn default_max_files() -> u32 {
    5
}

/// ログローテーション設定構造体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogRotationConfig {
    /// ローテーション方式（現在は "size" のみサポート）
    #[serde(default = "default_rotation_type")]
    pub rotation_type: String,

    /// 最大ファイルサイズ（MB単位）
    #[serde(default = "default_max_size_mb")]
    pub max_size_mb: u64,

    /// 保持する世代数
    #[serde(default = "default_max_files")]
    pub max_files: u32,
}

impl Default for LogRotationConfig {
    fn default() -> Self {
        LogRotationConfig {
            rotation_type: default_rotation_type(),
            max_size_mb: default_max_size_mb(),
            max_files: default_max_files(),
        }
    }
}

/// settings.json 用構造体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub version: String,
    #[serde(default)]
    pub notification: Option<NotificationConfig>,
    #[serde(default)]
    pub timeout: Option<u64>,
    #[serde(default)]
    pub log_rotation: Option<LogRotationConfig>,
}

/// layout.json 用構造体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LayoutFile {
    pub version: String,
    pub layouts: Vec<LayoutConfig>,
}

/// 設定ファイル処理のエラー型
///
/// JSON パース、ファイルIO、バリデーションエラーなど、
/// 設定ファイル処理で発生するすべてのエラーを表します。
#[derive(Debug)]
pub struct AppConfigError {
    /// エラーメッセージ
    pub message: String,
}

impl std::fmt::Display for AppConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppConfigError {}

/// デフォルト設定ファイル (settings.json) のパスを取得
/// macOS の標準に従い、~/Library/Application Support/biz.nosetech.apptidying/settings.json を返す
pub fn get_default_settings_path() -> Result<PathBuf, AppConfigError> {
    let home = dirs::home_dir().ok_or_else(|| AppConfigError {
        message: "ホームディレクトリの取得に失敗しました".to_string(),
    })?;
    Ok(home.join("Library/Application Support/biz.nosetech.apptidying/settings.json"))
}

/// デフォルトレイアウトファイル (layout.json) のパスを取得
/// macOS の標準に従い、~/Library/Application Support/biz.nosetech.apptidying/layout.json を返す
pub fn get_default_layout_path() -> Result<PathBuf, AppConfigError> {
    let home = dirs::home_dir().ok_or_else(|| AppConfigError {
        message: "ホームディレクトリの取得に失敗しました".to_string(),
    })?;
    Ok(home.join("Library/Application Support/biz.nosetech.apptidying/layout.json"))
}

/// 設定ディレクトリを取得
/// デフォルト設定ファイルパスの親ディレクトリを返す
///
/// 注意: この関数は将来の設定ファイル管理機能用に残されています。
#[allow(dead_code)] // 将来の拡張用に残す（設定ファイル管理機能）
pub fn get_config_dir() -> Result<PathBuf, AppConfigError> {
    let default_path = get_default_settings_path()?;
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

/// settings.json をパースする
pub fn parse_settings_from_json(json_str: &str) -> Result<AppSettings, AppConfigError> {
    let settings: AppSettings = serde_json::from_str(json_str).map_err(|e| AppConfigError {
        message: format!("JSON パースエラー: {}", e),
    })?;

    validate_settings_syntax(&settings)?;
    Ok(settings)
}

/// layout.json をパースする
pub fn parse_layout_from_json(json_str: &str) -> Result<LayoutFile, AppConfigError> {
    let layout: LayoutFile = serde_json::from_str(json_str).map_err(|e| AppConfigError {
        message: format!("JSON パースエラー: {}", e),
    })?;

    validate_layout_syntax(&layout)?;
    Ok(layout)
}

/// settings.json ファイルを読み込む
pub fn load_settings_file(path: &PathBuf) -> Result<AppSettings, AppConfigError> {
    let content = fs::read_to_string(path).map_err(|e| AppConfigError {
        message: format!("ファイル読み込みエラー ({}): {}", path.display(), e),
    })?;

    parse_settings_from_json(&content)
}

/// layout.json ファイルを読み込む
pub fn load_layout_file(path: &PathBuf) -> Result<LayoutFile, AppConfigError> {
    let content = fs::read_to_string(path).map_err(|e| AppConfigError {
        message: format!("ファイル読み込みエラー ({}): {}", path.display(), e),
    })?;

    parse_layout_from_json(&content)
}

/// デフォルト settings.json から設定を読み込む
pub fn load_default_settings() -> Result<AppSettings, AppConfigError> {
    let config_path = get_default_settings_path()?;
    load_settings_file(&config_path)
}

/// デフォルト layout.json からレイアウトを読み込む
pub fn load_default_layout() -> Result<LayoutFile, AppConfigError> {
    let layout_path = get_default_layout_path()?;
    load_layout_file(&layout_path)
}

/// settings.json をファイルに保存する
///
/// 注意: この関数は将来の settings 編集機能用に残されています。
#[allow(dead_code)] // 将来の拡張用に残す（settings 編集機能）
pub fn save_settings_file(settings: &AppSettings, path: &PathBuf) -> Result<(), AppConfigError> {
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
    let json_str = serde_json::to_string_pretty(settings).map_err(|e| AppConfigError {
        message: format!("JSON シリアライズエラー: {}", e),
    })?;

    // ファイルに書き込み
    fs::write(path, json_str).map_err(|e| AppConfigError {
        message: format!("ファイル書き込みエラー ({}): {}", path.display(), e),
    })?;

    Ok(())
}

/// layout.json をファイルに保存する
pub fn save_layout_file(layout: &LayoutFile, path: &PathBuf) -> Result<(), AppConfigError> {
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
    let json_str = serde_json::to_string_pretty(layout).map_err(|e| AppConfigError {
        message: format!("JSON シリアライズエラー: {}", e),
    })?;

    // ファイルに書き込み
    fs::write(path, json_str).map_err(|e| AppConfigError {
        message: format!("ファイル書き込みエラー ({}): {}", path.display(), e),
    })?;

    Ok(())
}

/// settings.json の構文チェックを実行する
pub fn validate_settings_syntax(settings: &AppSettings) -> Result<(), AppConfigError> {
    // バージョンチェック
    if settings.version != "1.0" {
        return Err(AppConfigError {
            message: format!(
                "サポートされていないバージョン: {}（サポート: 1.0）",
                settings.version
            ),
        });
    }

    // 通知設定の検証
    if let Some(ref notification) = settings.notification {
        validate_notification_config(notification)?;
    }

    // ログローテーション設定の検証
    if let Some(ref log_rotation) = settings.log_rotation {
        validate_log_rotation_config(log_rotation)?;
    }

    Ok(())
}

/// layout.json の構文チェックを実行する
pub fn validate_layout_syntax(layout: &LayoutFile) -> Result<(), AppConfigError> {
    // バージョンチェック（Issue #121/#122: version 2.0 で tiling フィールドを追加サポート）
    if layout.version != "1.0" && layout.version != "2.0" {
        return Err(AppConfigError {
            message: format!(
                "サポートされていないバージョン: {}（サポート: 1.0, 2.0）",
                layout.version
            ),
        });
    }

    // レイアウトが空でないかチェック
    if layout.layouts.is_empty() {
        return Err(AppConfigError {
            message: "layouts フィールドが空です".to_string(),
        });
    }

    // 各レイアウトのディスプレイをチェック
    for layout_config in &layout.layouts {
        if layout_config.displays.is_empty() {
            return Err(AppConfigError {
                message: "レイアウトのディスプレイが空です".to_string(),
            });
        }

        // 各ディスプレイのウィンドウをチェック
        for display in &layout_config.displays {
            if display.windows.is_empty() {
                return Err(AppConfigError {
                    message: format!("ディスプレイ '{}' のウィンドウが空です", display.name),
                });
            }

            // ウィンドウの座標・サイズをチェック
            for window in &display.windows {
                validate_window_config(window, &display.name, &layout.version)?;
            }
        }
    }

    Ok(())
}

fn validate_window_config(
    window: &AppWindowConfig,
    display_name: &str,
    version: &str,
) -> Result<(), AppConfigError> {
    // tiling と position/size の相互排他チェック（Issue #121で決定。
    // 「指定」はJSON上のキー存在で判定するため、position/size の中身が
    // null（未指定）であってもキー自体が存在すれば「指定あり」として扱う）
    if window.tiling.is_some() && (window.position.is_some() || window.size.is_some()) {
        return Err(AppConfigError {
            message: format!(
                "ディスプレイ '{}' のアプリ '{}' のウィンドウ設定でエラー: 'tiling' と 'position'/'size' を同時に指定することはできません",
                display_name, window.app
            ),
        });
    }

    // tiling が指定されている場合のバリデーション
    if let Some(ref tiling) = window.tiling {
        // version 1.0 では tiling フィールドは非対応（Issue #121で決定）
        if version == "1.0" {
            return Err(AppConfigError {
                message: format!(
                    "ディスプレイ '{}' のアプリ '{}' のウィンドウ設定でエラー: version 1.0 では 'tiling' \
                     フィールドはサポートされていません（version 2.0 を指定してください）",
                    display_name, window.app
                ),
            });
        }

        parse_tile_keyword(tiling).map_err(|e| AppConfigError {
            message: format!(
                "ディスプレイ '{}' のアプリ '{}' のウィンドウ設定でエラー: {}",
                display_name, window.app, e.message
            ),
        })?;
    }

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
            // null は許可（Issue #120: x/y または width/height を片方だけ指定した場合、
            // 未指定側は `fill_absent_position`/`fill_absent_size` により load 実行時に
            // 現在のウィンドウ位置・サイズで補完されるため、ここではエラーとしない）
        }
        _ => {
            return Err(AppConfigError {
                message: format!("{} は文字列または数値である必要があります", field_name),
            });
        }
    }
    Ok(())
}

fn validate_position(position: &Position) -> Result<(), AppConfigError> {
    validate_value(&position.x, "x", &["left", "right"], 0)?;
    validate_value(&position.y, "y", &["top", "bottom"], 0)?;
    Ok(())
}

fn validate_size(size: &Size) -> Result<(), AppConfigError> {
    validate_value(&size.width, "width", &["half", "third", "max"], 1)?;
    validate_value(&size.height, "height", &["half", "third", "max"], 1)?;
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

/// ログローテーション設定を検証する
fn validate_log_rotation_config(log_rotation: &LogRotationConfig) -> Result<(), AppConfigError> {
    // rotation_type の検証
    if log_rotation.rotation_type != "size" {
        return Err(AppConfigError {
            message: format!(
                "無効な log_rotation.rotation_type 値: '{}' (size のみサポート)",
                log_rotation.rotation_type
            ),
        });
    }

    // max_size_mb の検証（1以上）
    if log_rotation.max_size_mb < 1 {
        return Err(AppConfigError {
            message: "log_rotation.max_size_mb は1以上の値である必要があります".to_string(),
        });
    }

    // max_files の検証（1以上）
    if log_rotation.max_files < 1 {
        return Err(AppConfigError {
            message: "log_rotation.max_files は1以上の値である必要があります".to_string(),
        });
    }

    Ok(())
}

/// ウィンドウの座標・サイズがディスプレイの境界内に収まっているかを検証
/// ディスプレイ外の座標や、画面より大きいサイズが設定されている場合は警告を返す
///
/// # 個別フィールド部分指定・size省略（Issue #120）とのチェック対象外について
///
/// `position`/`size` の個別フィールド（`x`/`y`/`width`/`height`）が `null`（部分指定で
/// 未指定）の場合、この時点では未指定側の値（現在のウィンドウ位置・サイズ）が確定して
/// いないため、`calculate_size_for_validation`/`calculate_position_for_validation` は
/// `Err` を返す。この検証は `load_layout` の実行直後（`process_window` で現在値を
/// 取得する前）に行われるため、未指定側の実際の値を用いた境界チェックはできず、
/// 意図的にスキップしている（`null` 自体は `validate_value` で許容される正当な値であり、
/// 構文エラーではない）。
///
/// `size` フィールド自体が丸ごと省略されている場合（`window.size == None`）も同様に
/// 境界チェックをスキップする。以前は `size` 省略時にウィンドウ高さをディスプレイの
/// 全高と仮定していたが、この仮定は `position.y == "top"` （メニューバー分オフセット
/// する `MACOS_MENU_BAR_HEIGHT` を加算する一方、高さは全高のまま）と組み合わさると
/// 必ず「下端がディスプレイの高さを超える」という誤検知を起こしていた
/// （実際の配置処理 `process_window`／`parse_position_value` では現在のウィンドウ
/// サイズを維持したまま `y` のみ変更するため、実際には問題は起きない）。
/// `size` 省略時も現在のウィンドウサイズは未確定であるため、個別フィールド `null` の
/// 場合と同様にチェック対象外とするのが一貫した扱いである。
fn validate_display_bounds(
    window: &AppWindowConfig,
    display_info: &crate::applescript::DisplayInfo,
    display_name: &str,
) -> Option<ValidationWarning> {
    // 座標をピクセル単位で計算してチェック
    if let Some(ref position) = window.position {
        // size が丸ごと省略されている場合、現在のウィンドウサイズが未確定であるため
        // 境界チェック対象外とする（関数冒頭のドキュメントコメント参照）
        let size = window.size.as_ref()?;

        let window_width = match calculate_size_for_validation(&size.width, display_info.width) {
            Ok(w) => w,
            // width が null（個別フィールド部分指定）等、現在値が未確定で計算できない場合は
            // 境界チェック対象外とする（関数冒頭のドキュメントコメント参照）
            Err(_) => return None,
        };

        let window_height = match calculate_size_for_validation(&size.height, display_info.height) {
            Ok(h) => h,
            Err(_) => return None,
        };

        // 座標を計算
        let (x, y) = match calculate_position_for_validation(
            position,
            Some(size),
            display_info.width,
            display_info.height,
            window_width,
            window_height,
        ) {
            Ok((x, y)) => (x, y),
            Err(_) => return None,
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
pub fn validate_layout_bounds(
    layout: &LayoutFile,
    connected_displays: &[crate::applescript::DisplayInfo],
) -> Result<Vec<ValidationWarning>, AppConfigError> {
    let mut warnings = Vec::new();

    // 最初のレイアウトをチェック
    let layout_config = layout.layouts.first().ok_or_else(|| AppConfigError {
        message: "レイアウトが定義されていません".to_string(),
    })?;

    for display_config in &layout_config.displays {
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

/// layout.json の構文チェックと境界値チェックの両方を実行
/// connected_displays が指定されていない場合は構文チェックのみを実行
pub fn validate_layout(
    layout: &LayoutFile,
    connected_displays: Option<&[crate::applescript::DisplayInfo]>,
) -> Result<Vec<ValidationWarning>, AppConfigError> {
    // 構文チェックを実行
    validate_layout_syntax(layout)?;

    // 境界値チェックを実行
    if let Some(displays) = connected_displays {
        validate_layout_bounds(layout, displays)
    } else {
        Ok(Vec::new())
    }
}

// =============================================================================
// 検証用ヘルパー関数
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

/// JSON値のサイズ情報から width/height が "max" かどうかを判定する
///
/// # Arguments
/// * `size_value` - 判定対象のサイズ情報（JSON値）
///
/// # Returns
/// * `(bool, bool)` - (width_is_max, height_is_max)
fn is_size_max_from_json(size_value: Option<&serde_json::Value>) -> (bool, bool) {
    if let Some(size) = size_value {
        if let Some(obj) = size.as_object() {
            let width_is_max = obj
                .get("width")
                .and_then(|v| v.as_str())
                .map(|s| s == "max")
                .unwrap_or(false);
            let height_is_max = obj
                .get("height")
                .and_then(|v| v.as_str())
                .map(|s| s == "max")
                .unwrap_or(false);
            (width_is_max, height_is_max)
        } else {
            (false, false)
        }
    } else {
        // サイズ情報が指定されていない場合は、max 判定を行わない
        (false, false)
    }
}

/// Size 構造体のサイズ情報から width/height が "max" かどうかを判定する
///
/// # Arguments
/// * `size` - 判定対象のサイズ情報（Size 構造体）
///
/// # Returns
/// * `(bool, bool)` - (width_is_max, height_is_max)
fn is_size_max(size: Option<&Size>) -> (bool, bool) {
    if let Some(size) = size {
        let width_is_max = if let serde_json::Value::String(ref s) = size.width {
            s == "max"
        } else {
            false
        };
        let height_is_max = if let serde_json::Value::String(ref s) = size.height {
            s == "max"
        } else {
            false
        };
        (width_is_max, height_is_max)
    } else {
        // サイズ情報が指定されていない場合は、max 判定を行わない
        (false, false)
    }
}

/// 座標値をピクセル単位で計算（検証用）
#[allow(clippy::too_many_arguments)]
fn calculate_position_for_validation(
    position: &Position,
    size: Option<&Size>,
    display_width: i32,
    display_height: i32,
    window_width: i32,
    window_height: i32,
) -> Result<(i32, i32), AppConfigError> {
    // サイズ情報から width/height が "max" かどうかを判定
    let (width_is_max, height_is_max) = is_size_max(size);

    // width が "max" の場合、X座標を 0 に設定
    let x = if width_is_max {
        0
    } else {
        calculate_x_for_validation(&position.x, display_width, window_width)?
    };

    // height が "max" の場合、Y座標を 0 に設定
    let y = if height_is_max {
        0
    } else {
        calculate_y_for_validation(&position.y, display_height, window_height)?
    };

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
            "top" => Ok(MACOS_MENU_BAR_HEIGHT), // メニューバーの高さ
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
// パターン計算関数
// =============================================================================

/// 位置値を絶対座標に変換
/// (x, y) 座標を返す
#[allow(clippy::too_many_arguments)]
pub fn parse_position_value(
    value: &serde_json::Value,
    size_value: Option<&serde_json::Value>,
    display_width: i32,
    display_height: i32,
    window_width: i32,
    window_height: i32,
    field_name: &str,
) -> Result<(i32, i32), AppConfigError> {
    // サイズ情報から width/height が "max" かどうかを判定
    let (width_is_max, height_is_max) = is_size_max_from_json(size_value);

    match value {
        serde_json::Value::Object(obj) => {
            let x = obj.get("x").ok_or_else(|| AppConfigError {
                message: format!("{} に x フィールドが見つかりません", field_name),
            })?;

            let y = obj.get("y").ok_or_else(|| AppConfigError {
                message: format!("{} に y フィールドが見つかりません", field_name),
            })?;

            // width が "max" の場合、X座標を 0 に設定
            let x_val = if width_is_max {
                0
            } else {
                parse_x_value(x, display_width, window_width)?
            };

            // height が "max" の場合、Y座標を 0 に設定
            let y_val = if height_is_max {
                0
            } else {
                parse_y_value(y, display_height, window_height)?
            };

            Ok((x_val, y_val))
        }
        _ => Err(AppConfigError {
            message: format!("{} はオブジェクトである必要があります", field_name),
        }),
    }
}

/// X座標値を変換
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

/// Y座標値を変換
fn parse_y_value(
    value: &serde_json::Value,
    display_height: i32,
    window_height: i32,
) -> Result<i32, AppConfigError> {
    match value {
        serde_json::Value::String(s) => match s.as_str() {
            "top" => Ok(MACOS_MENU_BAR_HEIGHT), // メニューバーの高さ
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

/// サイズ値を絶対サイズに変換
/// (width, height) サイズを返す
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

            // height が "max" の場合、メニューバー高さを考慮して計算
            let height_val = match height {
                serde_json::Value::String(s) if s == "max" => {
                    display_height - MACOS_MENU_BAR_HEIGHT
                }
                _ => parse_height_value(height, display_height)?,
            };

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

/// 幅値を変換
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

/// 高さ値を変換
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

// =============================================================================
// OS標準タイリング判定（Issue #118）
// =============================================================================

/// macOS標準の「ウインドウ」メニューで実現可能な配置パターン
///
/// `System Events` の `position`/`size` プロパティを直接設定する代わりに、この列挙値に
/// 対応するメニュー項目（「移動とサイズ変更」サブメニュー、または「画面全体に表示」）を
/// クリックして配置する（`src/applescript/window_menu.rs` の
/// [`tile_window_via_menu`](crate::applescript::tile_window_via_menu) が使用する）。
///
/// `Left`/`Right`/`Top`/`Bottom`/`TopLeft`/`TopRight`/`BottomLeft`/`BottomRight` は
/// 「ウインドウ」メニュー配下の「移動とサイズ変更」サブメニューの項目に対応し、
/// `FullScreen` のみ「ウインドウ」メニュー直下の項目（サブメニューを経由しない）に対応する。
///
/// **注意**: `position`/`size` のパターン指定から自動推測する仕組み（旧
/// `resolve_tile_keyword()`、Issue #118）は Issue #123 の設計変更に伴い削除された。
/// 現在この enum は、layout.json version 2.0 の明示的な `tiling` フィールド
/// （[`AppWindowConfig::tiling`]、Issue #121で設計・Issue #122で実装）から
/// [`parse_tile_keyword`] を経由して利用される。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileKeyword {
    /// 左半分（「移動とサイズ変更」＞「左」）
    Left,
    /// 右半分（「移動とサイズ変更」＞「右」）
    Right,
    /// 上半分（「移動とサイズ変更」＞「上」）
    Top,
    /// 下半分（「移動とサイズ変更」＞「下」）
    Bottom,
    /// 左上4分割（「移動とサイズ変更」＞「左上」）
    TopLeft,
    /// 右上4分割（「移動とサイズ変更」＞「右上」）
    TopRight,
    /// 左下4分割（「移動とサイズ変更」＞「左下」）
    BottomLeft,
    /// 右下4分割（「移動とサイズ変更」＞「右下」）
    BottomRight,
    /// 画面全体に表示（macOS標準のフルスクリーン機能。「移動とサイズ変更」の配下ではない）
    FullScreen,
}

impl TileKeyword {
    /// 対応するAppleScriptメニュー項目名（日本語表記）を返す
    ///
    /// ログ出力用。実際のメニュー項目探索では、表記揺れに対応するため
    /// [`menu_item_name_candidates`](Self::menu_item_name_candidates) が返す
    /// 複数候補を使用する。
    pub fn menu_item_name(&self) -> &'static str {
        self.menu_item_name_candidates()[0]
    }

    /// メニュー項目名の候補一覧（日本語表記・英語表記）を返す
    ///
    /// macOSの言語設定によりメニュー項目の表記が異なるため、AppleScript側の
    /// 検索ではこの候補を順に試す。日本語表記を先頭に置く（`menu_item_name()` が
    /// 先頭要素を返す前提のため）。英語表記はmacOS標準の表記を想定しているが、
    /// 実機での検証は日本語環境でのみ行っている（Issue #118）。
    pub fn menu_item_name_candidates(&self) -> &'static [&'static str] {
        match self {
            TileKeyword::Left => &["左", "Left"],
            TileKeyword::Right => &["右", "Right"],
            TileKeyword::Top => &["上", "Top"],
            TileKeyword::Bottom => &["下", "Bottom"],
            TileKeyword::TopLeft => &["左上", "Top Left"],
            TileKeyword::TopRight => &["右上", "Top Right"],
            TileKeyword::BottomLeft => &["左下", "Bottom Left"],
            TileKeyword::BottomRight => &["右下", "Bottom Right"],
            TileKeyword::FullScreen => &["画面全体に表示", "Enter Full Screen"],
        }
    }

    /// 「移動とサイズ変更」サブメニュー配下の項目かどうか
    ///
    /// `false` の場合（`FullScreen`）は「ウインドウ」メニュー直下の項目として扱う。
    pub fn is_submenu_item(&self) -> bool {
        !matches!(self, TileKeyword::FullScreen)
    }
}

/// layout.json の `tiling` フィールドで指定可能なキーワード一覧（Issue #121で決定）
///
/// 英語・小文字・ケバブケース表記で固定し、エイリアスや大文字表記は許容しない。
/// `position`/`size` のパターン指定（`left`/`right`/`half`/`third`/`max` 等）との
/// 表記一貫性を優先している。
const TILE_KEYWORD_VALUES: &[&str] = &[
    "left",
    "right",
    "top",
    "bottom",
    "top-left",
    "top-right",
    "bottom-left",
    "bottom-right",
    "full-screen",
];

/// `tiling` フィールドの文字列値を [`TileKeyword`] にパースする
///
/// layout.json の `tiling` フィールドの値（例: `"left"`, `"top-left"`）を検証しつつ
/// 対応する `TileKeyword` に変換する。許容されるキーワード一覧にないキーワードは
/// バリデーションエラーとなる（Issue #121で決定。エイリアス・大文字表記は非対応）。
///
/// # Arguments
/// * `value` - `tiling` フィールドの文字列値
///
/// # Returns
/// * `Ok(TileKeyword)` - 対応する `TileKeyword`
/// * `Err(AppConfigError)` - 未知のキーワードが指定された場合
pub fn parse_tile_keyword(value: &str) -> Result<TileKeyword, AppConfigError> {
    match value {
        "left" => Ok(TileKeyword::Left),
        "right" => Ok(TileKeyword::Right),
        "top" => Ok(TileKeyword::Top),
        "bottom" => Ok(TileKeyword::Bottom),
        "top-left" => Ok(TileKeyword::TopLeft),
        "top-right" => Ok(TileKeyword::TopRight),
        "bottom-left" => Ok(TileKeyword::BottomLeft),
        "bottom-right" => Ok(TileKeyword::BottomRight),
        "full-screen" => Ok(TileKeyword::FullScreen),
        _ => Err(AppConfigError {
            message: format!(
                "無効な tiling 値: '{}' ({} を指定)",
                value,
                TILE_KEYWORD_VALUES.join(", ")
            ),
        }),
    }
}

/// `serde_json::Value` が未指定（`null`）かどうかを判定する
fn value_is_absent(value: &serde_json::Value) -> bool {
    value.is_null()
}

// =============================================================================
// position/size の個別フィールド部分指定への対応（Issue #120）
// =============================================================================

/// `Position` の `x`/`y` のうち未指定（`null`）のフィールドを、現在のウィンドウ位置で補完する
///
/// `x`/`y` を片方だけ指定した `layout.json` に対応するための関数。指定されていない側は
/// 変更せず現在のウィンドウ位置を維持したいため、`load` 実行時にディスプレイ相対座標へ
/// 変換済みの現在位置（`current`）で補完してから、既存の [`parse_position_value`] に渡す。
///
/// `x`/`y` のどちらも `null` でない場合（補完が不要な場合）は `current` を使用しないため、
/// `current` が `None` でも常に `Some` を返す。
///
/// # Arguments
/// * `position` - `layout.json` の位置指定（一部フィールドが `null` の可能性がある）
/// * `current` - 現在のウィンドウ位置（ディスプレイ原点を差し引いた相対座標）。
///   取得できなかった場合は `None`
///
/// # Returns
/// * `Some(Position)` - `x`/`y` の `null` フィールドを補完した新しい `Position`
/// * `None` - `null` フィールドの補完が必要だが `current` が取得できなかった場合。
///   呼び出し側は、現在値が不明な状態で位置を確定させることを避けるため、
///   このウィンドウの位置・サイズ操作自体をスキップすることを想定している
pub fn fill_absent_position(position: &Position, current: Option<(i32, i32)>) -> Option<Position> {
    let x_absent = value_is_absent(&position.x);
    let y_absent = value_is_absent(&position.y);

    if (x_absent || y_absent) && current.is_none() {
        return None;
    }

    let (current_x, current_y) = current.unwrap_or((0, 0));
    Some(Position {
        x: if x_absent {
            serde_json::json!(current_x)
        } else {
            position.x.clone()
        },
        y: if y_absent {
            serde_json::json!(current_y)
        } else {
            position.y.clone()
        },
    })
}

/// `Size` の `width`/`height` のうち未指定（`null`）のフィールドを、現在のウィンドウサイズで補完する
///
/// `width`/`height` を片方だけ指定した `layout.json` に対応するための関数。指定されていない
/// 側は変更せず現在のウィンドウサイズを維持したいため、現在サイズ（`current`）で補完してから
/// 既存の [`parse_size_value`] に渡す。
///
/// `width`/`height` のどちらも `null` でない場合（補完が不要な場合）は `current` を
/// 使用しないため、`current` が `None` でも常に `Some` を返す。
///
/// # Arguments
/// * `size` - `layout.json` のサイズ指定（一部フィールドが `null` の可能性がある）
/// * `current` - 現在のウィンドウサイズ（幅, 高さ）。取得できなかった場合は `None`
///
/// # Returns
/// * `Some(Size)` - `width`/`height` の `null` フィールドを補完した新しい `Size`
/// * `None` - `null` フィールドの補完が必要だが `current` が取得できなかった場合。
///   呼び出し側は、現在値が不明な状態でサイズを確定させることを避けるため、
///   このウィンドウの位置・サイズ操作自体をスキップすることを想定している
pub fn fill_absent_size(size: &Size, current: Option<(i32, i32)>) -> Option<Size> {
    let width_absent = value_is_absent(&size.width);
    let height_absent = value_is_absent(&size.height);

    if (width_absent || height_absent) && current.is_none() {
        return None;
    }

    let (current_width, current_height) = current.unwrap_or((0, 0));
    Some(Size {
        width: if width_absent {
            serde_json::json!(current_width)
        } else {
            size.width.clone()
        },
        height: if height_absent {
            serde_json::json!(current_height)
        } else {
            size.height.clone()
        },
    })
}

// 注意: 以前存在した `resolve_tile_keyword()`（position/size のパターン指定から
// OS標準タイリングを推測する関数、Issue #118）は、Issue #123 の設計変更に伴い削除された。
// OS標準タイリング機能は、position/size 指定からの自動推測ではなく、Issue #123 で
// 追加される layout.json version 2.0 の明示的な `tiling` フィールドが指定された場合
// にのみ動作する（position/size 指定時は常に従来の直接プロパティ設定処理を使用する）。
