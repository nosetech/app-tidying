use crate::applescript;
use crate::config::{AppConfig, AppWindowConfig, DisplayConfig, LayoutConfig, Position, Size};
use std::collections::HashMap;
use std::path::PathBuf;

/// save機能の実行結果
#[derive(Debug, Clone)]
pub struct SaveResult {
    /// すべて成功したか
    pub all_success: bool,
    /// 保存したアプリケーション数
    pub saved_app_count: usize,
    /// 保存したウィンドウ数
    pub saved_window_count: usize,
    /// スキップしたウィンドウ数（最小化・非表示・システムウィンドウ）
    pub skipped_window_count: usize,
    /// 失敗したアプリ名のリスト
    pub failed_apps: Vec<String>,
}

/// save機能のエラー型
#[derive(Debug, Clone)]
pub struct SaveError {
    pub message: String,
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SaveError {}

/// 現在のウィンドウレイアウトを保存する
///
/// # Arguments
/// * `output_path` - 保存先のファイルパス
/// * `include_own_terminal` - ターミナルウィンドウを含めるか（--own オプション）
///
/// # Returns
/// * `Ok(SaveResult)` - 成功または部分成功
/// * `Err(SaveError)` - 全体失敗（致命的エラー）
pub fn save_layout(
    output_path: &PathBuf,
    include_own_terminal: bool,
) -> Result<SaveResult, SaveError> {
    // 1. ディスプレイ情報を取得
    let displays = applescript::get_all_connected_displays().map_err(|e| SaveError {
        message: format!("ディスプレイ情報の取得に失敗しました: {}", e),
    })?;

    if displays.is_empty() {
        return Err(SaveError {
            message: "接続されているディスプレイが見つかりません".to_string(),
        });
    }

    // 2. 実行中アプリケーション一覧を取得
    let running_apps = applescript::get_running_applications().map_err(|e| SaveError {
        message: format!("実行中アプリケーション一覧の取得に失敗しました: {}", e),
    })?;

    // 3. 自プロセスIDを取得（ターミナルウィンドウ除外判定用）
    let own_process_id = if include_own_terminal {
        None
    } else {
        get_own_process_id()
    };

    // 4. 各アプリケーションのウィンドウ情報を収集
    let mut display_windows: HashMap<String, Vec<AppWindowConfig>> = HashMap::new();
    let mut saved_app_count = 0;
    let mut saved_window_count = 0;
    let mut skipped_window_count = 0;
    let mut failed_apps = Vec::new();

    for app in running_apps {
        // ウィンドウ情報を取得
        let windows = match applescript::get_all_windows(&app.name) {
            Ok(windows) => windows,
            Err(e) => {
                log::debug!("アプリ '{}' のウィンドウ取得に失敗: {}", app.name, e);
                failed_apps.push(app.name.clone());
                continue;
            }
        };

        if windows.is_empty() {
            continue;
        }

        // ウィンドウをフィルタリングして、ディスプレイ別に分類
        let mut app_saved_count = 0;
        for window in windows {
            // ウィンドウを保存対象に含めるか判定
            if !should_include_window(&window, &app.name, own_process_id, app.process_id) {
                skipped_window_count += 1;
                continue;
            }

            // ウィンドウが属するディスプレイを判定
            let display = match find_display_for_window(&window, &displays) {
                Some(d) => d,
                None => {
                    log::warn!(
                        "ウィンドウ '{}' (アプリ: {}) はディスプレイ範囲外です",
                        window.title,
                        app.name
                    );
                    skipped_window_count += 1;
                    continue;
                }
            };

            // AppWindowConfig を構築（ピクセル単位で保存）
            let window_config = AppWindowConfig {
                app: app.name.clone(),
                title: Some(window.title.clone()),
                position: Some(Position {
                    x: serde_json::json!(window.position.0),
                    y: serde_json::json!(window.position.1),
                }),
                size: Some(Size {
                    width: serde_json::json!(window.size.0),
                    height: serde_json::json!(window.size.1),
                }),
            };

            // ディスプレイごとにウィンドウを分類
            display_windows
                .entry(display.name.clone())
                .or_default()
                .push(window_config);

            saved_window_count += 1;
            app_saved_count += 1;
        }

        if app_saved_count > 0 {
            saved_app_count += 1;
        }
    }

    // 5. 保存すべきウィンドウが存在するか確認
    if saved_window_count == 0 {
        return Err(SaveError {
            message: "保存すべきウィンドウが見つかりませんでした。アプリケーションを起動してから再度お試しください".to_string(),
        });
    }

    // 6. AppConfig 構造体を構築
    let mut display_configs = Vec::new();
    for display in &displays {
        let windows = display_windows.remove(&display.name).unwrap_or_default();

        // ディスプレイに保存対象ウィンドウがある場合のみ含める
        if !windows.is_empty() {
            display_configs.push(DisplayConfig {
                name: display.name.clone(),
                windows,
            });
        }
    }

    let config = AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            name: "saved_layout".to_string(),
            displays: display_configs,
        }],
        notification: None,
        timeout: None,
    };

    // 7. JSONファイルに保存
    crate::config::save_config_file(&config, output_path).map_err(|e| SaveError {
        message: format!("設定ファイルの保存に失敗しました: {}", e),
    })?;

    log::info!(
        "ウィンドウレイアウトを保存しました: {} (アプリ: {}, ウィンドウ: {})",
        output_path.display(),
        saved_app_count,
        saved_window_count
    );

    Ok(SaveResult {
        all_success: failed_apps.is_empty(),
        saved_app_count,
        saved_window_count,
        skipped_window_count,
        failed_apps,
    })
}

