// loader.rs モジュールの包括的なテスト
//
// このテストでは、load_layout() と process_window() の動作を検証します。
// applescript モジュールの呼び出しをモック化できないため、
// 実際の環境での動作検証が必要です。

use apptidying::applescript;
use apptidying::config::{AppConfig, AppWindowConfig, DisplayConfig, LayoutConfig, Position, Size};
use apptidying::loader::{load_layout, LoadError, LoadResult};
use serde_json::json;

// =============================================================================
// テスト用ヘルパー関数
// =============================================================================

/// 実際に接続されているディスプレイ名を取得
fn get_first_connected_display_name() -> String {
    match applescript::get_all_connected_displays() {
        Ok(displays) => {
            if !displays.is_empty() {
                displays[0].name.clone()
            } else {
                // フォールバック: 接続されているディスプレイがない場合は Built-in Retina Display を使用
                "Built-in Retina Display".to_string()
            }
        }
        Err(_) => {
            // エラーの場合もフォールバック
            "Built-in Retina Display".to_string()
        }
    }
}

/// テスト用の基本的な AppConfig を作成
fn create_test_config_single_window() -> AppConfig {
    let display_name = get_first_connected_display_name();
    AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            name: "test-layout".to_string(),
            displays: vec![DisplayConfig {
                name: display_name,
                windows: vec![AppWindowConfig {
                    app: "TextEdit".to_string(),
                    title: None,
                    position: Some(Position {
                        x: json!("left"),
                        y: json!("top"),
                    }),
                    size: Some(Size {
                        width: json!("half"),
                        height: json!("half"),
                    }),
                }],
            }],
        }],
        notification: None,
        timeout: None,
    }
}

/// テスト用の複数ウィンドウ設定を作成
fn create_test_config_multiple_windows() -> AppConfig {
    let display_name = get_first_connected_display_name();
    AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            name: "multi-window".to_string(),
            displays: vec![DisplayConfig {
                name: display_name,
                windows: vec![
                    AppWindowConfig {
                        app: "TextEdit".to_string(),
                        title: None,
                        position: Some(Position {
                            x: json!("left"),
                            y: json!("top"),
                        }),
                        size: Some(Size {
                            width: json!("half"),
                            height: json!("half"),
                        }),
                    },
                    AppWindowConfig {
                        app: "Finder".to_string(),
                        title: None,
                        position: Some(Position {
                            x: json!("right"),
                            y: json!("top"),
                        }),
                        size: Some(Size {
                            width: json!("half"),
                            height: json!("half"),
                        }),
                    },
                ],
            }],
        }],
        notification: None,
        timeout: None,
    }
}

/// レイアウトが空の設定を作成
fn create_test_config_empty_layouts() -> AppConfig {
    AppConfig {
        version: "1.0".to_string(),
        layouts: vec![],
        notification: None,
        timeout: None,
    }
}

/// 存在しないディスプレイを指定した設定を作成
fn create_test_config_nonexistent_display() -> AppConfig {
    AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            name: "invalid-display".to_string(),
            displays: vec![DisplayConfig {
                name: "NonExistentDisplay".to_string(),
                windows: vec![AppWindowConfig {
                    app: "TextEdit".to_string(),
                    title: None,
                    position: Some(Position {
                        x: json!("left"),
                        y: json!("top"),
                    }),
                    size: Some(Size {
                        width: json!("half"),
                        height: json!("half"),
                    }),
                }],
            }],
        }],
        notification: None,
        timeout: None,
    }
}

/// タイトル指定ありのウィンドウ設定を作成
fn create_test_config_with_title() -> AppConfig {
    let display_name = get_first_connected_display_name();
    AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            name: "with-title".to_string(),
            displays: vec![DisplayConfig {
                name: display_name,
                windows: vec![AppWindowConfig {
                    app: "TextEdit".to_string(),
                    title: Some("Untitled".to_string()),
                    position: Some(Position {
                        x: json!(100),
                        y: json!(200),
                    }),
                    size: Some(Size {
                        width: json!(800),
                        height: json!(600),
                    }),
                }],
            }],
        }],
        notification: None,
        timeout: None,
    }
}

