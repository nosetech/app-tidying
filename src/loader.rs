use crate::applescript::{self, DisplayInfo};
use crate::config::{AppConfig, AppWindowConfig};
use std::thread;
use std::time::Duration;

/// load機能の実行結果
#[derive(Debug, Clone)]
pub struct LoadResult {
    /// すべて成功したか
    pub all_success: bool,
    /// 成功したウィンドウ数
    pub success_count: usize,
    /// 失敗したウィンドウ数
    pub failure_count: usize,
    /// 失敗したアプリ名のリスト
    pub failed_apps: Vec<String>,
}

/// load機能のエラー型
#[derive(Debug, Clone)]
pub struct LoadError {
    /// エラーメッセージ
    pub message: String,
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LoadError {}

/// ウィンドウレイアウトを復元する
///
/// # Arguments
/// * `config` - 設定ファイル (AppConfig)
/// * `timeout_ms` - アプリ起動待機時間（ミリ秒）
///
/// # Returns
/// * `Ok(LoadResult)` - 成功または部分成功
/// * `Err(LoadError)` - 全体失敗（致命的エラー）
pub fn load_layout(config: &AppConfig, timeout_ms: u64) -> Result<LoadResult, LoadError> {
    // 1. 接続されているディスプレイ情報を取得
    let _connected_displays = applescript::get_all_connected_displays().map_err(|e| LoadError {
        message: format!("ディスプレイ情報の取得に失敗しました: {}", e),
    })?;

    log::debug!("接続ディスプレイ情報を取得しました");

    // 2. 最初のレイアウトを使用
    let layout = config.layouts.first().ok_or_else(|| LoadError {
        message: "レイアウトが定義されていません".to_string(),
    })?;

    log::info!("レイアウト '{}' を適用します", layout.name);

    // 3. 成功・失敗カウンタ
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut failed_apps: Vec<String> = Vec::new();

    // 4. 各ディスプレイの設定を処理
    for display_config in &layout.displays {
        log::debug!("ディスプレイ '{}' の処理を開始", display_config.name);

        // 4.1 ディスプレイ情報を取得
        let display_info = match applescript::get_display_info(Some(&display_config.name)) {
            Ok(info) => info,
            Err(e) => {
                log::warn!(
                    "ディスプレイ '{}' が接続されていません: {}。スキップします。",
                    display_config.name,
                    e
                );
                continue;
            }
        };

        log::debug!(
            "ディスプレイ情報: {} ({}x{})",
            display_info.name,
            display_info.width,
            display_info.height
        );

        // 4.2 各ウィンドウの設定を処理
        for window_config in &display_config.windows {
            log::debug!("アプリ '{}' の処理を開始", window_config.app);

            match process_window(window_config, &display_info, timeout_ms) {
                Ok(()) => {
                    log::info!(
                        "アプリ '{}' のウィンドウ配置に成功しました",
                        window_config.app
                    );
                    success_count += 1;
                }
                Err(e) => {
                    log::warn!(
                        "アプリ '{}' のウィンドウ配置に失敗しました: {}",
                        window_config.app,
                        e
                    );
                    failure_count += 1;
                    if !failed_apps.contains(&window_config.app) {
                        failed_apps.push(window_config.app.clone());
                    }
                }
            }
        }
    }

    // 5. 結果の判定
    if failure_count == 0 {
        log::info!(
            "すべてのウィンドウ配置に成功しました（成功: {}）",
            success_count
        );
        Ok(LoadResult {
            all_success: true,
            success_count,
            failure_count,
            failed_apps,
        })
    } else if success_count > 0 {
        // 部分失敗
        log::warn!(
            "一部のウィンドウ配置に失敗しました（成功: {}, 失敗: {}, 失敗したアプリ: {}）",
            success_count,
            failure_count,
            failed_apps.join(", ")
        );
        Ok(LoadResult {
            all_success: false,
            success_count,
            failure_count,
            failed_apps,
        })
    } else {
        // 全体失敗
        let error_msg = format!(
            "すべてのウィンドウ配置に失敗しました。失敗したアプリ: {}",
            failed_apps.join(", ")
        );
        log::error!("{}", error_msg);
        Err(LoadError { message: error_msg })
    }
}

/// 個別ウィンドウの処理
fn process_window(
    window_config: &AppWindowConfig,
    display_info: &DisplayInfo,
    timeout_ms: u64,
) -> Result<(), String> {
    // 1. アプリを起動またはアクティブ化
    log::debug!(
        "アプリ '{}' を起動またはアクティブ化します",
        window_config.app
    );
    let _launch_result = applescript::launch_or_activate_app(&window_config.app, timeout_ms)
        .map_err(|e| format!("アプリ起動失敗: {}", e))?;

    // 2. ウィンドウの存在確認
    let window_exists = if let Some(ref title) = window_config.title {
        log::debug!("ウィンドウタイトル '{}' を検索します", title);
        match applescript::find_window_by_title(&window_config.app, title) {
            Ok(window_info_opt) => window_info_opt.is_some(),
            Err(e) => {
                log::warn!("ウィンドウ検索でエラーが発生しました: {}", e);
                false
            }
        }
    } else {
        // タイトル指定なしの場合、全ウィンドウを取得して存在確認
        match applescript::get_all_windows(&window_config.app) {
            Ok(windows) => !windows.is_empty(),
            Err(e) => {
                log::warn!("ウィンドウ一覧取得でエラーが発生しました: {}", e);
                false
            }
        }
    };

    // 3. ウィンドウが存在しない場合は新規作成
    if !window_exists {
        log::info!("ウィンドウが存在しないため、新規作成します");
        applescript::create_new_window(&window_config.app)
            .map_err(|e| format!("新規ウィンドウ作成失敗: {}", e))?;

        // 新規ウィンドウの作成を待機
        thread::sleep(Duration::from_millis(500));
    }

    // 4. サイズと位置を計算
    let position_opt = if let Some(ref position) = window_config.position {
        // サイズがあれば計算、なければデフォルトを使用
        let width = if let Some(ref size) = window_config.size {
            calculate_width(size, display_info.width)?
        } else {
            display_info.width
        };

        let height = if let Some(ref size) = window_config.size {
            calculate_height(size, display_info.height)?
        } else {
            display_info.height
        };

        Some(calculate_position(
            position,
            display_info.width,
            display_info.height,
            width,
            height,
        )?)
    } else {
        None
    };

    let size_opt = if let Some(ref size) = window_config.size {
        let width = calculate_width(size, display_info.width)?;
        let height = calculate_height(size, display_info.height)?;
        Some((width, height))
    } else {
        None
    };

    if let (Some(position), Some(size)) = (position_opt, size_opt) {
        log::debug!(
            "ウィンドウを配置します: 位置=({}, {}), サイズ=({}, {})",
            position.0,
            position.1,
            size.0,
            size.1
        );
    } else if let Some(position) = position_opt {
        log::debug!(
            "ウィンドウを移動します: 位置=({}, {})",
            position.0,
            position.1
        );
    } else if let Some(size) = size_opt {
        log::debug!(
            "ウィンドウをリサイズします: サイズ=({}, {})",
            size.0,
            size.1
        );
    }

    // 5. ウィンドウを移動・リサイズ
    applescript::resize_window(
        &window_config.app,
        window_config.title.as_deref(),
        position_opt,
        size_opt,
    )
    .map_err(|e| format!("ウィンドウリサイズ失敗: {}", e))?;

    Ok(())
}

/// 幅を計算する
fn calculate_width(size: &crate::config::Size, display_width: i32) -> Result<i32, String> {
    use crate::config::parse_size_value;
    use serde_json::Value;

    let width_value = match &size.width {
        v if *v == Value::Null => Value::Null,
        v => v.clone(),
    };

    if width_value == Value::Null {
        return Ok(display_width);
    }

    match parse_size_value(&width_value, display_width, display_width, "width") {
        Ok((w, _)) => Ok(w),
        Err(e) => Err(format!("幅計算失敗: {}", e)),
    }
}

/// 高さを計算する
fn calculate_height(size: &crate::config::Size, display_height: i32) -> Result<i32, String> {
    use crate::config::parse_size_value;
    use serde_json::Value;

    let height_value = match &size.height {
        v if *v == Value::Null => Value::Null,
        v => v.clone(),
    };

    if height_value == Value::Null {
        return Ok(display_height);
    }

    match parse_size_value(&height_value, display_height, display_height, "height") {
        Ok((_, h)) => Ok(h),
        Err(e) => Err(format!("高さ計算失敗: {}", e)),
    }
}

/// 位置を計算する
fn calculate_position(
    position: &crate::config::Position,
    display_width: i32,
    display_height: i32,
    window_width: i32,
    window_height: i32,
) -> Result<(i32, i32), String> {
    use crate::config::parse_position_value;

    match parse_position_value(
        &serde_json::to_value(position).unwrap(),
        display_width,
        display_height,
        window_width,
        window_height,
        "position",
    ) {
        Ok((x, y)) => Ok((x, y)),
        Err(e) => Err(format!("位置計算失敗: {}", e)),
    }
}