/// ウィンドウを保存対象に含めるべきか判定
fn should_include_window(
    window: &applescript::WindowInfo,
    app_name: &str,
    own_process_id: Option<i32>,
    app_process_id: Option<i32>,
) -> bool {
    // 1. 最小化されたウィンドウを除外
    if window.minimized {
        log::debug!(
            "ウィンドウ '{}' は最小化されているためスキップ",
            window.title
        );
        return false;
    }

    // 2. 非表示ウィンドウを除外
    if !window.visible {
        log::debug!("ウィンドウ '{}' は非表示のためスキップ", window.title);
        return false;
    }

    // 3. システムウィンドウを除外
    if applescript::is_excluded_window(app_name, &window.title) {
        log::debug!(
            "ウィンドウ '{}' はシステムウィンドウのためスキップ",
            window.title
        );
        return false;
    }

    // 4. --own オプションが無効の場合、自ターミナルを除外
    if let (Some(own_pid), Some(app_pid)) = (own_process_id, app_process_id) {
        if own_pid == app_pid {
            log::debug!(
                "ウィンドウ '{}' は実行中のターミナルのためスキップ (--own なし)",
                window.title
            );
            return false;
        }
    }

    true
}

/// ウィンドウが属するディスプレイを判定
///
/// ウィンドウの位置座標から、どのディスプレイに属するかを判定します。
/// ウィンドウの中心座標がディスプレイの範囲内にあるかで判断します。
fn find_display_for_window(
    window: &applescript::WindowInfo,
    displays: &[applescript::DisplayInfo],
) -> Option<applescript::DisplayInfo> {
    // ウィンドウの中心座標を計算
    let center_x = window.position.0 + window.size.0 / 2;
    let center_y = window.position.1 + window.size.1 / 2;

    // 中心座標が含まれるディスプレイを検索
    for display in displays {
        let display_right = display.origin_x + display.width;
        let display_bottom = display.origin_y + display.height;

        if center_x >= display.origin_x
            && center_x < display_right
            && center_y >= display.origin_y
            && center_y < display_bottom
        {
            return Some(display.clone());
        }
    }

    None
}

/// 自プロセスIDを取得
fn get_own_process_id() -> Option<i32> {
    std::process::id().try_into().ok()
}