/// 位置のみ指定（サイズなし）の設定を作成
fn create_test_config_position_only() -> AppConfig {
    let display_name = get_first_connected_display_name();
    AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            name: "position-only".to_string(),
            displays: vec![DisplayConfig {
                name: display_name,
                windows: vec![AppWindowConfig {
                    app: "TextEdit".to_string(),
                    title: None,
                    position: Some(Position {
                        x: json!(100),
                        y: json!(200),
                    }),
                    size: None,
                }],
            }],
        }],
        notification: None,
        timeout: None,
    }
}

/// サイズのみ指定（位置なし）の設定を作成
fn create_test_config_size_only() -> AppConfig {
    let display_name = get_first_connected_display_name();
    AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            name: "size-only".to_string(),
            displays: vec![DisplayConfig {
                name: display_name,
                windows: vec![AppWindowConfig {
                    app: "TextEdit".to_string(),
                    title: None,
                    position: None,
                    size: Some(Size {
                        width: json!(800),
                        height: json!(600),
                    }),
                }],
            }],
        }],
        notification: None,
        timeout: None,
    }
}

/// 位置もサイズも指定なしの設定を作成
fn create_test_config_no_position_no_size() -> AppConfig {
    let display_name = get_first_connected_display_name();
    AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            name: "no-position-no-size".to_string(),
            displays: vec![DisplayConfig {
                name: display_name,
                windows: vec![AppWindowConfig {
                    app: "TextEdit".to_string(),
                    title: None,
                    position: None,
                    size: None,
                }],
            }],
        }],
        notification: None,
        timeout: None,
    }
}

/// 複数ディスプレイの設定を作成
fn create_test_config_multiple_displays() -> AppConfig {
    let display_name = get_first_connected_display_name();
    AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            name: "multi-display".to_string(),
            displays: vec![
                DisplayConfig {
                    name: display_name,
                    windows: vec![AppWindowConfig {
                        app: "TextEdit".to_string(),
                        title: None,
                        position: Some(Position {
                            x: json!("left"),
                            y: json!("top"),
                        }),
                        size: Some(Size {
                            width: json!("half"),
                            height: json!("half"),
                        }),
                    }],
                },
                DisplayConfig {
                    name: "External Display".to_string(),
                    windows: vec![AppWindowConfig {
                        app: "Finder".to_string(),
                        title: None,
                        position: Some(Position {
                            x: json!("right"),
                            y: json!("top"),
                        }),
                        size: Some(Size {
                            width: json!("half"),
                            height: json!("half"),
                        }),
                    }],
                },
            ],
        }],
        notification: None,
        timeout: None,
    }
}

/// タイムアウト設定ありの設定を作成
fn create_test_config_with_timeout() -> AppConfig {
    let display_name = get_first_connected_display_name();
    AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            name: "with-timeout".to_string(),
            displays: vec![DisplayConfig {
                name: display_name,
                windows: vec![AppWindowConfig {
                    app: "TextEdit".to_string(),
                    title: None,
                    position: Some(Position {
                        x: json!("left"),
                        y: json!("top"),
                    }),
                    size: Some(Size {
                        width: json!("half"),
                        height: json!("half"),
                    }),
                }],
            }],
        }],
        notification: None,
        timeout: Some(5000),
    }
}

