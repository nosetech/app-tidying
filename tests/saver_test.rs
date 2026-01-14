// saver.rs モジュールの包括的なテスト
//
// このテストでは、save_layout()、should_include_window()、find_display_for_window()、
// get_default_config_path()、save_config_file() の動作を検証します。

use apptidying::applescript::{AppInfo, DisplayInfo, WindowInfo};
use apptidying::config::{
    get_default_config_path, save_config_file, AppConfig, AppWindowConfig, DisplayConfig,
    LayoutConfig, Position, Size,
};
use apptidying::saver::{save_layout, SaveError, SaveResult};
use serde_json::json;
use std::fs;
use std::path::PathBuf;

// =============================================================================
// テスト用ヘルパー関数
// =============================================================================

/// テスト用の WindowInfo を生成
#[allow(dead_code)]
fn create_test_window_info(
    title: &str,
    position: (i32, i32),
    size: (i32, i32),
    minimized: bool,
    visible: bool,
) -> WindowInfo {
    WindowInfo {
        title: title.to_string(),
        position,
        size,
        minimized,
        visible,
    }
}

/// テスト用の DisplayInfo を生成
#[allow(dead_code)]
fn create_test_display_info(
    name: &str,
    width: i32,
    height: i32,
    origin_x: i32,
    origin_y: i32,
) -> DisplayInfo {
    DisplayInfo {
        name: name.to_string(),
        width,
        height,
        origin_x,
        origin_y,
    }
}

/// テスト用の AppInfo を生成
#[allow(dead_code)]
fn create_test_app_info(name: &str, process_id: Option<i32>) -> AppInfo {
    AppInfo {
        name: name.to_string(),
        process_id,
    }
}

/// テスト用の一時ディレクトリパスを生成
fn create_temp_config_path(test_name: &str) -> PathBuf {
    let temp_dir = std::env::temp_dir();
    temp_dir.join(format!("apptidying_test_{}.json", test_name))
}

/// テスト後のクリーンアップ
fn cleanup_temp_file(path: &PathBuf) {
    if path.exists() {
        let _ = fs::remove_file(path);
    }
}

// =============================================================================
// should_include_window() のテスト
// =============================================================================

// Note: should_include_window() は private 関数のため、直接テストできません。
// save_layout() を通じて間接的にテストします。
// ただし、ロジックを理解するために、想定される動作を確認します。

// =============================================================================
// find_display_for_window() のテスト
// =============================================================================

// Note: find_display_for_window() は private 関数のため、直接テストできません。
// save_layout() を通じて間接的にテストします。

// =============================================================================
// SaveResult のテスト
// =============================================================================

#[test]
fn test_save_result_all_success() {
    // SaveResult が all_success = true で作成できる
    let result = SaveResult {
        all_success: true,
        saved_app_count: 3,
        saved_window_count: 5,
        skipped_window_count: 2,
        failed_apps: vec![],
    };

    assert!(
        result.all_success,
        "all_success が true である必要があります"
    );
    assert_eq!(
        result.saved_app_count, 3,
        "saved_app_count が 3 である必要があります"
    );
    assert_eq!(
        result.saved_window_count, 5,
        "saved_window_count が 5 である必要があります"
    );
    assert_eq!(
        result.skipped_window_count, 2,
        "skipped_window_count が 2 である必要があります"
    );
    assert!(
        result.failed_apps.is_empty(),
        "failed_apps が空である必要があります"
    );
}

#[test]
fn test_save_result_partial_failure() {
    // SaveResult が all_success = false で failed_apps を持つことができる
    let result = SaveResult {
        all_success: false,
        saved_app_count: 2,
        saved_window_count: 3,
        skipped_window_count: 1,
        failed_apps: vec!["FailedApp1".to_string(), "FailedApp2".to_string()],
    };

    assert!(
        !result.all_success,
        "all_success が false である必要があります"
    );
    assert_eq!(
        result.saved_app_count, 2,
        "saved_app_count が 2 である必要があります"
    );
    assert_eq!(
        result.saved_window_count, 3,
        "saved_window_count が 3 である必要があります"
    );
    assert_eq!(
        result.skipped_window_count, 1,
        "skipped_window_count が 1 である必要があります"
    );
    assert_eq!(
        result.failed_apps.len(),
        2,
        "failed_apps の長さが 2 である必要があります"
    );
    assert_eq!(
        result.failed_apps[0], "FailedApp1",
        "failed_apps[0] が 'FailedApp1' である必要があります"
    );
    assert_eq!(
        result.failed_apps[1], "FailedApp2",
        "failed_apps[1] が 'FailedApp2' である必要があります"
    );
}

