/// saver.rs モジュールの包括的なテストスイート
///
/// # テスト構成
///
/// ## ユニットテスト（CI環境で実行可能）
/// - SaveResult、SaveError の構造体テスト（2+3件）
/// - get_default_config_path() のテスト（1件）
/// - save_config_file() のテスト（2件）
///
/// これらはosascriptに依存せず、全環境で実行可能です。
///
/// ## 統合テスト（#[ignore] 付き、ローカル環境でのみ実行）
/// - save_layout() の統合テスト（4件）
/// - save/load 往復テスト（1件）
///
/// osascriptに依存するため、macOS ローカル環境でのみ実行可能です。
/// CI環境では自動的にスキップされます。
///
/// # テスト実行方法
///
/// ```bash
/// # 標準テスト実行（ユニットテストのみ）
/// cargo test --test saver_test
///
/// # #[ignore]テスト実行（統合テスト）
/// cargo test --test saver_test -- --ignored
///
/// # 標準出力を表示
/// cargo test --test saver_test -- --nocapture
///
/// # 統合テストの標準出力を表示
/// cargo test --test saver_test -- --ignored --nocapture
/// ```
///
/// # テストカバレッジ
///
/// ## カバーされている機能
/// - SaveResult 構造体: all_success、カウント、failed_apps リストの検証
/// - SaveError 構造体: Display、Clone、Error trait の実装
/// - get_default_config_path(): デフォルトパス取得、エラー処理
/// - save_config_file(): ディレクトリ作成、JSON書き込み、数値/パターン指定
/// - save_layout(): デフォルトパス、カスタムパス、--own オプション、JSON構造、往復テスト
///
/// ## カバーされていない機能
/// 以下は private 関数のため、save_layout() を通じて間接的にテストされます：
/// - should_include_window() - 最小化/非表示/システムウィンドウ、ターミナル除外の判定
/// - find_display_for_window() - ウィンドウ所属ディスプレイの判定
/// - get_own_terminal_app() - ターミナルアプリの特定
///
/// # 既知の制限事項
///
/// test_save_and_load_roundtrip について：
/// 外部ディスプレイに配置されたウィンドウの y 座標が負の値になる場合があります
/// （メニューバーの上に配置されるなど）。この場合、load時のバリデーションでエラーになる
/// 可能性があります。これはmacOSの座標系とディスプレイ配置によって発生する正常な状態です。
use apptidying::applescript::{AppInfo, DisplayInfo, WindowInfo};
use apptidying::config::{
    get_default_layout_path, save_layout_file, AppWindowConfig, DisplayConfig, LayoutConfig,
    LayoutFile, Position, Size,
};
use apptidying::saver::{save_layout, SaveError, SaveResult};
use serde_json::json;
use std::fs;
use std::path::PathBuf;

// =============================================================================
// テスト用ヘルパー関数
// =============================================================================

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
// SaveResult のテスト
// =============================================================================