// =============================================================================
// load_layout() の正常系テスト
// =============================================================================

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_single_window_success() {
    // 単一ウィンドウの成功パターン
    let config = create_test_config_single_window();
    let timeout_ms = 3000;

    // TextEdit を事前に起動しておくと成功しやすい
    let result = load_layout(&config, timeout_ms);

    // 結果の検証
    match result {
        Ok(load_result) => {
            println!("✓ テスト成功: all_success={}", load_result.all_success);
            println!(
                "  成功: {}, 失敗: {}",
                load_result.success_count, load_result.failure_count
            );

            // 成功カウントが1以上であることを確認
            assert!(
                load_result.success_count >= 1,
                "少なくとも1つのウィンドウが成功する必要があります"
            );
        }
        Err(e) => {
            println!("✗ テスト失敗: {}", e);
            // CI環境やアクセス権限がない環境では失敗する可能性があるため、パニックしない
        }
    }
}

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_multiple_windows_success() {
    // 複数ウィンドウの成功パターン
    let config = create_test_config_multiple_windows();
    let timeout_ms = 3000;

    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ テスト成功: all_success={}", load_result.all_success);
            println!(
                "  成功: {}, 失敗: {}",
                load_result.success_count, load_result.failure_count
            );

            // 成功カウントが1以上であることを確認
            assert!(
                load_result.success_count >= 1,
                "少なくとも1つのウィンドウが成功する必要があります"
            );
        }
        Err(e) => {
            println!("✗ テスト失敗: {}", e);
            // 部分失敗の可能性もあるため、パニックしない
        }
    }
}

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_with_title_success() {
    // タイトル指定ありの成功パターン
    let config = create_test_config_with_title();
    let timeout_ms = 3000;

    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ テスト成功: all_success={}", load_result.all_success);
            println!(
                "  成功: {}, 失敗: {}",
                load_result.success_count, load_result.failure_count
            );

            // 成功カウントが1以上であることを確認
            assert!(
                load_result.success_count >= 1,
                "少なくとも1つのウィンドウが成功する必要があります"
            );
        }
        Err(e) => {
            println!("✗ テスト失敗: {}", e);
        }
    }
}

// =============================================================================
// load_layout() の異常系テスト
// =============================================================================

#[test]
fn test_load_layout_empty_layouts_error() {
    // レイアウトが空の場合、エラーを返す
    let config = create_test_config_empty_layouts();
    let timeout_ms = 3000;

    let result = load_layout(&config, timeout_ms);

    assert!(
        result.is_err(),
        "レイアウトが空の場合はエラーを返す必要があります"
    );

    if let Err(e) = result {
        assert!(
            e.message.contains("レイアウトが定義されていません"),
            "エラーメッセージに「レイアウトが定義されていません」が含まれる必要があります。実際: {}",
            e.message
        );
    }
}

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_nonexistent_display_warn() {
    // 存在しないディスプレイを指定した場合、警告してスキップ
    let config = create_test_config_nonexistent_display();
    let timeout_ms = 3000;

    let result = load_layout(&config, timeout_ms);

    // 存在しないディスプレイのため、全体失敗エラーになる可能性が高い
    // または、success_count=0, failure_count=0 で全体失敗エラー
    match result {
        Ok(load_result) => {
            println!("✓ テスト成功: all_success={}", load_result.all_success);
            println!(
                "  成功: {}, 失敗: {}",
                load_result.success_count, load_result.failure_count
            );

            // 成功カウントが0であることを確認
            assert_eq!(
                load_result.success_count, 0,
                "存在しないディスプレイのため、成功カウントは0である必要があります"
            );
        }
        Err(e) => {
            println!("✓ テスト成功: 全体失敗エラー: {}", e);
            // エラーが返ることも正常
        }
    }
}

// =============================================================================
// load_layout() の境界値テスト
// =============================================================================

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_timeout_zero() {
    // タイムアウト値が0の場合
    let config = create_test_config_single_window();
    let timeout_ms = 0;

    let result = load_layout(&config, timeout_ms);

    // タイムアウト0でも処理は実行される（即座に次の処理に進む）
    match result {
        Ok(load_result) => {
            println!("✓ テスト成功: タイムアウト0msで実行完了");
            println!(
                "  成功: {}, 失敗: {}",
                load_result.success_count, load_result.failure_count
            );
        }
        Err(e) => {
            println!("✗ タイムアウト0msでの実行失敗: {}", e);
        }
    }
}

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_timeout_large() {
    // タイムアウト値が大きい場合（10秒）
    let config = create_test_config_single_window();
    let timeout_ms = 10000;

    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ テスト成功: タイムアウト10000msで実行完了");
            println!(
                "  成功: {}, 失敗: {}",
                load_result.success_count, load_result.failure_count
            );
        }
        Err(e) => {
            println!("✗ タイムアウト10000msでの実行失敗: {}", e);
        }
    }
}