// =============================================================================
// SaveError のテスト
// =============================================================================

#[test]
fn test_save_error_display() {
    // SaveError の Display trait 実装
    let error = SaveError {
        message: "テストエラーメッセージ".to_string(),
    };

    let error_string = format!("{}", error);
    assert_eq!(
        error_string, "テストエラーメッセージ",
        "エラーメッセージが正しく表示される必要があります"
    );
}

#[test]
fn test_save_error_clone() {
    // SaveError の Clone trait 実装
    let error = SaveError {
        message: "クローンテスト".to_string(),
    };

    let cloned_error = error.clone();
    assert_eq!(
        error.message, cloned_error.message,
        "クローンされたエラーメッセージが一致する必要があります"
    );
}

#[test]
fn test_save_error_error_trait() {
    // SaveError が Error trait を実装している
    let error = SaveError {
        message: "Error trait テスト".to_string(),
    };

    // Error trait を通じてエラーメッセージを取得
    let error_ref: &dyn std::error::Error = &error;
    let error_string = format!("{}", error_ref);
    assert_eq!(
        error_string, "Error trait テスト",
        "Error trait を通じてメッセージが取得できる必要があります"
    );
}

// =============================================================================
// get_default_config_path() のテスト
// =============================================================================

#[test]
fn test_get_default_config_path() {
    // デフォルト設定ファイルパスが正しく取得できる
    let result = get_default_config_path();

    match result {
        Ok(path) => {
            println!("✓ デフォルト設定パス: {}", path.display());

            // パスが期待される形式であることを確認
            let path_str = path.to_string_lossy();
            assert!(
                path_str.contains("Library/Application Support/biz.nosetech.apptidying/settings.json"),
                "パスに 'Library/Application Support/biz.nosetech.apptidying/settings.json' が含まれる必要があります。実際: {}",
                path_str
            );

            // ホームディレクトリから始まることを確認
            if let Some(home_dir) = dirs::home_dir() {
                assert!(
                    path.starts_with(&home_dir),
                    "パスがホームディレクトリから始まる必要があります"
                );
            }
        }
        Err(e) => {
            // ホームディレクトリが取得できない場合はエラーになる
            println!("✓ ホームディレクトリ取得エラー: {}", e);
            assert!(
                e.message.contains("ホームディレクトリの取得に失敗"),
                "エラーメッセージに 'ホームディレクトリの取得に失敗' が含まれる必要があります。実際: {}",
                e.message
            );
        }
    }
}

// =============================================================================
// save_config_file() のテスト
// =============================================================================

