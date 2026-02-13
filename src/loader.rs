use crate::applescript::{self, DisplayInfo};
use crate::config::{AppWindowConfig, LayoutFile};
use rayon::prelude::*;
use std::collections::HashSet;
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

/// ウィンドウ処理の結果
#[derive(Debug, Clone)]
struct WindowProcessResult {
    /// アプリケーション名
    app_name: String,
    /// 処理に成功したか
    success: bool,
    /// エラーメッセージ（失敗時）
    error_message: Option<String>,
}

/// ウィンドウレイアウトを復元する
///
/// # Arguments
/// * `layout` - レイアウトファイル (LayoutFile)
/// * `timeout_ms` - アプリ起動待機時間（ミリ秒）
///
/// # Returns
/// * `Ok(LoadResult)` - 成功または部分成功
/// * `Err(LoadError)` - 全体失敗（致命的エラー）
pub fn load_layout(layout: &LayoutFile, timeout_ms: u64) -> Result<LoadResult, LoadError> {
    // 1. 接続されているディスプレイ情報を取得
    let connected_displays = applescript::get_all_connected_displays().map_err(|e| LoadError {
        message: format!("ディスプレイ情報の取得に失敗しました: {}", e),
    })?;

    log::debug!("接続ディスプレイ情報を取得しました");

    // 2. レイアウトファイルの境界値チェックを実行
    let warnings =
        crate::config::validate_layout_bounds(layout, &connected_displays).map_err(|e| {
            LoadError {
                message: format!("設定ファイルの検証に失敗しました: {}", e),
            }
        })?;

    // ワーニングをログ出力
    for warning in &warnings {
        log::warn!(
            "ディスプレイ '{}' のアプリ '{}': {}",
            warning.display_name,
            warning.app_name,
            warning.message
        );
    }

    // 3. 最初のレイアウトを使用
    let layout_config = layout.layouts.first().ok_or_else(|| LoadError {
        message: "レイアウトが定義されていません".to_string(),
    })?;

    log::info!("レイアウトを適用します");

    // 4. ワーニング対象をHashSetに変換（O(1)検索用）
    let warning_set: HashSet<(String, String)> = warnings
        .iter()
        .map(|w| (w.display_name.clone(), w.app_name.clone()))
        .collect();

    // 4. 成功・失敗カウンタ
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut failed_apps: Vec<String> = Vec::new();

    // 5. 各ディスプレイの設定を処理
    for display_config in &layout_config.displays {
        log::debug!("ディスプレイ '{}' の処理を開始", display_config.name);

        // 5.1 ディスプレイ情報を取得
        let (display_info, is_fallback) =
            match applescript::get_display_info(Some(&display_config.name)) {
                Ok(info) => (info, false),
                Err(e) => {
                    log::warn!(
                        "ディスプレイ '{}' が接続されていません: {}",
                        display_config.name,
                        e
                    );

                    // フォールバック: 接続されているディスプレイの最初のものを使用
                    if let Some(fallback_display) = connected_displays.first() {
                        log::info!(
                            "ディスプレイ '{}' を使用してウィンドウを配置します（フォールバック）",
                            fallback_display.name
                        );
                        (fallback_display.clone(), true)
                    } else {
                        // ディスプレイが一つも接続されていない（致命的エラー）
                        log::error!("接続されているディスプレイが見つかりません");
                        return Err(LoadError {
                            message: "接続されているディスプレイが見つかりません".to_string(),
                        });
                    }
                }
            };

        log::debug!(
            "ディスプレイ情報: {} ({}x{})",
            display_info.name,
            display_info.width,
            display_info.height
        );

        // 5.2 各ウィンドウの設定を処理（並列処理）
        let window_results: Vec<WindowProcessResult> = display_config
            .windows
            .par_iter()
            .map(|window_config| {
                log::debug!("アプリ '{}' の処理を開始", window_config.app);

                // ウィンドウがワーニング対象かチェック
                // フォールバック時は、フォールバック先のディスプレイに対してサイズ超過をチェック
                let has_warning = if is_fallback {
                    // フォールバック時：フォールバック先ディスプレイのサイズ超過をチェック
                    if let Some(ref size) = window_config.size {
                        let size_value =
                            serde_json::to_value(size).unwrap_or_else(|_| serde_json::json!({}));
                        match crate::config::parse_size_value(
                            &size_value,
                            display_info.width,
                            display_info.height,
                            "size",
                        ) {
                            Ok((width, height)) => {
                                // フォールバック先ディスプレイに収まらないか確認
                                width > display_info.width || height > display_info.height
                            }
                            Err(_) => false,
                        }
                    } else {
                        false
                    }
                } else {
                    // 非フォールバック時：元の警告セットをチェック
                    warning_set.contains(&(display_config.name.clone(), window_config.app.clone()))
                };

                if has_warning {
                    log::warn!(
                        "アプリ '{}' は検証エラーのためスキップされます",
                        window_config.app
                    );
                    return WindowProcessResult {
                        app_name: window_config.app.clone(),
                        success: false,
                        error_message: Some("検証エラー".to_string()),
                    };
                }

                match process_window(window_config, &display_info, timeout_ms) {
                    Ok(()) => {
                        log::info!(
                            "アプリ '{}' のウィンドウ配置に成功しました",
                            window_config.app
                        );
                        WindowProcessResult {
                            app_name: window_config.app.clone(),
                            success: true,
                            error_message: None,
                        }
                    }
                    Err(e) => {
                        log::warn!(
                            "アプリ '{}' のウィンドウ配置に失敗しました: {}",
                            window_config.app,
                            e
                        );
                        WindowProcessResult {
                            app_name: window_config.app.clone(),
                            success: false,
                            error_message: Some(e),
                        }
                    }
                }
            })
            .collect();

        // 並列処理の結果を集約
        for result in window_results {
            if result.success {
                success_count += 1;
            } else {
                failure_count += 1;
                if !failed_apps.contains(&result.app_name) {
                    failed_apps.push(result.app_name);
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
    let window_exists = match applescript::get_all_windows(&window_config.app) {
        Ok(windows) => !windows.is_empty(),
        Err(e) => {
            log::warn!("ウィンドウ一覧取得でエラーが発生しました: {}", e);
            false
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

    // 4. サイズを計算
    let (size_opt, position_opt) = if let Some(ref size) = window_config.size {
        let size_value = serde_json::to_value(size)
            .map_err(|e| format!("サイズ情報のシリアライズに失敗しました: {}", e))?;
        let (width, height) = crate::config::parse_size_value(
            &size_value,
            display_info.width,
            display_info.height,
            "size",
        )
        .map_err(|e| format!("サイズ計算失敗: {}", e))?;

        let position = if let Some(ref position) = window_config.position {
            let position_value = serde_json::to_value(position)
                .map_err(|e| format!("位置情報のシリアライズに失敗しました: {}", e))?;
            let (x, y) = crate::config::parse_position_value(
                &position_value,
                Some(&size_value),
                display_info.width,
                display_info.height,
                width,
                height,
                "position",
            )
            .map_err(|e| format!("位置計算失敗: {}", e))?;
            // ディスプレイの origin 座標を加算して、全体座標系に変換
            Some((x + display_info.origin_x, y + display_info.origin_y))
        } else {
            None
        };

        (Some((width, height)), position)
    } else if let Some(ref position) = window_config.position {
        // サイズ指定なしの場合はディスプレイサイズを使用
        let position_value = serde_json::to_value(position)
            .map_err(|e| format!("位置情報のシリアライズに失敗しました: {}", e))?;
        let (x, y) = crate::config::parse_position_value(
            &position_value,
            None,
            display_info.width,
            display_info.height,
            display_info.width,
            display_info.height,
            "position",
        )
        .map_err(|e| format!("位置計算失敗: {}", e))?;
        // ディスプレイの origin 座標を加算して、全体座標系に変換
        (
            None,
            Some((x + display_info.origin_x, y + display_info.origin_y)),
        )
    } else {
        (None, None)
    };

    if let (Some(size), Some(position)) = (size_opt, position_opt) {
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
    } else {
        log::debug!("位置もサイズも指定されていないため、ウィンドウ操作をスキップします");
        return Ok(());
    }

    // 5. ウィンドウを移動・リサイズ
    applescript::resize_window(
        &window_config.app,
        position_opt,
        size_opt,
    )
    .map_err(|e| {
        log::warn!(
            "ウィンドウのリサイズに失敗しました: アプリ: {}, 位置: {:?}, サイズ: {:?}, AppleScript エラー: {}",
            window_config.app,
            position_opt,
            size_opt,
            e.message
        );
        format!("ウィンドウのリサイズに失敗しました: {}", e.message)
    })?;

    Ok(())
}