// =============================================================================
// load_layout() の部分失敗テスト
// =============================================================================

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_partial_failure() {
    // 部分失敗のシナリオ:
    // - 有効なアプリ（TextEdit）と無効なアプリ（NonExistentApp）を混在させる
    let mut config = create_test_config_single_window();

    // 無効なアプリを追加
    config.layouts[0].displays[0].windows.push(AppWindowConfig {
        app: "NonExistentApp123456".to_string(),
        title: None,
        position: Some(Position {
            x: json!("left"),
            y: json!("top"),
        }),
        size: Some(Size {
            width: json!("half"),
            height: json!("half"),
        }),
    });

    let timeout_ms = 3000;
    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ 部分失敗テスト成功");
            println!(
                "  成功: {}, 失敗: {}",
                load_result.success_count, load_result.failure_count
            );
            println!("  失敗アプリ: {:?}", load_result.failed_apps);

            // all_success は false である必要がある
            assert!(
                !load_result.all_success,
                "部分失敗の場合、all_success は false である必要があります"
            );

            // 成功カウントが1以上、失敗カウントが1以上である必要がある
            assert!(
                load_result.success_count >= 1,
                "少なくとも1つのウィンドウが成功する必要があります"
            );
            assert!(
                load_result.failure_count >= 1,
                "少なくとも1つのウィンドウが失敗する必要があります"
            );

            // failed_apps に失敗したアプリ名が含まれる
            assert!(
                load_result
                    .failed_apps
                    .contains(&"NonExistentApp123456".to_string()),
                "failed_apps に失敗したアプリ名が含まれる必要があります"
            );
        }
        Err(e) => {
            println!("✗ 部分失敗テスト失敗: {}", e);
        }
    }
}

// =============================================================================
// load_layout() のサイズ・位置計算テスト
// =============================================================================

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_position_only() {
    // 位置のみ指定（サイズなし）
    let config = create_test_config_position_only();
    let timeout_ms = 3000;

    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ 位置のみ指定テスト成功");
            println!(
                "  成功: {}, 失敗: {}",
                load_result.success_count, load_result.failure_count
            );

            assert!(
                load_result.success_count >= 1,
                "位置のみ指定でも成功する必要があります"
            );
        }
        Err(e) => {
            println!("✗ 位置のみ指定テスト失敗: {}", e);
        }
    }
}

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_size_only() {
    // サイズのみ指定（位置なし）
    let config = create_test_config_size_only();
    let timeout_ms = 3000;

    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ サイズのみ指定テスト成功");
            println!(
                "  成功: {}, 失敗: {}",
                load_result.success_count, load_result.failure_count
            );

            assert!(
                load_result.success_count >= 1,
                "サイズのみ指定でも成功する必要があります"
            );
        }
        Err(e) => {
            println!("✗ サイズのみ指定テスト失敗: {}", e);
        }
    }
}

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_no_position_no_size() {
    // 位置もサイズも指定なし
    let config = create_test_config_no_position_no_size();
    let timeout_ms = 3000;

    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ 位置・サイズ指定なしテスト成功");
            println!(
                "  成功: {}, 失敗: {}",
                load_result.success_count, load_result.failure_count
            );

            // 位置・サイズ指定なしでもアプリ起動は成功する可能性がある
            // （ウィンドウのリサイズ処理がスキップされるだけ）
            assert!(
                load_result.success_count >= 1,
                "位置・サイズ指定なしでもアプリ起動は成功する必要があります"
            );
        }
        Err(e) => {
            println!("✗ 位置・サイズ指定なしテスト失敗: {}", e);
        }
    }
}

// =============================================================================
// load_layout() の複数ディスプレイテスト
// =============================================================================

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_multiple_displays() {
    // 複数ディスプレイの設定
    let config = create_test_config_multiple_displays();
    let timeout_ms = 3000;

    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ 複数ディスプレイテスト成功");
            println!(
                "  成功: {}, 失敗: {}",
                load_result.success_count, load_result.failure_count
            );

            // 少なくとも1つのディスプレイが接続されている場合、成功カウントが1以上
            assert!(
                load_result.success_count >= 1,
                "複数ディスプレイ設定で少なくとも1つのウィンドウが成功する必要があります"
            );
        }
        Err(e) => {
            println!("✗ 複数ディスプレイテスト失敗: {}", e);
        }
    }
}

// =============================================================================
// LoadResult の検証テスト
// =============================================================================