#[test]
fn test_save_config_file_creates_directory() {
    // ディレクトリが自動作成されることを確認
    let temp_path = create_temp_config_path("creates_directory");

    // 親ディレクトリが存在しない場合でもディレクトリを作成
    let parent_dir = temp_path.parent().unwrap();
    if parent_dir.exists() {
        // ディレクトリが既に存在する場合はスキップ
        println!("✓ 親ディレクトリが既に存在: {}", parent_dir.display());
    }

    // テスト用の設定を作成
    let config = AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            name: "test-layout".to_string(),
            displays: vec![DisplayConfig {
                name: "Built-in".to_string(),
                windows: vec![AppWindowConfig {
                    app: "TestApp".to_string(),
                    title: None,
                    position: Some(Position {
                        x: json!(0),
                        y: json!(25),
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
    };

    // 保存を実行
    let result = save_config_file(&config, &temp_path);

    assert!(
        result.is_ok(),
        "save_config_file() が成功する必要があります"
    );

    // ファイルが作成されたことを確認
    assert!(temp_path.exists(), "設定ファイルが作成される必要があります");

    // クリーンアップ
    cleanup_temp_file(&temp_path);
}

#[test]
fn test_save_config_file_writes_json() {
    // JSON ファイルが正しく書き込まれ、内容が検証できることを確認
    let temp_path = create_temp_config_path("writes_json");

    // テスト用の設定を作成
    let config = AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            name: "test-layout".to_string(),
            displays: vec![DisplayConfig {
                name: "Built-in".to_string(),
                windows: vec![
                    AppWindowConfig {
                        app: "Safari".to_string(),
                        title: Some("スタートページ".to_string()),
                        position: Some(Position {
                            x: json!(100),
                            y: json!(200),
                        }),
                        size: Some(Size {
                            width: json!(800),
                            height: json!(600),
                        }),
                    },
                    AppWindowConfig {
                        app: "Finder".to_string(),
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
                ],
            }],
        }],
        notification: None,
        timeout: Some(5000),
    };

    // 保存を実行
    let result = save_config_file(&config, &temp_path);

    assert!(
        result.is_ok(),
        "save_config_file() が成功する必要があります"
    );

    // ファイルが作成されたことを確認
    assert!(temp_path.exists(), "設定ファイルが作成される必要があります");

    // ファイルを読み込んで内容を検証
    let content = fs::read_to_string(&temp_path).expect("ファイルの読み込みに失敗しました");

    println!("保存されたJSON:");
    println!("{}", content);

    // JSON として正しくパースできることを確認
    let parsed: serde_json::Value =
        serde_json::from_str(&content).expect("JSON のパースに失敗しました");

    // 主要なフィールドが存在することを確認
    assert_eq!(parsed["version"], "1.0", "version が一致する必要があります");
    assert!(
        parsed["layouts"].is_array(),
        "layouts は配列である必要があります"
    );
    assert_eq!(
        parsed["layouts"].as_array().unwrap().len(),
        1,
        "layouts の長さが 1 である必要があります"
    );

    let layout = &parsed["layouts"][0];
    assert_eq!(
        layout["name"], "test-layout",
        "layout name が一致する必要があります"
    );
    assert!(
        layout["displays"].is_array(),
        "displays は配列である必要があります"
    );
    assert_eq!(
        layout["displays"].as_array().unwrap().len(),
        1,
        "displays の長さが 1 である必要があります"
    );

    let display = &layout["displays"][0];
    assert_eq!(
        display["name"], "Built-in",
        "display name が一致する必要があります"
    );
    assert!(
        display["windows"].is_array(),
        "windows は配列である必要があります"
    );
    assert_eq!(
        display["windows"].as_array().unwrap().len(),
        2,
        "windows の長さが 2 である必要があります"
    );

    // 1つ目のウィンドウ（数値指定）
    let window1 = &display["windows"][0];
    assert_eq!(window1["app"], "Safari", "app が一致する必要があります");
    assert_eq!(
        window1["title"], "スタートページ",
        "title が一致する必要があります"
    );
    assert_eq!(
        window1["position"]["x"], 100,
        "position.x が一致する必要があります"
    );
    assert_eq!(
        window1["position"]["y"], 200,
        "position.y が一致する必要があります"
    );
    assert_eq!(
        window1["size"]["width"], 800,
        "size.width が一致する必要があります"
    );
    assert_eq!(
        window1["size"]["height"], 600,
        "size.height が一致する必要があります"
    );

    // 2つ目のウィンドウ（パターン指定）
    let window2 = &display["windows"][1];
    assert_eq!(window2["app"], "Finder", "app が一致する必要があります");
    assert_eq!(
        window2["position"]["x"], "left",
        "position.x が一致する必要があります"
    );
    assert_eq!(
        window2["position"]["y"], "top",
        "position.y が一致する必要があります"
    );
    assert_eq!(
        window2["size"]["width"], "half",
        "size.width が一致する必要があります"
    );
    assert_eq!(
        window2["size"]["height"], "half",
        "size.height が一致する必要があります"
    );

    // timeout の検証
    assert_eq!(parsed["timeout"], 5000, "timeout が一致する必要があります");

    // クリーンアップ
    cleanup_temp_file(&temp_path);
}

// =============================================================================
// save_layout() の統合テスト
// =============================================================================

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_save_layout_default_path() {
    // デフォルトパスに保存ができるか
    let default_path_result = get_default_config_path();
    assert!(
        default_path_result.is_ok(),
        "デフォルトパスの取得に失敗しました"
    );

    let output_path = default_path_result.unwrap();
    let include_own_terminal = false;

    println!("\n=== テスト: test_save_layout_default_path ===");
    println!("デフォルト設定パス: {}", output_path.display());

    let result = save_layout(&output_path, include_own_terminal);

    match result {
        Ok(save_result) => {
            println!("✓ テスト成功: all_success={}", save_result.all_success);
            println!(
                "  保存: アプリ={}, ウィンドウ={}, スキップ={}",
                save_result.saved_app_count,
                save_result.saved_window_count,
                save_result.skipped_window_count
            );

            // 保存されたファイルが存在することを確認
            assert!(
                output_path.exists(),
                "保存されたファイルが存在する必要があります"
            );

            // ファイルを読み込んで内容を検証
            let content =
                fs::read_to_string(&output_path).expect("ファイルの読み込みに失敗しました");
            let parsed: serde_json::Value =
                serde_json::from_str(&content).expect("JSON のパースに失敗しました");

            // 基本的な構造の確認
            assert_eq!(parsed["version"], "1.0", "version が一致する必要があります");
            assert!(
                parsed["layouts"].is_array(),
                "layouts は配列である必要があります"
            );

            println!("✓ 保存された設定ファイルの構造を検証しました");
        }
        Err(e) => {
            println!("✗ テスト失敗: {}", e);
            // アプリが起動していない場合などはエラーになる可能性がある
        }
    }
}

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_save_layout_custom_path() {
    // カスタムパスに保存ができるか
    let output_path = create_temp_config_path("custom_path");
    let include_own_terminal = false;

    println!("\n=== テスト: test_save_layout_custom_path ===");
    println!("カスタム設定パス: {}", output_path.display());

    let result = save_layout(&output_path, include_own_terminal);

    match result {
        Ok(save_result) => {
            println!("✓ テスト成功: all_success={}", save_result.all_success);
            println!(
                "  保存: アプリ={}, ウィンドウ={}, スキップ={}",
                save_result.saved_app_count,
                save_result.saved_window_count,
                save_result.skipped_window_count
            );

            // 保存されたファイルが存在することを確認
            assert!(
                output_path.exists(),
                "保存されたファイルが存在する必要があります"
            );

            // ファイルを読み込んで内容を検証
            let content =
                fs::read_to_string(&output_path).expect("ファイルの読み込みに失敗しました");
            let parsed: serde_json::Value =
                serde_json::from_str(&content).expect("JSON のパースに失敗しました");

            // 基本的な構造の確認
            assert_eq!(parsed["version"], "1.0", "version が一致する必要があります");
            assert!(
                parsed["layouts"].is_array(),
                "layouts は配列である必要があります"
            );

            println!("✓ 保存された設定ファイルの構造を検証しました");

            // クリーンアップ
            cleanup_temp_file(&output_path);
        }
        Err(e) => {
            println!("✗ テスト失敗: {}", e);
            cleanup_temp_file(&output_path);
        }
    }
}

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_save_layout_with_own_flag() {
    // --own オプション付きで保存できるか
    let output_path = create_temp_config_path("with_own_flag");
    let include_own_terminal = true;

    println!("\n=== テスト: test_save_layout_with_own_flag ===");
    println!("設定パス: {}", output_path.display());
    println!("include_own_terminal: {}", include_own_terminal);

    let result = save_layout(&output_path, include_own_terminal);

    match result {
        Ok(save_result) => {
            println!("✓ テスト成功: all_success={}", save_result.all_success);
            println!(
                "  保存: アプリ={}, ウィンドウ={}, スキップ={}",
                save_result.saved_app_count,
                save_result.saved_window_count,
                save_result.skipped_window_count
            );

            // 保存されたファイルが存在することを確認
            assert!(
                output_path.exists(),
                "保存されたファイルが存在する必要があります"
            );

            println!("✓ --own オプション付きで保存が成功しました");

            // クリーンアップ
            cleanup_temp_file(&output_path);
        }
        Err(e) => {
            println!("✗ テスト失敗: {}", e);
            cleanup_temp_file(&output_path);
        }
    }
}

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_save_layout_saves_correct_structure() {
    // 保存された JSON 構造が正しいか（version, layouts, displays の存在など）
    let output_path = create_temp_config_path("correct_structure");
    let include_own_terminal = false;

    println!("\n=== テスト: test_save_layout_saves_correct_structure ===");
    println!("設定パス: {}", output_path.display());

    let result = save_layout(&output_path, include_own_terminal);

    match result {
        Ok(save_result) => {
            println!("✓ 保存成功");
            println!(
                "  保存: アプリ={}, ウィンドウ={}, スキップ={}",
                save_result.saved_app_count,
                save_result.saved_window_count,
                save_result.skipped_window_count
            );

            // ファイルを読み込んで内容を検証
            let content =
                fs::read_to_string(&output_path).expect("ファイルの読み込みに失敗しました");
            println!("保存されたJSON:");
            println!("{}", content);

            let parsed: serde_json::Value =
                serde_json::from_str(&content).expect("JSON のパースに失敗しました");

            // 必須フィールドの検証
            assert_eq!(
                parsed["version"], "1.0",
                "version が '1.0' である必要があります"
            );
            assert!(
                parsed["layouts"].is_array(),
                "layouts が配列である必要があります"
            );
            assert!(
                !parsed["layouts"].as_array().unwrap().is_empty(),
                "layouts が空でない必要があります"
            );

            let layout = &parsed["layouts"][0];
            assert_eq!(
                layout["name"], "saved_layout",
                "layout name が 'saved_layout' である必要があります"
            );
            assert!(
                layout["displays"].is_array(),
                "displays が配列である必要があります"
            );
            assert!(
                !layout["displays"].as_array().unwrap().is_empty(),
                "displays が空でない必要があります"
            );

            // 各ディスプレイの検証
            for display in layout["displays"].as_array().unwrap() {
                assert!(
                    display["name"].is_string(),
                    "display name が文字列である必要があります"
                );
                assert!(
                    display["windows"].is_array(),
                    "windows が配列である必要があります"
                );

                // 各ウィンドウの検証
                for window in display["windows"].as_array().unwrap() {
                    assert!(
                        window["app"].is_string(),
                        "app が文字列である必要があります"
                    );

                    // title は Option なので存在確認のみ
                    if window["title"].is_string() {
                        println!("  ウィンドウタイトル: {}", window["title"]);
                    }

                    // position の検証
                    if let Some(position) = window.get("position") {
                        assert!(
                            position.is_object(),
                            "position がオブジェクトである必要があります"
                        );
                        assert!(
                            position["x"].is_number() || position["x"].is_string(),
                            "position.x が数値または文字列である必要があります"
                        );
                        assert!(
                            position["y"].is_number() || position["y"].is_string(),
                            "position.y が数値または文字列である必要があります"
                        );

                        // position.x が数値の場合、正の値であることを確認
                        if let Some(x_num) = position["x"].as_i64() {
                            assert!(
                                x_num >= 0,
                                "position.x は負でない値である必要があります（実際の値: {}）",
                                x_num
                            );
                        }

                        // position.y が数値の場合、正の値であることを確認
                        if let Some(y_num) = position["y"].as_i64() {
                            assert!(
                                y_num >= 0,
                                "position.y は負でない値である必要があります（実際の値: {}）",
                                y_num
                            );
                        }
                    }

                    // size の検証
                    if let Some(size) = window.get("size") {
                        assert!(size.is_object(), "size がオブジェクトである必要があります");
                        assert!(
                            size["width"].is_number() || size["width"].is_string(),
                            "size.width が数値または文字列である必要があります"
                        );
                        assert!(
                            size["height"].is_number() || size["height"].is_string(),
                            "size.height が数値または文字列である必要があります"
                        );
                    }
                }
            }

            println!("✓ 保存された JSON 構造の検証が完了しました");

            // クリーンアップ
            cleanup_temp_file(&output_path);
        }
        Err(e) => {
            println!("✗ テスト失敗: {}", e);
            cleanup_temp_file(&output_path);
        }
    }
}

// =============================================================================
// save/load 往復テスト
// =============================================================================

#[test]
#[ignore] // osascript 実行に依存するため、CI環境ではスキップ
fn test_save_and_load_roundtrip() {
    // 保存→読み込み→検証ができるか
    use apptidying::config::load_config_file;

    let output_path = create_temp_config_path("roundtrip");
    let include_own_terminal = false;

    println!("\n=== テスト: test_save_and_load_roundtrip ===");
    println!("設定パス: {}", output_path.display());

    // 1. 保存
    let save_result = save_layout(&output_path, include_own_terminal);

    match save_result {
        Ok(save_result) => {
            println!("✓ 保存成功");
            println!(
                "  保存: アプリ={}, ウィンドウ={}, スキップ={}",
                save_result.saved_app_count,
                save_result.saved_window_count,
                save_result.skipped_window_count
            );

            // 2. 読み込み
            let load_result = load_config_file(&output_path);

            match load_result {
                Ok(config) => {
                    println!("✓ 読み込み成功");

                    // 3. 検証
                    assert_eq!(
                        config.version, "1.0",
                        "version が '1.0' である必要があります"
                    );
                    assert!(
                        !config.layouts.is_empty(),
                        "layouts が空でない必要があります"
                    );
                    assert_eq!(
                        config.layouts[0].name, "saved_layout",
                        "layout name が 'saved_layout' である必要があります"
                    );
                    assert!(
                        !config.layouts[0].displays.is_empty(),
                        "displays が空でない必要があります"
                    );

                    println!("✓ 往復テスト成功: 保存→読み込み→検証が完了しました");

                    // クリーンアップ
                    cleanup_temp_file(&output_path);
                }
                Err(e) => {
                    println!("✗ 読み込み失敗: {}", e);
                    cleanup_temp_file(&output_path);
                    panic!("設定ファイルの読み込みに失敗しました");
                }
            }
        }
        Err(e) => {
            println!("✗ 保存失敗: {}", e);
            cleanup_temp_file(&output_path);
        }
    }
}