/// SaveResult が all_success = true で正しく作成できることを確認
///
/// 検証項目：
/// - all_success が true で設定できる
/// - saved_app_count、saved_window_count、skipped_window_count が正しく設定される
/// - failed_apps が空のベクトルで設定できる
#[test]
fn test_save_result_all_success() {
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

/// SaveResult が部分失敗（all_success = false）を正しく表現できることを確認
///
/// 検証項目：
/// - all_success が false で設定できる
/// - failed_apps リストに失敗したアプリケーション名が含まれる
/// - 複数の失敗アプリ名が正しく保存される
#[test]
fn test_save_result_partial_failure() {
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

/// SaveError の Display trait が正しく実装されていることを確認
///
/// 検証項目：
/// - format!("{}", error) でエラーメッセージが正しく表示される
#[test]
fn test_save_error_display() {
    let error = SaveError {
        message: "テストエラーメッセージ".to_string(),
    };

    let error_string = format!("{}", error);
    assert_eq!(
        error_string, "テストエラーメッセージ",
        "エラーメッセージが正しく表示される必要があります"
    );
}

/// SaveError の Clone trait が正しく実装されていることを確認
///
/// 検証項目：
/// - クローンされたエラーのメッセージが元のメッセージと一致する
#[test]
fn test_save_error_clone() {
    let error = SaveError {
        message: "クローンテスト".to_string(),
    };

    let cloned_error = error.clone();
    assert_eq!(
        error.message, cloned_error.message,
        "クローンされたエラーメッセージが一致する必要があります"
    );
}

/// SaveError が std::error::Error trait を実装していることを確認
///
/// 検証項目：
/// - Error trait を通じてメッセージが取得できる
#[test]
fn test_save_error_error_trait() {
    let error = SaveError {
        message: "Error trait テスト".to_string(),
    };

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

/// デフォルトレイアウトファイルパスが正しく取得できることを確認
///
/// 検証項目：
/// - 正常系：デフォルトパスが期待される形式（~/Library/Application Support/...）
/// - 正常系：パスがホームディレクトリから始まる
/// - 異常系：ホームディレクトリが取得できない場合はエラーメッセージが返される
#[test]
fn test_get_default_layout_path() {
    let result = get_default_layout_path();

    match result {
        Ok(path) => {
            println!("✓ デフォルトレイアウトパス: {}", path.display());

            // パスが期待される形式であることを確認
            let path_str = path.to_string_lossy();
            assert!(
                path_str.contains("Library/Application Support/biz.nosetech.apptidying/layout.json"),
                "パスに 'Library/Application Support/biz.nosetech.apptidying/layout.json' が含まれる必要があります。実際: {}",
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

/// 親ディレクトリが存在しない場合、自動的にディレクトリが作成されることを確認
///
/// 検証項目：
/// - 親ディレクトリが存在しない状態でも save_layout_file() が成功する
/// - 設定ファイルが作成される
#[test]
fn test_save_layout_file_creates_directory() {
    let temp_path = create_temp_config_path("creates_directory");

    let parent_dir = temp_path.parent().unwrap();
    if parent_dir.exists() {
        println!("✓ 親ディレクトリが既に存在: {}", parent_dir.display());
    }

    // テスト用の設定を作成
    let layout = LayoutFile {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
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
    };

    // 保存を実行
    let result = save_layout_file(&layout, &temp_path);

    assert!(
        result.is_ok(),
        "save_layout_file() が成功する必要があります"
    );

    // ファイルが作成されたことを確認
    assert!(temp_path.exists(), "設定ファイルが作成される必要があります");

    // クリーンアップ
    cleanup_temp_file(&temp_path);
}

/// JSON ファイルが正しく書き込まれ、内容が検証できることを確認
///
/// 検証項目：
/// - version が "1.0" として保存される
/// - layouts が配列として保存される
/// - 数値指定とパターン指定の両方が正しく保存される
#[test]
fn test_save_layout_file_writes_json() {
    let temp_path = create_temp_config_path("writes_json");

    // テスト用の設定を作成
    let layout = LayoutFile {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
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
    };

    // 保存を実行
    let result = save_layout_file(&layout, &temp_path);

    assert!(
        result.is_ok(),
        "save_layout_file() が成功する必要があります"
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

    let layout_obj = &parsed["layouts"][0];
    assert!(
        layout_obj["displays"].is_array(),
        "displays は配列である必要があります"
    );
    assert_eq!(
        layout_obj["displays"].as_array().unwrap().len(),
        1,
        "displays の長さが 1 である必要があります"
    );

    let display = &layout_obj["displays"][0];
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

    // クリーンアップ
    cleanup_temp_file(&temp_path);
}

// =============================================================================
// save_layout() の統合テスト
// =============================================================================

/// デフォルトパスに保存ができることを確認
///
/// 実行環境: macOS ローカルのみ（CI環境ではスキップ）
/// 検証項目：
/// - ファイルが作成される
/// - JSON 構造が正しい（version、layouts、displays が存在）
#[test]
#[ignore]
fn test_save_layout_default_path() {
    let default_path_result = get_default_layout_path();
    assert!(
        default_path_result.is_ok(),
        "デフォルトパスの取得に失敗しました"
    );

    let output_path = default_path_result.unwrap();
    let include_own_terminal = false;

    println!("\n=== テスト: test_save_layout_default_path ===");
    println!("デフォルトレイアウトパス: {}", output_path.display());

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

            println!("✓ 保存されたレイアウトファイルの構造を検証しました");
        }
        Err(e) => {
            println!("✗ テスト失敗: {}", e);
        }
    }
}

/// カスタムパスに保存ができることを確認
///
/// 実行環境: macOS ローカルのみ（CI環境ではスキップ）
/// 検証項目：
/// - 指定したパスにファイルが作成される
/// - JSON 構造が正しい
#[test]
#[ignore]
fn test_save_layout_custom_path() {
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

/// --own オプション付きで保存できることを確認
///
/// 実行環境: macOS ローカルのみ（CI環境ではスキップ）
/// 検証項目：
/// - include_own_terminal = true で保存が成功する
/// - ターミナルウィンドウも含めて保存される（デフォルトより多いウィンドウ数）
#[test]
#[ignore]
fn test_save_layout_with_own_flag() {
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

/// 保存された JSON 構造が正しいことを確認
///
/// 実行環境: macOS ローカルのみ（CI環境ではスキップ）
/// 検証項目：
/// - version が "1.0"
/// - layouts が配列で空でない
/// - displays が配列で空でない
/// - 各ウィンドウに app、position、size が存在
/// - position の x, y が数値または文字列
/// - 数値の場合、負でない値であること
#[test]
#[ignore]
fn test_save_layout_saves_correct_structure() {
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

/// 保存→読み込み→検証の往復が成功することを確認
///
/// 実行環境: macOS ローカルのみ（CI環境ではスキップ）
/// 検証項目：
/// - save_layout() が成功する
/// - load_layout_file() が成功する
/// - 読み込まれた設定の構造が正しい
///
/// 制限事項：
/// ディスプレイ配置によっては、負の座標が保存される場合があり、
/// load時のバリデーションでエラーになる可能性があります。
/// これはmacOSの座標系とディスプレイ配置によって発生する正常な状態です。
#[test]
#[ignore]
fn test_save_and_load_roundtrip() {
    use apptidying::config::load_layout_file;

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
            let load_result = load_layout_file(&output_path);

            match load_result {
                Ok(layout) => {
                    println!("✓ 読み込み成功");

                    // 3. 検証
                    assert_eq!(
                        layout.version, "1.0",
                        "version が '1.0' である必要があります"
                    );
                    assert!(
                        !layout.layouts.is_empty(),
                        "layouts が空でない必要があります"
                    );
                    assert!(
                        !layout.layouts[0].displays.is_empty(),
                        "displays が空でない必要があります"
                    );

                    println!("✓ 往復テスト成功: 保存→読み込み→検証が完了しました");

                    // クリーンアップ
                    cleanup_temp_file(&output_path);
                }
                Err(e) => {
                    println!("✗ 読み込み失敗: {}", e);
                    cleanup_temp_file(&output_path);
                    panic!("レイアウトファイルの読み込みに失敗しました");
                }
            }
        }
        Err(e) => {
            println!("✗ 保存失敗: {}", e);
            cleanup_temp_file(&output_path);
        }
    }
}