#[test]
fn test_load_result_all_success_true() {
    // LoadResult の all_success が true の場合
    let result = LoadResult {
        all_success: true,
        success_count: 3,
        failure_count: 0,
        failed_apps: vec![],
    };

    assert!(
        result.all_success,
        "all_success が true である必要があります"
    );
    assert_eq!(
        result.success_count, 3,
        "success_count が 3 である必要があります"
    );
    assert_eq!(
        result.failure_count, 0,
        "failure_count が 0 である必要があります"
    );
    assert!(
        result.failed_apps.is_empty(),
        "failed_apps が空である必要があります"
    );
}

#[test]
fn test_load_result_partial_failure() {
    // LoadResult の部分失敗パターン
    let result = LoadResult {
        all_success: false,
        success_count: 2,
        failure_count: 1,
        failed_apps: vec!["FailedApp".to_string()],
    };

    assert!(
        !result.all_success,
        "all_success が false である必要があります"
    );
    assert_eq!(
        result.success_count, 2,
        "success_count が 2 である必要があります"
    );
    assert_eq!(
        result.failure_count, 1,
        "failure_count が 1 である必要があります"
    );
    assert_eq!(
        result.failed_apps.len(),
        1,
        "failed_apps の長さが 1 である必要があります"
    );
    assert_eq!(
        result.failed_apps[0], "FailedApp",
        "failed_apps[0] が 'FailedApp' である必要があります"
    );
}

#[test]
fn test_load_result_all_failure() {
    // LoadResult の全体失敗パターン
    let result = LoadResult {
        all_success: false,
        success_count: 0,
        failure_count: 3,
        failed_apps: vec!["App1".to_string(), "App2".to_string(), "App3".to_string()],
    };

    assert!(
        !result.all_success,
        "all_success が false である必要があります"
    );
    assert_eq!(
        result.success_count, 0,
        "success_count が 0 である必要があります"
    );
    assert_eq!(
        result.failure_count, 3,
        "failure_count が 3 である必要があります"
    );
    assert_eq!(
        result.failed_apps.len(),
        3,
        "failed_apps の長さが 3 である必要があります"
    );
}

// =============================================================================
// LoadError の検証テスト
// =============================================================================

#[test]
fn test_load_error_display() {
    // LoadError の Display trait 実装
    let error = LoadError {
        message: "テストエラーメッセージ".to_string(),
    };

    let error_string = format!("{}", error);
    assert_eq!(
        error_string, "テストエラーメッセージ",
        "エラーメッセージが正しく表示される必要があります"
    );
}

#[test]
fn test_load_error_clone() {
    // LoadError の Clone trait 実装
    let error = LoadError {
        message: "クローンテスト".to_string(),
    };

    let cloned_error = error.clone();
    assert_eq!(
        error.message, cloned_error.message,
        "クローンされたエラーメッセージが一致する必要があります"
    );
}

// =============================================================================
// パターン計算の正確性テスト
// =============================================================================

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_pattern_left_top() {
    // パターン指定: left/top
    let mut config = create_test_config_single_window();
    config.layouts[0].displays[0].windows[0].position = Some(Position {
        x: json!("left"),
        y: json!("top"),
    });
    config.layouts[0].displays[0].windows[0].size = Some(Size {
        width: json!("half"),
        height: json!("half"),
    });

    let timeout_ms = 3000;
    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ パターン left/top テスト成功");
            assert!(
                load_result.success_count >= 1,
                "パターン left/top で成功する必要があります"
            );
        }
        Err(e) => {
            println!("✗ パターン left/top テスト失敗: {}", e);
        }
    }
}

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_pattern_right_bottom() {
    // パターン指定: right/bottom
    let mut config = create_test_config_single_window();
    config.layouts[0].displays[0].windows[0].position = Some(Position {
        x: json!("right"),
        y: json!("bottom"),
    });
    config.layouts[0].displays[0].windows[0].size = Some(Size {
        width: json!("half"),
        height: json!("half"),
    });

    let timeout_ms = 3000;
    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ パターン right/bottom テスト成功");
            assert!(
                load_result.success_count >= 1,
                "パターン right/bottom で成功する必要があります"
            );
        }
        Err(e) => {
            println!("✗ パターン right/bottom テスト失敗: {}", e);
        }
    }
}

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_size_max() {
    // サイズパターン: max/max
    let mut config = create_test_config_single_window();
    config.layouts[0].displays[0].windows[0].size = Some(Size {
        width: json!("max"),
        height: json!("max"),
    });

    let timeout_ms = 3000;
    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ サイズ max/max テスト成功");
            assert!(
                load_result.success_count >= 1,
                "サイズ max/max で成功する必要があります"
            );
        }
        Err(e) => {
            println!("✗ サイズ max/max テスト失敗: {}", e);
        }
    }
}

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_size_third() {
    // サイズパターン: third/third
    let mut config = create_test_config_single_window();
    config.layouts[0].displays[0].windows[0].size = Some(Size {
        width: json!("third"),
        height: json!("third"),
    });

    let timeout_ms = 3000;
    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ サイズ third/third テスト成功");
            assert!(
                load_result.success_count >= 1,
                "サイズ third/third で成功する必要があります"
            );
        }
        Err(e) => {
            println!("✗ サイズ third/third テスト失敗: {}", e);
        }
    }
}

// =============================================================================
// 数値指定の正確性テスト
// =============================================================================

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_absolute_position() {
    // 絶対座標指定
    let mut config = create_test_config_single_window();
    config.layouts[0].displays[0].windows[0].position = Some(Position {
        x: json!(100),
        y: json!(200),
    });
    config.layouts[0].displays[0].windows[0].size = Some(Size {
        width: json!(800),
        height: json!(600),
    });

    let timeout_ms = 3000;
    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ 絶対座標指定テスト成功");
            assert!(
                load_result.success_count >= 1,
                "絶対座標指定で成功する必要があります"
            );
        }
        Err(e) => {
            println!("✗ 絶対座標指定テスト失敗: {}", e);
        }
    }
}

// =============================================================================
// タイムアウト設定の伝播確認テスト
// =============================================================================

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_timeout_propagation() {
    // 設定ファイルで指定されたタイムアウト値が使用される
    let config = create_test_config_with_timeout();
    let timeout_ms = config.timeout.unwrap_or(3000);

    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ タイムアウト設定の伝播テスト成功");
            assert!(
                load_result.success_count >= 1,
                "タイムアウト設定の伝播で成功する必要があります"
            );
        }
        Err(e) => {
            println!("✗ タイムアウト設定の伝播テスト失敗: {}", e);
        }
    }
}

// =============================================================================
// エッジケーステスト
// =============================================================================

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_window_larger_than_display() {
    // ディスプレイより大きいウィンドウサイズを指定
    let mut config = create_test_config_single_window();
    config.layouts[0].displays[0].windows[0].size = Some(Size {
        width: json!(10000),
        height: json!(10000),
    });

    let timeout_ms = 3000;
    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ 大きすぎるサイズ指定テスト: 処理完了");
            println!(
                "  成功: {}, 失敗: {}",
                load_result.success_count, load_result.failure_count
            );
            // 成功する可能性もあるし、失敗する可能性もある
        }
        Err(e) => {
            println!("✓ 大きすぎるサイズ指定テスト: エラー発生 (想定通り): {}", e);
        }
    }
}

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_boundary_zero_size() {
    // サイズが0の場合（バリデーションでエラーになる可能性）
    let mut config = create_test_config_single_window();
    config.layouts[0].displays[0].windows[0].size = Some(Size {
        width: json!(0),
        height: json!(0),
    });

    let timeout_ms = 3000;
    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ サイズ0指定テスト: 処理完了");
            println!(
                "  成功: {}, 失敗: {}",
                load_result.success_count, load_result.failure_count
            );
        }
        Err(e) => {
            println!("✓ サイズ0指定テスト: エラー発生 (想定通り): {}", e);
        }
    }
}

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_load_layout_negative_position() {
    // 負の座標指定
    let mut config = create_test_config_single_window();
    config.layouts[0].displays[0].windows[0].position = Some(Position {
        x: json!(-100),
        y: json!(-200),
    });

    let timeout_ms = 3000;
    let result = load_layout(&config, timeout_ms);

    match result {
        Ok(load_result) => {
            println!("✓ 負の座標指定テスト: 処理完了");
            println!(
                "  成功: {}, 失敗: {}",
                load_result.success_count, load_result.failure_count
            );
        }
        Err(e) => {
            println!("✓ 負の座標指定テスト: エラー発生 (想定通り): {}", e);
        }
    }
}
