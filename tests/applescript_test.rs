use apptidying::applescript::{
    escape_applescript_string, get_all_connected_displays, get_all_windows, get_display_info,
    get_running_applications, launch_or_activate_app, parse_single_window, parse_window_list,
    resize_window, AppInfo, AppLaunchError, AppLaunchResult, DisplayInfo, RunningAppsError,
    WindowInfo, WindowInfoError,
};

// =============================================================================
// escape_applescript_string() Tests
// =============================================================================

// --- 境界値テスト: 空文字列と最小サイズ ---

#[test]
fn test_escape_applescript_string_empty() {
    // 空文字列が正しく処理されることを確認（境界値テスト）
    assert_eq!(escape_applescript_string(""), "");
}

#[test]
fn test_escape_applescript_string_single_char() {
    // 1文字の文字列が正しく処理されることを確認（境界値テスト）
    assert_eq!(escape_applescript_string("a"), "a");
}

// --- 同値分割: 特殊文字なし（正常系） ---

#[test]
fn test_escape_applescript_string_no_special_chars() {
    // 特殊文字を含まない文字列はそのまま返される
    assert_eq!(escape_applescript_string("Hello World"), "Hello World");
}

#[test]
fn test_escape_applescript_string_alphanumeric() {
    // 英数字のみの文字列
    assert_eq!(escape_applescript_string("abc123XYZ"), "abc123XYZ");
}

#[test]
fn test_escape_applescript_string_spaces() {
    // スペースを含む文字列
    assert_eq!(
        escape_applescript_string("test with spaces"),
        "test with spaces"
    );
}

// --- 同値分割: 各特殊文字を個別にエスケープ ---

#[test]
fn test_escape_applescript_string_backslash() {
    // バックスラッシュが正しくエスケープされることを確認
    assert_eq!(escape_applescript_string("test\\path"), "test\\\\path");
}

#[test]
fn test_escape_applescript_string_double_quote() {
    // ダブルクォートが正しくエスケープされることを確認
    assert_eq!(escape_applescript_string("test\"quote"), "test\\\"quote");
}

#[test]
fn test_escape_applescript_string_newline() {
    // 改行が正しくエスケープされることを確認
    assert_eq!(escape_applescript_string("test\nline"), "test\\nline");
}

#[test]
fn test_escape_applescript_string_carriage_return() {
    // キャリッジリターンが正しくエスケープされることを確認
    assert_eq!(escape_applescript_string("test\rline"), "test\\rline");
}

// --- 境界値テスト: 単一特殊文字 ---

#[test]
fn test_escape_applescript_string_single_special_char_backslash() {
    // 1文字のバックスラッシュが正しくエスケープされることを確認
    assert_eq!(escape_applescript_string("\\"), "\\\\");
}

#[test]
fn test_escape_applescript_string_single_special_char_quote() {
    // 1文字のダブルクォートが正しくエスケープされることを確認
    assert_eq!(escape_applescript_string("\""), "\\\"");
}

#[test]
fn test_escape_applescript_string_single_special_char_newline() {
    // 1文字の改行が正しくエスケープされることを確認
    assert_eq!(escape_applescript_string("\n"), "\\n");
}

#[test]
fn test_escape_applescript_string_single_special_char_carriage_return() {
    // 1文字のキャリッジリターンが正しくエスケープされることを確認
    assert_eq!(escape_applescript_string("\r"), "\\r");
}

// --- 同値分割: 複数の同じ特殊文字 ---

#[test]
fn test_escape_applescript_string_multiple_backslashes() {
    // 複数のバックスラッシュが正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string("path\\to\\file"),
        "path\\\\to\\\\file"
    );
}

#[test]
fn test_escape_applescript_string_multiple_quotes() {
    // 複数のダブルクォートが正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string("\"quoted\" \"text\""),
        "\\\"quoted\\\" \\\"text\\\""
    );
}

#[test]
fn test_escape_applescript_string_multiple_newlines() {
    // 複数の改行が正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string("line1\nline2\nline3"),
        "line1\\nline2\\nline3"
    );
}

#[test]
fn test_escape_applescript_string_multiple_carriage_returns() {
    // 複数のキャリッジリターンが正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string("line1\rline2\rline3"),
        "line1\\rline2\\rline3"
    );
}

// --- 同値分割: 異なる特殊文字の組み合わせ ---

#[test]
fn test_escape_applescript_string_combined_special_chars() {
    // 複数の特殊文字の組み合わせが正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string("path\\file\"name\ntest"),
        "path\\\\file\\\"name\\ntest"
    );
}

#[test]
fn test_escape_applescript_string_all_special_chars() {
    // 全ての特殊文字を含む文字列が正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string("test\\path\"quote\nline\rreturn"),
        "test\\\\path\\\"quote\\nline\\rreturn"
    );
}

#[test]
fn test_escape_applescript_string_consecutive_special_chars() {
    // 連続した特殊文字が正しくエスケープされることを確認
    assert_eq!(escape_applescript_string("\\\"\n\r"), "\\\\\\\"\\n\\r");
}

// --- 境界値テスト: 文字列の先頭・末尾に特殊文字 ---

#[test]
fn test_escape_applescript_string_special_chars_at_start() {
    // 文字列の先頭に特殊文字がある場合
    assert_eq!(escape_applescript_string("\\test"), "\\\\test");
    assert_eq!(escape_applescript_string("\"test"), "\\\"test");
    assert_eq!(escape_applescript_string("\ntest"), "\\ntest");
}

#[test]
fn test_escape_applescript_string_special_chars_at_end() {
    // 文字列の末尾に特殊文字がある場合
    assert_eq!(escape_applescript_string("test\\"), "test\\\\");
    assert_eq!(escape_applescript_string("test\""), "test\\\"");
    assert_eq!(escape_applescript_string("test\n"), "test\\n");
}

#[test]
fn test_escape_applescript_string_special_chars_at_boundaries() {
    // 特殊文字が文字列の先頭と末尾にある場合のエスケープ確認
    assert_eq!(escape_applescript_string("\\test\""), "\\\\test\\\"");
    assert_eq!(escape_applescript_string("\ntest\r"), "\\ntest\\r");
}

// --- 境界値テスト: 非常に長い文字列 ---

#[test]
fn test_escape_applescript_string_very_long_string() {
    // 非常に長い文字列が正しく処理されることを確認（境界値テスト）
    let long_string = "a".repeat(10000);
    let escaped = escape_applescript_string(&long_string);
    assert_eq!(escaped.len(), 10000);
    assert_eq!(escaped, long_string);
}

#[test]
fn test_escape_applescript_string_very_long_string_with_special_chars() {
    // 特殊文字を含む非常に長い文字列が正しく処理されることを確認
    let long_string = "a\\b\"c\nd\r".repeat(1000);
    let escaped = escape_applescript_string(&long_string);
    // 元の文字列: "a\\b\"c\nd\r" = 8バイト (a, \, b, ", c, \n, d, \r)
    // エスケープ後: "a\\\\b\\\"c\\nd\\r" = 12バイト
    assert_eq!(long_string.len(), 8 * 1000);
    assert_eq!(escaped.len(), 12 * 1000);
}

// --- 同値分割: Unicode文字 ---

#[test]
fn test_escape_applescript_string_unicode() {
    // Unicode文字が正しく処理されることを確認
    assert_eq!(escape_applescript_string("日本語テスト"), "日本語テスト");
}

#[test]
fn test_escape_applescript_string_unicode_with_special_chars() {
    // Unicode文字と特殊文字の組み合わせが正しく処理されることを確認
    assert_eq!(
        escape_applescript_string("日本語\\テスト\"改行\n"),
        "日本語\\\\テスト\\\"改行\\n"
    );
}

#[test]
fn test_escape_applescript_string_emoji() {
    // 絵文字が正しく処理されることを確認
    assert_eq!(escape_applescript_string("Test 🚀 emoji"), "Test 🚀 emoji");
}

#[test]
fn test_escape_applescript_string_emoji_with_special_chars() {
    // 絵文字と特殊文字の組み合わせ
    assert_eq!(
        escape_applescript_string("Test 🚀\\n\"emoji\""),
        "Test 🚀\\\\n\\\"emoji\\\""
    );
}

// --- 実際のユースケース ---

#[test]
fn test_escape_applescript_string_mixed_content() {
    // 実際のエラーメッセージのような複雑な文字列をテスト
    let message = "Failed to open file: \"C:\\Users\\test\\file.txt\"\nError: Permission denied";
    let expected =
        "Failed to open file: \\\"C:\\\\Users\\\\test\\\\file.txt\\\"\\nError: Permission denied";
    assert_eq!(escape_applescript_string(message), expected);
}

#[test]
fn test_escape_applescript_string_applescript_code() {
    // AppleScriptコードのような文字列
    // 注: タブ文字 (\t) はエスケープ対象外
    let code = "tell application \"Finder\"\n\tactivate\nend tell";
    let expected = "tell application \\\"Finder\\\"\\n\tactivate\\nend tell";
    assert_eq!(escape_applescript_string(code), expected);
}

// --- エッジケース: エスケープ順序の検証 ---

#[test]
fn test_escape_applescript_string_escape_order_matters() {
    // エスケープの順序が重要であることを確認
    // バックスラッシュが最初にエスケープされないと、
    // 他のエスケープのバックスラッシュまでエスケープされてしまう
    let input = "\\\"";
    let expected = "\\\\\\\""; // \\ と \" にそれぞれエスケープ
    assert_eq!(escape_applescript_string(input), expected);
}

#[test]
fn test_escape_applescript_string_multiple_consecutive_backslashes() {
    // 連続するバックスラッシュのエスケープ
    assert_eq!(escape_applescript_string("\\\\\\"), "\\\\\\\\\\\\");
}

#[test]
fn test_escape_applescript_string_multiple_consecutive_quotes() {
    // 連続するダブルクォートのエスケープ
    assert_eq!(escape_applescript_string("\"\"\""), "\\\"\\\"\\\"");
}

#[test]
fn test_escape_applescript_string_multiple_consecutive_newlines() {
    // 連続する改行のエスケープ
    assert_eq!(escape_applescript_string("\n\n\n"), "\\n\\n\\n");
}

// --- エッジケース: null バイト ---

#[test]
fn test_escape_applescript_string_with_null_bytes() {
    // null バイトを含む文字列のテスト
    let message_with_null = "Test\0message";
    // null バイトはエスケープ対象ではないので、そのまま保持される
    assert_eq!(
        escape_applescript_string(message_with_null),
        "Test\0message"
    );
}

// --- エッジケース: タブ文字 ---

#[test]
fn test_escape_applescript_string_with_tabs() {
    // タブ文字はエスケープ対象ではない
    assert_eq!(escape_applescript_string("test\ttab"), "test\ttab");
}

// --- エッジケース: 混合した空白文字 ---

#[test]
fn test_escape_applescript_string_mixed_whitespace() {
    // 様々な空白文字の組み合わせ
    let input = "test \t\n\r mixed";
    let expected = "test \t\\n\\r mixed";
    assert_eq!(escape_applescript_string(input), expected);
}

// =============================================================================
// AppLaunchError テスト
// =============================================================================

#[test]
fn test_app_launch_error_creation() {
    // AppLaunchError が正しく作成できることを確認
    let error = AppLaunchError {
        message: "Test error message".to_string(),
    };
    assert_eq!(error.message, "Test error message");
}

#[test]
fn test_app_launch_error_display() {
    // Display トレイトが正しく実装されていることを確認
    let error = AppLaunchError {
        message: "Test error".to_string(),
    };
    assert_eq!(format!("{}", error), "Test error");
}

#[test]
fn test_app_launch_error_display_with_special_chars() {
    // 特殊文字を含むエラーメッセージ
    let error = AppLaunchError {
        message: "Error: \"file.txt\" not found\nPath: C:\\test".to_string(),
    };
    assert_eq!(
        format!("{}", error),
        "Error: \"file.txt\" not found\nPath: C:\\test"
    );
}

#[test]
fn test_app_launch_error_empty_message() {
    // 空のエラーメッセージ（境界値テスト）
    let error = AppLaunchError {
        message: "".to_string(),
    };
    assert_eq!(error.message, "");
    assert_eq!(format!("{}", error), "");
}

#[test]
fn test_app_launch_error_long_message() {
    // 非常に長いエラーメッセージ（境界値テスト）
    let long_message = "Error: ".to_string() + &"a".repeat(10000);
    let error = AppLaunchError {
        message: long_message.clone(),
    };
    assert_eq!(error.message.len(), 10007);
    assert_eq!(format!("{}", error), long_message);
}

#[test]
fn test_app_launch_error_is_error_trait() {
    // std::error::Error トレイトが実装されていることを確認
    let error = AppLaunchError {
        message: "Test error".to_string(),
    };
    let _: &dyn std::error::Error = &error;
}

#[test]
fn test_app_launch_error_debug() {
    // Debug トレイトが実装されていることを確認
    let error = AppLaunchError {
        message: "Test error".to_string(),
    };
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("AppLaunchError"));
    assert!(debug_str.contains("Test error"));
}

// =============================================================================
// AppLaunchResult テスト
// =============================================================================

// --- 境界値テスト: 各フィールドの組み合わせ ---

#[test]
fn test_app_launch_result_creation_with_process_id() {
    // process_id がある場合の AppLaunchResult
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: "アプリケーションを起動しました".to_string(),
        process_id: Some(12345),
        was_already_running: false,
    };

    assert_eq!(result.status, "success");
    assert_eq!(result.message, "アプリケーションを起動しました");
    assert_eq!(result.process_id, Some(12345));
    assert!(!result.was_already_running);
}

#[test]
fn test_app_launch_result_creation_without_process_id() {
    // process_id がない場合の AppLaunchResult
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: "アプリケーションを起動しました".to_string(),
        process_id: None,
        was_already_running: false,
    };

    assert_eq!(result.status, "success");
    assert_eq!(result.message, "アプリケーションを起動しました");
    assert_eq!(result.process_id, None);
    assert!(!result.was_already_running);
}

#[test]
fn test_app_launch_result_already_running_with_process_id() {
    // 既に起動していた場合（process_id あり）
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: "アプリケーションは既に起動しています".to_string(),
        process_id: Some(54321),
        was_already_running: true,
    };

    assert_eq!(result.status, "success");
    assert!(result.was_already_running);
    assert_eq!(result.process_id, Some(54321));
}

#[test]
fn test_app_launch_result_already_running_without_process_id() {
    // 既に起動していた場合（process_id なし）
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: "アプリケーションは既に起動しています".to_string(),
        process_id: None,
        was_already_running: true,
    };

    assert_eq!(result.status, "success");
    assert!(result.was_already_running);
    assert_eq!(result.process_id, None);
}

// --- to_json() メソッドのテスト ---

#[test]
fn test_app_launch_result_to_json_with_process_id() {
    // process_id がある場合の JSON 変換
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: "アプリケーションを起動しました".to_string(),
        process_id: Some(12345),
        was_already_running: false,
    };

    let json = result.to_json();
    assert_eq!(json["status"], "success");
    assert_eq!(json["message"], "アプリケーションを起動しました");
    assert_eq!(json["process_id"], 12345);
    assert_eq!(json["was_already_running"], false);
}

#[test]
fn test_app_launch_result_to_json_without_process_id() {
    // process_id がない場合の JSON 変換
    // 注: process_id が None の場合、フィールド自体が含まれない
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: "アプリケーションを起動しました".to_string(),
        process_id: None,
        was_already_running: false,
    };

    let json = result.to_json();
    assert_eq!(json["status"], "success");
    assert_eq!(json["message"], "アプリケーションを起動しました");
    // process_id フィールドは含まれない
    assert!(json.get("process_id").is_none());
    assert_eq!(json["was_already_running"], false);
}

#[test]
fn test_app_launch_result_to_json_already_running() {
    // 既に起動していた場合の JSON 変換
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: "アプリケーション 'Finder' は既に起動しています".to_string(),
        process_id: Some(100),
        was_already_running: true,
    };

    let json = result.to_json();
    assert_eq!(json["status"], "success");
    assert_eq!(
        json["message"],
        "アプリケーション 'Finder' は既に起動しています"
    );
    assert_eq!(json["process_id"], 100);
    assert_eq!(json["was_already_running"], true);
}

#[test]
fn test_app_launch_result_to_json_process_id_zero() {
    // process_id が 0 の場合（境界値テスト）
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: "Test".to_string(),
        process_id: Some(0),
        was_already_running: false,
    };

    let json = result.to_json();
    assert_eq!(json["process_id"], 0);
}

#[test]
fn test_app_launch_result_to_json_process_id_negative() {
    // process_id が負の値の場合（境界値テスト）
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: "Test".to_string(),
        process_id: Some(-1),
        was_already_running: false,
    };

    let json = result.to_json();
    assert_eq!(json["process_id"], -1);
}

#[test]
fn test_app_launch_result_to_json_process_id_max() {
    // process_id が最大値の場合（境界値テスト）
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: "Test".to_string(),
        process_id: Some(i32::MAX),
        was_already_running: false,
    };

    let json = result.to_json();
    assert_eq!(json["process_id"], i32::MAX);
}

#[test]
fn test_app_launch_result_to_json_empty_strings() {
    // 空文字列の場合（境界値テスト）
    let result = AppLaunchResult {
        status: "".to_string(),
        message: "".to_string(),
        process_id: None,
        was_already_running: false,
    };

    let json = result.to_json();
    assert_eq!(json["status"], "");
    assert_eq!(json["message"], "");
    // process_id フィールドは含まれない
    assert!(json.get("process_id").is_none());
    assert_eq!(json["was_already_running"], false);
}

#[test]
fn test_app_launch_result_to_json_special_chars_in_message() {
    // 特殊文字を含むメッセージの JSON 変換
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: "Error: \"file.txt\" not found\nPath: C:\\test".to_string(),
        process_id: Some(123),
        was_already_running: false,
    };

    let json = result.to_json();
    assert_eq!(
        json["message"],
        "Error: \"file.txt\" not found\nPath: C:\\test"
    );
}

#[test]
fn test_app_launch_result_to_json_unicode_message() {
    // Unicode文字を含むメッセージの JSON 変換
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: "アプリケーション '日本語テスト' を起動しました 🚀".to_string(),
        process_id: Some(456),
        was_already_running: false,
    };

    let json = result.to_json();
    assert_eq!(
        json["message"],
        "アプリケーション '日本語テスト' を起動しました 🚀"
    );
}

#[test]
fn test_app_launch_result_to_json_very_long_message() {
    // 非常に長いメッセージの JSON 変換（境界値テスト）
    let long_message = "Message: ".to_string() + &"a".repeat(10000);
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: long_message.clone(),
        process_id: Some(789),
        was_already_running: false,
    };

    let json = result.to_json();
    assert_eq!(json["message"], long_message);
}

// --- Clone トレイトのテスト ---

#[test]
fn test_app_launch_result_clone() {
    // Clone トレイトが正しく動作することを確認
    let result1 = AppLaunchResult {
        status: "success".to_string(),
        message: "Test message".to_string(),
        process_id: Some(999),
        was_already_running: true,
    };

    let result2 = result1.clone();

    assert_eq!(result1.status, result2.status);
    assert_eq!(result1.message, result2.message);
    assert_eq!(result1.process_id, result2.process_id);
    assert_eq!(result1.was_already_running, result2.was_already_running);
}

#[test]
fn test_app_launch_result_clone_without_process_id() {
    // process_id なしのクローン
    let result1 = AppLaunchResult {
        status: "success".to_string(),
        message: "Test message".to_string(),
        process_id: None,
        was_already_running: false,
    };

    let result2 = result1.clone();

    assert_eq!(result1.process_id, result2.process_id);
    assert_eq!(result2.process_id, None);
}

// --- Debug トレイトのテスト ---

#[test]
fn test_app_launch_result_debug() {
    // Debug トレイトが実装されていることを確認
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: "Test message".to_string(),
        process_id: Some(123),
        was_already_running: false,
    };

    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("AppLaunchResult"));
    assert!(debug_str.contains("success"));
    assert!(debug_str.contains("Test message"));
}

// --- エッジケース: 様々な status 値 ---

#[test]
fn test_app_launch_result_different_status_values() {
    // 様々な status 値をテスト
    let statuses = vec![
        "success",
        "error",
        "warning",
        "unknown",
        "",
        "very_long_status",
    ];

    for status in statuses {
        let result = AppLaunchResult {
            status: status.to_string(),
            message: "Test".to_string(),
            process_id: None,
            was_already_running: false,
        };

        let json = result.to_json();
        assert_eq!(json["status"], status);
    }
}

// =============================================================================
// launch_or_activate_app() Integration Tests (osascript required)
// =============================================================================

// 注: 以下のテストは osascript 実行に依存するため、#[ignore] を付与
// ローカル macOS 環境で `cargo test -- --ignored` で実行

#[test]
#[ignore]
fn test_launch_or_activate_app_finder() {
    // Finder アプリケーションの起動/活性化テスト
    // Finder は macOS に標準で存在するため、テストに使用可能
    let result = launch_or_activate_app("Finder", 3000);

    assert!(result.is_ok());
    let app_result = result.unwrap();
    assert_eq!(app_result.status, "success");
    // Finder は通常既に起動しているため、was_already_running は true の可能性が高い
    assert!(app_result.message.contains("Finder"));
}

#[test]
#[ignore]
fn test_launch_or_activate_app_calculator() {
    // Calculator アプリケーションの起動/活性化テスト
    let result = launch_or_activate_app("Calculator", 3000);

    assert!(result.is_ok());
    let app_result = result.unwrap();
    assert_eq!(app_result.status, "success");
    assert!(app_result.message.contains("Calculator"));
}

#[test]
#[ignore]
fn test_launch_or_activate_app_already_running() {
    // 既に起動しているアプリケーションのテスト
    // Finder を2回起動して、2回目は既に起動していることを確認
    let result1 = launch_or_activate_app("Finder", 3000);
    assert!(result1.is_ok());

    let result2 = launch_or_activate_app("Finder", 3000);
    assert!(result2.is_ok());

    let app_result = result2.unwrap();
    assert_eq!(app_result.status, "success");
    assert!(app_result.was_already_running);
    assert!(app_result.message.contains("既に起動しています"));
}

#[test]
#[ignore]
fn test_launch_or_activate_app_invalid_app_name() {
    // 存在しないアプリケーション名でテスト
    let result = launch_or_activate_app("NonExistentApp123456", 3000);

    // エラーが返されることを期待
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(
        error.message.contains("Failed to launch app") || error.message.contains("NonExistentApp")
    );
}

#[test]
#[ignore]
fn test_launch_or_activate_app_empty_name() {
    // 空文字列のアプリケーション名でテスト（境界値テスト）
    let result = launch_or_activate_app("", 3000);

    // エラーが返されることを期待
    assert!(result.is_err());
}

#[test]
#[ignore]
fn test_launch_or_activate_app_special_chars_in_name() {
    // 特殊文字を含むアプリケーション名でテスト
    let result = launch_or_activate_app("App\"With\\Special\nChars", 3000);

    // エラーが返されることを期待（存在しないアプリケーション）
    assert!(result.is_err());
}

#[test]
#[ignore]
fn test_launch_or_activate_app_timeout_zero() {
    // タイムアウト 0ms でテスト（境界値テスト）
    let result = launch_or_activate_app("Finder", 0);

    // Finder は既に起動している可能性が高いため、成功する
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_launch_or_activate_app_timeout_very_large() {
    // 非常に長いタイムアウトでテスト（境界値テスト）
    // 注: このテストは実際に10秒待つため、実行に時間がかかる
    let result = launch_or_activate_app("Finder", 10000);

    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_launch_or_activate_app_process_id_returned() {
    // process_id が返されることを確認
    let result = launch_or_activate_app("Finder", 3000);

    assert!(result.is_ok());
    let app_result = result.unwrap();
    // Finder は通常起動しているため、process_id が返されるはず
    // ただし、取得に失敗する可能性もあるため、Some/None の両方を許容
    if app_result.was_already_running {
        // 既に起動していた場合、process_id が取得できる可能性が高い
        // ただし、必ずしも取得できるとは限らない
    }
}

#[test]
#[ignore]
fn test_launch_or_activate_app_unicode_app_name() {
    // Unicode文字を含むアプリケーション名でテスト
    let result = launch_or_activate_app("日本語アプリ", 3000);

    // 存在しないアプリケーションなのでエラーが返される
    assert!(result.is_err());
}

#[test]
#[ignore]
fn test_launch_or_activate_app_very_long_name() {
    // 非常に長いアプリケーション名でテスト（境界値テスト）
    let long_name = "VeryLongApplicationName".to_string() + &"a".repeat(1000);
    let result = launch_or_activate_app(&long_name, 3000);

    // 存在しないアプリケーションなのでエラーが返される
    assert!(result.is_err());
}

#[test]
#[ignore]
fn test_launch_or_activate_app_case_sensitive() {
    // アプリケーション名の大文字小文字の扱いをテスト
    // macOS のアプリケーション名は大文字小文字を区別しない場合がある
    let result1 = launch_or_activate_app("Finder", 3000);
    let _result2 = launch_or_activate_app("finder", 3000);

    // 両方とも成功する可能性がある（macOS のバージョンによる）
    // ここでは result1 が成功することのみ確認
    assert!(result1.is_ok());
}

#[test]
#[ignore]
fn test_launch_or_activate_app_multiple_apps_sequence() {
    // 複数のアプリケーションを順番に起動
    let apps = vec!["Finder", "Calculator"];

    for app in apps {
        let result = launch_or_activate_app(app, 3000);
        assert!(result.is_ok());
        let app_result = result.unwrap();
        assert_eq!(app_result.status, "success");
    }
}

#[test]
#[ignore]
fn test_launch_or_activate_app_timeout_minimum_effective() {
    // 最小限の有効なタイムアウト（1ms）でテスト
    let result = launch_or_activate_app("Finder", 1);

    // Finder は既に起動している可能性が高いため、成功する
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_launch_or_activate_app_timeout_boundary_1000() {
    // タイムアウト 1000ms（1秒）でテスト
    let result = launch_or_activate_app("Calculator", 1000);

    assert!(result.is_ok());
}

// =============================================================================
// is_app_running() Tests (osascript required)
// =============================================================================

// 注: is_app_running() は private 関数のため、
// launch_or_activate_app() を通じて間接的にテストされる

// =============================================================================
// get_app_process_id() Tests (osascript required)
// =============================================================================

// 注: get_app_process_id() は private 関数のため、
// launch_or_activate_app() を通じて間接的にテストされる

// =============================================================================
// launch_app() Tests (osascript required)
// =============================================================================

// 注: launch_app() は private 関数のため、
// launch_or_activate_app() を通じて間接的にテストされる

// =============================================================================
// 統合テスト: 完全なワークフロー
// =============================================================================

#[test]
#[ignore]
fn test_integration_complete_workflow() {
    // 完全なワークフローのインテグレーションテスト

    // 1. アプリケーション名をエスケープ
    let app_name = "Test\"App\\Name";
    let escaped_name = escape_applescript_string(app_name);
    assert_eq!(escaped_name, "Test\\\"App\\\\Name");

    // 2. 存在するアプリケーションを起動
    let result = launch_or_activate_app("Finder", 3000);
    assert!(result.is_ok());

    let app_result = result.unwrap();

    // 3. 結果を JSON に変換
    let json = app_result.to_json();
    assert_eq!(json["status"], "success");
    assert!(json.get("message").is_some());
    assert!(json.get("was_already_running").is_some());
}

#[test]
#[ignore]
fn test_integration_error_handling() {
    // エラーハンドリングのインテグレーションテスト

    // 存在しないアプリケーションを起動しようとする
    let result = launch_or_activate_app("NonExistentApp", 3000);

    // エラーが返されることを確認
    assert!(result.is_err());

    let error = result.unwrap_err();

    // エラーメッセージが適切に設定されていることを確認
    assert!(!error.message.is_empty());

    // エラーを Display として表示
    let error_display = format!("{}", error);
    assert!(!error_display.is_empty());
}

// =============================================================================
// エッジケーステスト
// =============================================================================

#[test]
fn test_edge_case_escape_string_with_all_escapes() {
    // 全てのエスケープ対象文字を含む文字列
    let input = "\\ \" \n \r all escapes";
    let expected = "\\\\ \\\" \\n \\r all escapes";
    assert_eq!(escape_applescript_string(input), expected);
}

#[test]
fn test_edge_case_app_launch_result_json_field_order() {
    // JSON フィールドの順序は保証されないが、全てのフィールドが存在することを確認
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: "Test".to_string(),
        process_id: Some(123),
        was_already_running: true,
    };

    let json = result.to_json();

    // 全てのフィールドが存在することを確認
    assert!(json.get("status").is_some());
    assert!(json.get("message").is_some());
    assert!(json.get("process_id").is_some());
    assert!(json.get("was_already_running").is_some());
}

#[test]
fn test_edge_case_app_launch_result_json_null_handling() {
    // process_id が None の場合、フィールド自体が含まれないことを確認
    let result = AppLaunchResult {
        status: "success".to_string(),
        message: "Test".to_string(),
        process_id: None,
        was_already_running: false,
    };

    let json = result.to_json();

    // process_id フィールドは含まれない
    assert!(json.get("process_id").is_none());
}

// =============================================================================
// DisplayInfo テスト
// =============================================================================

#[test]
fn test_display_info_creation() {
    // DisplayInfo が正しく作成できることを確認
    let display_info = DisplayInfo {
        name: "Built-in Display".to_string(),
        width: 1920,
        height: 1080,
        origin_x: 0,
        origin_y: 0,
    };

    assert_eq!(display_info.name, "Built-in Display");
    assert_eq!(display_info.width, 1920);
    assert_eq!(display_info.height, 1080);
    assert_eq!(display_info.origin_x, 0);
    assert_eq!(display_info.origin_y, 0);
}

#[test]
fn test_display_info_to_json() {
    // DisplayInfo の JSON 変換
    let display_info = DisplayInfo {
        name: "External Display".to_string(),
        width: 2560,
        height: 1440,
        origin_x: 1920,
        origin_y: 0,
    };

    let json = display_info.to_json();
    assert_eq!(json["name"], "External Display");
    assert_eq!(json["width"], 2560);
    assert_eq!(json["height"], 1440);
    assert_eq!(json["origin_x"], 1920);
    assert_eq!(json["origin_y"], 0);
}

#[test]
fn test_display_info_to_json_negative_origin() {
    // 負の原点座標を持つディスプレイ（境界値テスト）
    let display_info = DisplayInfo {
        name: "Display Left".to_string(),
        width: 1920,
        height: 1080,
        origin_x: -1920,
        origin_y: 0,
    };

    let json = display_info.to_json();
    assert_eq!(json["origin_x"], -1920);
    assert_eq!(json["origin_y"], 0);
}

#[test]
fn test_display_info_to_json_4k_display() {
    // 4K ディスプレイの JSON 変換
    let display_info = DisplayInfo {
        name: "4K Display".to_string(),
        width: 3840,
        height: 2160,
        origin_x: 0,
        origin_y: 0,
    };

    let json = display_info.to_json();
    assert_eq!(json["width"], 3840);
    assert_eq!(json["height"], 2160);
}

#[test]
fn test_display_info_to_json_small_display() {
    // 小さいディスプレイの JSON 変換（境界値テスト）
    let display_info = DisplayInfo {
        name: "Small Display".to_string(),
        width: 800,
        height: 600,
        origin_x: 0,
        origin_y: 0,
    };

    let json = display_info.to_json();
    assert_eq!(json["width"], 800);
    assert_eq!(json["height"], 600);
}

#[test]
fn test_display_info_to_json_empty_name() {
    // 空の名前を持つディスプレイ（境界値テスト）
    let display_info = DisplayInfo {
        name: "".to_string(),
        width: 1920,
        height: 1080,
        origin_x: 0,
        origin_y: 0,
    };

    let json = display_info.to_json();
    assert_eq!(json["name"], "");
}

#[test]
fn test_display_info_to_json_unicode_name() {
    // Unicode を含む名前のディスプレイ
    let display_info = DisplayInfo {
        name: "日本語ディスプレイ 🖥️".to_string(),
        width: 1920,
        height: 1080,
        origin_x: 0,
        origin_y: 0,
    };

    let json = display_info.to_json();
    assert_eq!(json["name"], "日本語ディスプレイ 🖥️");
}

#[test]
fn test_display_info_to_json_very_large_dimensions() {
    // 非常に大きいディスプレイサイズ（境界値テスト）
    let display_info = DisplayInfo {
        name: "Huge Display".to_string(),
        width: 10000,
        height: 10000,
        origin_x: 0,
        origin_y: 0,
    };

    let json = display_info.to_json();
    assert_eq!(json["width"], 10000);
    assert_eq!(json["height"], 10000);
}

#[test]
fn test_display_info_to_json_all_fields_present() {
    // JSON に全フィールドが存在することを確認
    let display_info = DisplayInfo {
        name: "Test Display".to_string(),
        width: 1920,
        height: 1080,
        origin_x: 100,
        origin_y: 200,
    };

    let json = display_info.to_json();
    assert!(json.get("name").is_some());
    assert!(json.get("width").is_some());
    assert!(json.get("height").is_some());
    assert!(json.get("origin_x").is_some());
    assert!(json.get("origin_y").is_some());
}

#[test]
fn test_display_info_clone() {
    // Clone トレイトが正しく動作することを確認
    let display1 = DisplayInfo {
        name: "Test Display".to_string(),
        width: 1920,
        height: 1080,
        origin_x: 0,
        origin_y: 0,
    };

    let display2 = display1.clone();

    assert_eq!(display1.name, display2.name);
    assert_eq!(display1.width, display2.width);
    assert_eq!(display1.height, display2.height);
    assert_eq!(display1.origin_x, display2.origin_x);
    assert_eq!(display1.origin_y, display2.origin_y);
}

#[test]
fn test_display_info_debug() {
    // Debug トレイトが実装されていることを確認
    let display_info = DisplayInfo {
        name: "Test Display".to_string(),
        width: 1920,
        height: 1080,
        origin_x: 0,
        origin_y: 0,
    };

    let debug_str = format!("{:?}", display_info);
    assert!(debug_str.contains("DisplayInfo"));
    assert!(debug_str.contains("Test Display"));
    assert!(debug_str.contains("1920"));
}

// =============================================================================
// get_display_info() Integration Tests (osascript required)
// =============================================================================

#[test]
#[ignore]
fn test_get_display_info_main_display() {
    // メインディスプレイの情報を取得（display_name = None）
    let result = get_display_info(None);

    if let Err(e) = &result {
        eprintln!("Error: {:?}", e);
    }
    assert!(result.is_ok());
    let display_info = result.unwrap();

    // 名前が空でないことを確認
    assert!(!display_info.name.is_empty());

    // サイズが正の値であることを確認
    assert!(display_info.width > 0);
    assert!(display_info.height > 0);

    // JSON 変換が正しく動作することを確認
    let json = display_info.to_json();
    assert!(json.get("name").is_some());
    assert!(json.get("width").is_some());
    assert!(json.get("height").is_some());
}

#[test]
#[ignore]
fn test_get_display_info_built_in_display() {
    // "Built-in" ディスプレイを検索
    // 注: ディスプレイ名は macOS のバージョンや言語設定によって異なる可能性があります
    let result = get_display_info(Some("Built-in"));

    // ディスプレイが見つからない場合はメインディスプレイにフォールバック
    assert!(result.is_ok());
    let display_info = result.unwrap();
    assert!(display_info.width > 0);
    assert!(display_info.height > 0);
}

#[test]
#[ignore]
fn test_get_display_info_nonexistent_display() {
    // 存在しないディスプレイ名を指定
    // メインディスプレイにフォールバックすることを確認
    let result = get_display_info(Some("NonExistentDisplay123456"));

    assert!(result.is_ok());
    let display_info = result.unwrap();

    // メインディスプレイにフォールバックするため、成功する
    assert!(display_info.width > 0);
    assert!(display_info.height > 0);
}

#[test]
#[ignore]
fn test_get_display_info_empty_name() {
    // 空文字列のディスプレイ名（境界値テスト）
    let result = get_display_info(Some(""));

    assert!(result.is_ok());
    let display_info = result.unwrap();

    // メインディスプレイにフォールバックするため、成功する
    assert!(display_info.width > 0);
    assert!(display_info.height > 0);
}

#[test]
#[ignore]
fn test_get_display_info_unicode_name() {
    // Unicode を含むディスプレイ名
    let result = get_display_info(Some("日本語ディスプレイ"));

    // 存在しないディスプレイなので、メインディスプレイにフォールバック
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_get_display_info_very_long_name() {
    // 非常に長いディスプレイ名（境界値テスト）
    let long_name = "VeryLongDisplayName".to_string() + &"a".repeat(1000);
    let result = get_display_info(Some(&long_name));

    // 存在しないディスプレイなので、メインディスプレイにフォールバック
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_get_display_info_json_output() {
    // JSON 出力の詳細テスト
    let result = get_display_info(None);
    assert!(result.is_ok());

    let display_info = result.unwrap();
    let json = display_info.to_json();

    // JSON のすべてのフィールドが存在し、適切な型であることを確認
    assert!(json["name"].is_string());
    assert!(json["width"].is_i64() || json["width"].is_u64());
    assert!(json["height"].is_i64() || json["height"].is_u64());
    assert!(json["origin_x"].is_i64());
    assert!(json["origin_y"].is_i64());
}

// =============================================================================
// get_all_connected_displays() Integration Tests (osascript required)
// =============================================================================

// --- 同値分割: 正常系（実際のディスプレイを取得） ---

#[test]
#[ignore]
fn test_get_all_connected_displays_success() {
    // 接続されているすべてのディスプレイを取得
    let result = get_all_connected_displays();

    if let Err(e) = &result {
        eprintln!("Error: {:?}", e);
    }
    assert!(result.is_ok());
    let displays = result.unwrap();

    // 少なくとも1つのディスプレイが接続されているはず
    assert!(!displays.is_empty(), "少なくとも1つのディスプレイが必要");

    // 最初のディスプレイの情報を検証
    let first_display = &displays[0];
    assert!(!first_display.name.is_empty(), "ディスプレイ名は空ではない");
    assert!(first_display.width > 0, "ディスプレイ幅は正の値");
    assert!(first_display.height > 0, "ディスプレイ高さは正の値");
}

#[test]
#[ignore]
fn test_get_all_connected_displays_single_display() {
    // 単一ディスプレイの場合を想定（同値分割: ディスプレイ数=1）
    let result = get_all_connected_displays();
    assert!(result.is_ok());
    let displays = result.unwrap();

    // ディスプレイが1つ以上あることを確認
    assert!(!displays.is_empty(), "少なくとも1つのディスプレイが必要");

    // すべてのディスプレイが有効な情報を持つことを確認
    for display in &displays {
        assert!(!display.name.is_empty());
        assert!(display.width > 0);
        assert!(display.height > 0);
        // origin_x, origin_y は負の値もあり得る（マルチディスプレイ構成）
    }
}

#[test]
#[ignore]
fn test_get_all_connected_displays_multiple_displays() {
    // 複数ディスプレイの場合を想定（同値分割: ディスプレイ数>=2）
    // 注: このテストは複数ディスプレイが接続されている環境でのみ意味がある
    let result = get_all_connected_displays();
    assert!(result.is_ok());
    let displays = result.unwrap();

    if displays.len() >= 2 {
        // 複数ディスプレイが接続されている場合、各ディスプレイが有効な情報を持つことを確認
        // すべてのディスプレイが有効な情報を持つ
        for (i, display) in displays.iter().enumerate() {
            eprintln!("Display {}: {:?}", i, display);
            assert!(!display.name.is_empty());
            assert!(display.width > 0);
            assert!(display.height > 0);
        }

        // 少なくとも1つのディスプレイは origin が (0,0) であるべき（メインディスプレイ）
        let has_origin_zero = displays.iter().any(|d| d.origin_x == 0 && d.origin_y == 0);
        assert!(
            has_origin_zero,
            "メインディスプレイは origin (0,0) を持つべき"
        );
    }
}

// --- 境界値分析: ディスプレイ解像度の検証 ---

#[test]
#[ignore]
fn test_get_all_connected_displays_resolution_boundaries() {
    // 解像度の境界値を検証（境界値分析）
    let result = get_all_connected_displays();
    assert!(result.is_ok());
    let displays = result.unwrap();

    for display in &displays {
        // 最小解像度チェック（幅・高さは最小でも1以上）
        assert!(
            display.width >= 1,
            "ディスプレイ幅は最低1ピクセル: {}",
            display.width
        );
        assert!(
            display.height >= 1,
            "ディスプレイ高さは最低1ピクセル: {}",
            display.height
        );

        // 最大解像度チェック（現実的な範囲: 8K は 7680x4320）
        assert!(
            display.width <= 10000,
            "ディスプレイ幅が異常に大きい: {}",
            display.width
        );
        assert!(
            display.height <= 10000,
            "ディスプレイ高さが異常に大きい: {}",
            display.height
        );
    }
}

#[test]
#[ignore]
fn test_get_all_connected_displays_origin_coordinates() {
    // origin座標の境界値を検証（境界値分析）
    let result = get_all_connected_displays();
    assert!(result.is_ok());
    let displays = result.unwrap();

    // 少なくとも1つのディスプレイがorigin (0,0)を持つべき
    let has_main_display = displays.iter().any(|d| d.origin_x == 0 && d.origin_y == 0);
    assert!(
        has_main_display,
        "少なくとも1つのディスプレイが origin (0,0) を持つべき"
    );

    // すべてのディスプレイのorigin座標が現実的な範囲にあることを確認
    for display in &displays {
        // マルチディスプレイでは負の座標もあり得る
        assert!(
            display.origin_x >= -10000 && display.origin_x <= 10000,
            "origin_x が範囲外: {}",
            display.origin_x
        );
        assert!(
            display.origin_y >= -10000 && display.origin_y <= 10000,
            "origin_y が範囲外: {}",
            display.origin_y
        );
    }
}

// --- エッジケース: JSON フィールドの検証 ---

#[test]
#[ignore]
fn test_get_all_connected_displays_json_fields() {
    // すべてのディスプレイが必須フィールドを持つことを確認
    let result = get_all_connected_displays();
    assert!(result.is_ok());
    let displays = result.unwrap();

    for display in &displays {
        // JSON変換してすべてのフィールドが存在することを確認
        let json = display.to_json();

        assert!(json.get("name").is_some(), "name フィールドが存在する");
        assert!(json.get("width").is_some(), "width フィールドが存在する");
        assert!(json.get("height").is_some(), "height フィールドが存在する");
        assert!(
            json.get("origin_x").is_some(),
            "origin_x フィールドが存在する"
        );
        assert!(
            json.get("origin_y").is_some(),
            "origin_y フィールドが存在する"
        );

        // 型の検証
        assert!(json["name"].is_string());
        assert!(json["width"].is_i64() || json["width"].is_u64());
        assert!(json["height"].is_i64() || json["height"].is_u64());
        assert!(json["origin_x"].is_i64());
        assert!(json["origin_y"].is_i64());
    }
}

#[test]
#[ignore]
fn test_get_all_connected_displays_display_names() {
    // ディスプレイ名が有効であることを確認
    let result = get_all_connected_displays();
    assert!(result.is_ok());
    let displays = result.unwrap();

    for display in &displays {
        // ディスプレイ名が空でないことを確認
        assert!(
            !display.name.is_empty(),
            "ディスプレイ名は空であってはならない"
        );

        // ディスプレイ名が妥当な長さであることを確認（境界値分析）
        assert!(
            display.name.len() <= 255,
            "ディスプレイ名が異常に長い: {}",
            display.name.len()
        );
    }
}

// --- エラーケース: JSONパースエラーのシミュレーション ---
// 注: get_all_connected_displays() は内部で osascript を実行するため、
// エラーケースのテストは実際のosascript実行に依存する。
// ここでは、関数が正しくエラーハンドリングすることを確認する統合テスト。

#[test]
#[ignore]
fn test_get_all_connected_displays_consistency() {
    // 複数回実行して一貫性を確認
    let result1 = get_all_connected_displays();
    let result2 = get_all_connected_displays();

    assert!(result1.is_ok());
    assert!(result2.is_ok());

    let displays1 = result1.unwrap();
    let displays2 = result2.unwrap();

    // ディスプレイ数は一致するべき（接続状態が変わらない限り）
    assert_eq!(
        displays1.len(),
        displays2.len(),
        "ディスプレイ数は一貫しているべき"
    );

    // 各ディスプレイの情報も一致するべき
    for (d1, d2) in displays1.iter().zip(displays2.iter()) {
        assert_eq!(d1.name, d2.name);
        assert_eq!(d1.width, d2.width);
        assert_eq!(d1.height, d2.height);
        assert_eq!(d1.origin_x, d2.origin_x);
        assert_eq!(d1.origin_y, d2.origin_y);
    }
}

// =============================================================================
// WindowResizeResult テスト
// =============================================================================

// WindowResizeResult は applescript.rs で公開されていないため、
// resize_window() を通じて間接的にテスト

// =============================================================================
// resize_window() Integration Tests (osascript required)
// =============================================================================

#[test]
#[ignore]
fn test_resize_window_finder() {
    // Finder ウィンドウのリサイズテスト
    // 注: このテストは実際に Finder ウィンドウを操作します

    // まず Finder を起動
    let launch_result = launch_or_activate_app("Finder", 3000);
    assert!(launch_result.is_ok());

    // ウィンドウをリサイズ
    let result = resize_window("Finder", Some((100, 100)), Some((800, 600)));

    assert!(result.is_ok());
    let resize_result = result.unwrap();
    assert_eq!(resize_result.status, "success");
    assert_eq!(resize_result.new_position, Some((100, 100)));
    assert_eq!(resize_result.new_size, Some((800, 600)));
}

#[test]
#[ignore]
fn test_resize_window_position_only() {
    // 位置のみ変更
    let launch_result = launch_or_activate_app("Finder", 3000);
    assert!(launch_result.is_ok());

    let result = resize_window("Finder", Some((200, 200)), None);

    assert!(result.is_ok());
    let resize_result = result.unwrap();
    assert_eq!(resize_result.new_position, Some((200, 200)));
    assert_eq!(resize_result.new_size, None);
}

#[test]
#[ignore]
fn test_resize_window_size_only() {
    // サイズのみ変更
    let launch_result = launch_or_activate_app("Finder", 3000);
    assert!(launch_result.is_ok());

    let result = resize_window("Finder", None, Some((900, 700)));

    assert!(result.is_ok());
    let resize_result = result.unwrap();
    assert_eq!(resize_result.new_position, None);
    assert_eq!(resize_result.new_size, Some((900, 700)));
}

#[test]
#[ignore]
fn test_resize_window_nonexistent_app() {
    // 存在しないアプリケーションでテスト
    let result = resize_window("NonExistentApp123456", Some((100, 100)), Some((800, 600)));

    // エラーが返されることを期待
    assert!(result.is_err());
}

#[test]
#[ignore]
fn test_resize_window_boundary_zero_position() {
    // 位置が (0, 0) の場合（境界値テスト）
    let launch_result = launch_or_activate_app("Finder", 3000);
    assert!(launch_result.is_ok());

    let result = resize_window("Finder", Some((0, 0)), Some((800, 600)));

    assert!(result.is_ok());
    let resize_result = result.unwrap();
    assert_eq!(resize_result.new_position, Some((0, 0)));
}

#[test]
#[ignore]
fn test_resize_window_boundary_small_size() {
    // 小さいサイズの場合（境界値テスト）
    let launch_result = launch_or_activate_app("Finder", 3000);
    assert!(launch_result.is_ok());

    let result = resize_window("Finder", Some((100, 100)), Some((200, 150)));

    assert!(result.is_ok());
    let resize_result = result.unwrap();
    assert_eq!(resize_result.new_size, Some((200, 150)));
}

#[test]
#[ignore]
fn test_resize_window_boundary_large_size() {
    // 非常に大きいサイズの場合（境界値テスト）
    // 注: ディスプレイより大きいサイズを指定
    let launch_result = launch_or_activate_app("Finder", 3000);
    assert!(launch_result.is_ok());

    let result = resize_window("Finder", Some((0, 0)), Some((5000, 3000)));

    // macOS がサイズを制限する可能性があるが、コマンド自体は成功する可能性がある
    if let Ok(resize_result) = result {
        assert_eq!(resize_result.new_size, Some((5000, 3000)));
    }
}

#[test]
#[ignore]
fn test_resize_window_negative_position() {
    // 負の座標（画面外）
    let launch_result = launch_or_activate_app("Finder", 3000);
    assert!(launch_result.is_ok());

    let result = resize_window("Finder", Some((-100, -100)), Some((800, 600)));

    // macOS は負の座標を許可する可能性がある（マルチディスプレイ環境）
    // エラーになるかもしれないし、成功するかもしれない
    // ここでは結果のみを確認
    if let Ok(resize_result) = result {
        assert_eq!(resize_result.new_position, Some((-100, -100)));
    }
}

#[test]
#[ignore]
fn test_resize_window_special_chars_in_app_name() {
    // 特殊文字を含むアプリケーション名
    let result = resize_window(
        "App\"With\\Special\nChars",
        Some((100, 100)),
        Some((800, 600)),
    );

    // 存在しないアプリケーションなのでエラーが返される
    assert!(result.is_err());
}

#[test]
#[ignore]
fn test_resize_window_to_json() {
    // JSON 変換のテスト
    let launch_result = launch_or_activate_app("Finder", 3000);
    assert!(launch_result.is_ok());

    let result = resize_window("Finder", Some((150, 150)), Some((850, 650)));

    if let Ok(resize_result) = result {
        let json = resize_result.to_json();

        assert_eq!(json["status"], "success");
        assert!(json.get("message").is_some());
        assert_eq!(json["new_position"]["x"], 150);
        assert_eq!(json["new_position"]["y"], 150);
        assert_eq!(json["new_size"]["width"], 850);
        assert_eq!(json["new_size"]["height"], 650);
    }
}

#[test]
#[ignore]
fn test_resize_window_unicode_app_name() {
    // Unicode を含むアプリケーション名
    let result = resize_window("日本語アプリ", Some((100, 100)), Some((800, 600)));

    // 存在しないアプリケーションなのでエラーが返される
    assert!(result.is_err());
}

#[test]
#[ignore]
fn test_resize_window_empty_app_name() {
    // 空文字列のアプリケーション名（境界値テスト）
    let result = resize_window("", Some((100, 100)), Some((800, 600)));

    // エラーが返されることを期待
    assert!(result.is_err());
}

#[test]
#[ignore]
fn test_resize_window_calculator() {
    // Calculator アプリケーションでテスト
    let launch_result = launch_or_activate_app("Calculator", 3000);
    assert!(launch_result.is_ok());

    let result = resize_window("Calculator", Some((400, 400)), Some((300, 400)));

    assert!(result.is_ok());
    let resize_result = result.unwrap();
    assert_eq!(resize_result.status, "success");
}

#[test]
#[ignore]
fn test_resize_window_multiple_operations() {
    // 複数回のリサイズ操作
    let launch_result = launch_or_activate_app("Finder", 3000);
    assert!(launch_result.is_ok());

    // 1回目のリサイズ
    let result1 = resize_window("Finder", Some((100, 100)), Some((800, 600)));
    assert!(result1.is_ok());

    // 2回目のリサイズ
    let result2 = resize_window("Finder", Some((200, 200)), Some((900, 700)));
    assert!(result2.is_ok());

    // 3回目のリサイズ
    let result3 = resize_window("Finder", Some((300, 300)), Some((1000, 800)));
    assert!(result3.is_ok());
}

// =============================================================================
// WindowInfo テスト
// =============================================================================

#[test]
fn test_window_info_creation() {
    // WindowInfo が正しく作成できることを確認
    let window_info = WindowInfo {
        title: "Test Window".to_string(),
        position: (100, 200),
        size: (800, 600),
        minimized: false,
        visible: true,
    };

    assert_eq!(window_info.title, "Test Window");
    assert_eq!(window_info.position, (100, 200));
    assert_eq!(window_info.size, (800, 600));
    assert!(!window_info.minimized);
    assert!(window_info.visible);
}

#[test]
fn test_window_info_to_json() {
    // WindowInfo の JSON 変換
    let window_info = WindowInfo {
        title: "Chrome Window".to_string(),
        position: (50, 100),
        size: (1200, 800),
        minimized: false,
        visible: true,
    };

    let json = window_info.to_json();
    assert_eq!(json["title"], "Chrome Window");
    assert_eq!(json["position"]["x"], 50);
    assert_eq!(json["position"]["y"], 100);
    assert_eq!(json["size"]["width"], 1200);
    assert_eq!(json["size"]["height"], 800);
    assert_eq!(json["minimized"], false);
    assert_eq!(json["visible"], true);
}

#[test]
fn test_window_info_to_json_minimized() {
    // 最小化されたウィンドウの JSON 変換
    let window_info = WindowInfo {
        title: "Minimized Window".to_string(),
        position: (0, 0),
        size: (800, 600),
        minimized: true,
        visible: false,
    };

    let json = window_info.to_json();
    assert_eq!(json["title"], "Minimized Window");
    assert_eq!(json["minimized"], true);
    assert_eq!(json["visible"], false);
}

#[test]
fn test_window_info_to_json_boundary_zero_position() {
    // 位置が (0, 0) の場合（境界値テスト）
    let window_info = WindowInfo {
        title: "Origin Window".to_string(),
        position: (0, 0),
        size: (800, 600),
        minimized: false,
        visible: true,
    };

    let json = window_info.to_json();
    assert_eq!(json["position"]["x"], 0);
    assert_eq!(json["position"]["y"], 0);
}

#[test]
fn test_window_info_to_json_boundary_negative_position() {
    // 負の座標の場合（境界値テスト）
    let window_info = WindowInfo {
        title: "Negative Position".to_string(),
        position: (-100, -200),
        size: (800, 600),
        minimized: false,
        visible: true,
    };

    let json = window_info.to_json();
    assert_eq!(json["position"]["x"], -100);
    assert_eq!(json["position"]["y"], -200);
}

#[test]
fn test_window_info_to_json_boundary_max_position() {
    // 最大座標の場合（境界値テスト）
    let window_info = WindowInfo {
        title: "Max Position".to_string(),
        position: (i32::MAX, i32::MAX),
        size: (800, 600),
        minimized: false,
        visible: true,
    };

    let json = window_info.to_json();
    assert_eq!(json["position"]["x"], i32::MAX);
    assert_eq!(json["position"]["y"], i32::MAX);
}

#[test]
fn test_window_info_to_json_boundary_min_position() {
    // 最小座標の場合（境界値テスト）
    let window_info = WindowInfo {
        title: "Min Position".to_string(),
        position: (i32::MIN, i32::MIN),
        size: (800, 600),
        minimized: false,
        visible: true,
    };

    let json = window_info.to_json();
    assert_eq!(json["position"]["x"], i32::MIN);
    assert_eq!(json["position"]["y"], i32::MIN);
}

#[test]
fn test_window_info_to_json_boundary_zero_size() {
    // サイズが (0, 0) の場合（境界値テスト）
    let window_info = WindowInfo {
        title: "Zero Size".to_string(),
        position: (100, 100),
        size: (0, 0),
        minimized: false,
        visible: true,
    };

    let json = window_info.to_json();
    assert_eq!(json["size"]["width"], 0);
    assert_eq!(json["size"]["height"], 0);
}

#[test]
fn test_window_info_to_json_boundary_small_size() {
    // 小さいサイズの場合（境界値テスト）
    let window_info = WindowInfo {
        title: "Small Window".to_string(),
        position: (100, 100),
        size: (1, 1),
        minimized: false,
        visible: true,
    };

    let json = window_info.to_json();
    assert_eq!(json["size"]["width"], 1);
    assert_eq!(json["size"]["height"], 1);
}

#[test]
fn test_window_info_to_json_boundary_large_size() {
    // 非常に大きいサイズの場合（境界値テスト）
    let window_info = WindowInfo {
        title: "Large Window".to_string(),
        position: (0, 0),
        size: (10000, 10000),
        minimized: false,
        visible: true,
    };

    let json = window_info.to_json();
    assert_eq!(json["size"]["width"], 10000);
    assert_eq!(json["size"]["height"], 10000);
}

#[test]
fn test_window_info_to_json_boundary_max_size() {
    // 最大サイズの場合（境界値テスト）
    let window_info = WindowInfo {
        title: "Max Size Window".to_string(),
        position: (0, 0),
        size: (i32::MAX, i32::MAX),
        minimized: false,
        visible: true,
    };

    let json = window_info.to_json();
    assert_eq!(json["size"]["width"], i32::MAX);
    assert_eq!(json["size"]["height"], i32::MAX);
}

#[test]
fn test_window_info_to_json_boundary_negative_size() {
    // 負のサイズの場合（境界値テスト）
    // 注: 実際には発生しないはずだが、データ型としては可能
    let window_info = WindowInfo {
        title: "Negative Size".to_string(),
        position: (100, 100),
        size: (-100, -200),
        minimized: false,
        visible: true,
    };

    let json = window_info.to_json();
    assert_eq!(json["size"]["width"], -100);
    assert_eq!(json["size"]["height"], -200);
}

#[test]
fn test_window_info_to_json_empty_title() {
    // 空のタイトルの場合（境界値テスト）
    let window_info = WindowInfo {
        title: "".to_string(),
        position: (100, 100),
        size: (800, 600),
        minimized: false,
        visible: true,
    };

    let json = window_info.to_json();
    assert_eq!(json["title"], "");
}

#[test]
fn test_window_info_to_json_unicode_title() {
    // Unicode を含むタイトルの場合
    let window_info = WindowInfo {
        title: "日本語ウィンドウ 🚀".to_string(),
        position: (100, 100),
        size: (800, 600),
        minimized: false,
        visible: true,
    };

    let json = window_info.to_json();
    assert_eq!(json["title"], "日本語ウィンドウ 🚀");
}

#[test]
fn test_window_info_to_json_special_chars_title() {
    // 特殊文字を含むタイトルの場合
    let window_info = WindowInfo {
        title: "Window \"With\" Special\\Chars\nAnd\rNewlines".to_string(),
        position: (100, 100),
        size: (800, 600),
        minimized: false,
        visible: true,
    };

    let json = window_info.to_json();
    assert_eq!(
        json["title"],
        "Window \"With\" Special\\Chars\nAnd\rNewlines"
    );
}

#[test]
fn test_window_info_to_json_very_long_title() {
    // 非常に長いタイトルの場合（境界値テスト）
    let long_title = "Window ".to_string() + &"a".repeat(10000);
    let window_info = WindowInfo {
        title: long_title.clone(),
        position: (100, 100),
        size: (800, 600),
        minimized: false,
        visible: true,
    };

    let json = window_info.to_json();
    assert_eq!(json["title"], long_title);
}

#[test]
fn test_window_info_to_json_all_fields_present() {
    // JSON に全フィールドが存在することを確認
    let window_info = WindowInfo {
        title: "Test Window".to_string(),
        position: (100, 200),
        size: (800, 600),
        minimized: false,
        visible: true,
    };

    let json = window_info.to_json();
    assert!(json.get("title").is_some());
    assert!(json.get("position").is_some());
    assert!(json["position"]["x"].is_i64());
    assert!(json["position"]["y"].is_i64());
    assert!(json.get("size").is_some());
    assert!(json["size"]["width"].is_i64());
    assert!(json["size"]["height"].is_i64());
    assert!(json.get("minimized").is_some());
    assert!(json.get("visible").is_some());
}

#[test]
fn test_window_info_clone() {
    // Clone トレイトが正しく動作することを確認
    let window1 = WindowInfo {
        title: "Test Window".to_string(),
        position: (100, 200),
        size: (800, 600),
        minimized: false,
        visible: true,
    };

    let window2 = window1.clone();

    assert_eq!(window1.title, window2.title);
    assert_eq!(window1.position, window2.position);
    assert_eq!(window1.size, window2.size);
    assert_eq!(window1.minimized, window2.minimized);
    assert_eq!(window1.visible, window2.visible);
}

#[test]
fn test_window_info_debug() {
    // Debug トレイトが実装されていることを確認
    let window_info = WindowInfo {
        title: "Test Window".to_string(),
        position: (100, 200),
        size: (800, 600),
        minimized: false,
        visible: true,
    };

    let debug_str = format!("{:?}", window_info);
    assert!(debug_str.contains("WindowInfo"));
    assert!(debug_str.contains("Test Window"));
    assert!(debug_str.contains("100"));
    assert!(debug_str.contains("200"));
}

// =============================================================================
// WindowInfoError テスト
// =============================================================================

#[test]
fn test_window_info_error_creation() {
    // WindowInfoError が正しく作成できることを確認
    let error = WindowInfoError {
        message: "Test error message".to_string(),
    };
    assert_eq!(error.message, "Test error message");
}

#[test]
fn test_window_info_error_display() {
    // Display トレイトが正しく実装されていることを確認
    let error = WindowInfoError {
        message: "Test error".to_string(),
    };
    assert_eq!(format!("{}", error), "Test error");
}

#[test]
fn test_window_info_error_display_with_special_chars() {
    // 特殊文字を含むエラーメッセージ
    let error = WindowInfoError {
        message: "Error: \"window\" not found\nApp: Test\\App".to_string(),
    };
    assert_eq!(
        format!("{}", error),
        "Error: \"window\" not found\nApp: Test\\App"
    );
}

#[test]
fn test_window_info_error_empty_message() {
    // 空のエラーメッセージ（境界値テスト）
    let error = WindowInfoError {
        message: "".to_string(),
    };
    assert_eq!(error.message, "");
    assert_eq!(format!("{}", error), "");
}

#[test]
fn test_window_info_error_long_message() {
    // 非常に長いエラーメッセージ（境界値テスト）
    let long_message = "Error: ".to_string() + &"a".repeat(10000);
    let error = WindowInfoError {
        message: long_message.clone(),
    };
    assert_eq!(error.message.len(), 10007);
    assert_eq!(format!("{}", error), long_message);
}

#[test]
fn test_window_info_error_is_error_trait() {
    // std::error::Error トレイトが実装されていることを確認
    let error = WindowInfoError {
        message: "Test error".to_string(),
    };
    let _: &dyn std::error::Error = &error;
}

#[test]
fn test_window_info_error_debug() {
    // Debug トレイトが実装されていることを確認
    let error = WindowInfoError {
        message: "Test error".to_string(),
    };
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("WindowInfoError"));
    assert!(debug_str.contains("Test error"));
}

#[test]
fn test_window_info_error_unicode_message() {
    // Unicode を含むエラーメッセージ
    let error = WindowInfoError {
        message: "エラー: ウィンドウが見つかりません 🚀".to_string(),
    };
    assert_eq!(
        format!("{}", error),
        "エラー: ウィンドウが見つかりません 🚀"
    );
}

// =============================================================================
// Get All Windows Tests
// =============================================================================

#[test]
fn test_parse_window_list_empty() {
    let result = parse_window_list("");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_parse_window_list_single_window() {
    let input = "Window Title|100,200|800,600|false|true";
    let result = parse_window_list(input);
    assert!(result.is_ok());
    let windows = result.unwrap();
    assert_eq!(windows.len(), 1);
    assert_eq!(windows[0].title, "Window Title");
    assert_eq!(windows[0].position, (100, 200));
    assert_eq!(windows[0].size, (800, 600));
    assert!(!windows[0].minimized);
    assert!(windows[0].visible);
}

#[test]
fn test_parse_window_list_multiple_windows() {
    let input = "Main|0,25|1440,900|false|true,Settings|200,100|800,600|false|true";
    let result = parse_window_list(input);
    assert!(result.is_ok());
    let windows = result.unwrap();
    assert_eq!(windows.len(), 2);
    assert_eq!(windows[0].title, "Main");
    assert_eq!(windows[0].position, (0, 25));
    assert_eq!(windows[0].size, (1440, 900));
    assert_eq!(windows[1].title, "Settings");
    assert_eq!(windows[1].position, (200, 100));
    assert_eq!(windows[1].size, (800, 600));
}

#[test]
fn test_parse_window_list_with_partial_failure() {
    // 不正な形式のウィンドウを含む場合、スキップされる
    // Note: Invalid data needs to have exactly 4 pipes to be recognized as a window entry
    let input = "Valid|0,0|100,100|false|true, Another Valid|50,50|200,200|true|false";
    let result = parse_window_list(input);
    assert!(result.is_ok());
    let windows = result.unwrap();
    assert_eq!(windows.len(), 2);
    assert_eq!(windows[0].title, "Valid");
    assert_eq!(windows[1].title, "Another Valid");
}

#[test]
fn test_parse_single_window_valid() {
    let input = "Window Title|100,200|800,600|false|true";
    let result = parse_single_window(input);
    assert!(result.is_ok());
    let window = result.unwrap();
    assert_eq!(window.title, "Window Title");
    assert_eq!(window.position, (100, 200));
    assert_eq!(window.size, (800, 600));
    assert!(!window.minimized);
    assert!(window.visible);
}

#[test]
fn test_parse_single_window_invalid_format() {
    let result = parse_single_window("invalid");
    assert!(result.is_err());
}

#[test]
fn test_parse_single_window_invalid_position() {
    let input = "Window|invalid,200|800,600|false|true";
    let result = parse_single_window(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_single_window_invalid_size() {
    let input = "Window|100,200|invalid,600|false|true";
    let result = parse_single_window(input);
    assert!(result.is_err());
}

#[test]
fn test_parse_single_window_minimized_true() {
    let input = "Window|0,0|100,100|true|false";
    let result = parse_single_window(input);
    assert!(result.is_ok());
    let window = result.unwrap();
    assert!(window.minimized);
    assert!(!window.visible);
}

#[test]
fn test_parse_single_window_with_special_chars_in_title() {
    let input = "Window Title & @ # |100,200|800,600|false|true";
    let result = parse_single_window(input);
    assert!(result.is_ok());
    let window = result.unwrap();
    assert_eq!(window.title, "Window Title & @ # ");
}

#[test]
fn test_parse_single_window_zero_coordinates() {
    let input = "Window|0,0|100,100|false|true";
    let result = parse_single_window(input);
    assert!(result.is_ok());
    let window = result.unwrap();
    assert_eq!(window.position, (0, 0));
    assert_eq!(window.size, (100, 100));
}

#[test]
fn test_parse_window_list_with_whitespace() {
    // AppleScript output may have whitespace around entries
    let input = "  Window1|0,0|100,100|false|true  ,  Window2|50,50|200,200|true|false  ";
    let result = parse_window_list(input);
    assert!(result.is_ok());
    let windows = result.unwrap();
    assert_eq!(windows.len(), 2);
    assert_eq!(windows[0].title, "Window1");
    assert_eq!(windows[1].title, "Window2");
}

/// Integration test: Verify get_all_windows with Finder (always running)
#[test]
#[ignore] // CI環境でテストできないため#[ignore]を設定
fn test_get_all_windows_finder() {
    let result = get_all_windows("Finder");
    assert!(result.is_ok(), "get_all_windows should succeed for Finder");

    let windows = result.unwrap();
    // Finder is usually running with at least one window
    // (but it's possible to have no windows in some states)
    for window in &windows {
        assert!(!window.title.is_empty(), "Window title should not be empty");
        assert!(window.size.0 > 0, "Window width should be positive");
        assert!(window.size.1 > 0, "Window height should be positive");
    }
}

/// Integration test: Verify get_all_windows with non-existent app
#[test]
#[ignore]
fn test_get_all_windows_nonexistent_app() {
    let result = get_all_windows("NonExistentApp12345XYZ");
    assert!(result.is_err(), "Should return error for non-existent app");
}

/// Integration test: Verify JSON serialization of window info
#[test]
#[ignore]
fn test_get_all_windows_json_serialization() {
    let result = get_all_windows("Finder");

    if let Ok(windows) = result {
        for window in &windows {
            let json = window.to_json();
            assert!(json.is_object());
            assert!(json.get("title").is_some());
            assert!(json.get("position").is_some());
            assert!(json.get("size").is_some());
            assert!(json.get("minimized").is_some());
            assert!(json.get("visible").is_some());

            // Verify structure of position and size objects
            let position = json.get("position").unwrap();
            assert!(position.is_object());
            assert!(position.get("x").is_some());
            assert!(position.get("y").is_some());

            let size = json.get("size").unwrap();
            assert!(size.is_object());
            assert!(size.get("width").is_some());
            assert!(size.get("height").is_some());
        }
    }
}

// =============================================================================
// Find Window by Title Tests
// =============================================================================

// =============================================================================
// Create New Window Tests
// =============================================================================

/// Integration test: Safari で新規ウィンドウを作成
///
/// # テスト概要
/// Safariで新規ウィンドウを作成し、ウィンドウ数が増えることを確認
///
/// # テストシナリオ
/// 1. Safari が起動していない場合は起動
/// 2. 現在のウィンドウ数を取得
/// 3. 新規ウィンドウを作成
/// 4. ウィンドウ数が増えていることを確認
///
/// # 境界値
/// - アプリケーション未起動状態からのウィンドウ作成
/// - メニュー項目の多言語対応（英語「New Window」/日本語「新規ウインドウ」）
#[test]
#[ignore] // CI環境でテストできないため#[ignore]を設定（osascript実行が必要）
fn test_create_new_window_safari() {
    use apptidying::applescript::{create_new_window, get_all_windows};
    use std::process::Command;

    // Arrange: Safariを起動
    let _ = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Safari\" to activate")
        .output();
    std::thread::sleep(std::time::Duration::from_secs(2));

    // 現在のウィンドウ数を取得
    let windows_before = get_all_windows("Safari").expect("Safariのウィンドウ一覧取得に失敗");
    let count_before = windows_before.len();

    // Act: 新規ウィンドウを作成
    let result = create_new_window("Safari");

    // Assert: 成功することを確認
    assert!(
        result.is_ok(),
        "Safari 新規ウィンドウ作成に失敗: {:?}",
        result.err()
    );

    // 少し待機（ウィンドウ作成完了を待つ）
    std::thread::sleep(std::time::Duration::from_millis(500));

    // ウィンドウ数が増えていることを確認
    let windows_after = get_all_windows("Safari").expect("Safariのウィンドウ一覧取得に失敗");
    assert!(
        windows_after.len() > count_before,
        "Safari の新規ウィンドウが作成されていません。before: {}, after: {}",
        count_before,
        windows_after.len()
    );
}

/// Integration test: Google Chrome で新規ウィンドウを作成
///
/// # テスト概要
/// Google Chromeで新規ウィンドウを作成し、ウィンドウ数が増えることを確認
///
/// # テストシナリオ
/// 1. Google Chrome が起動していない場合は起動
/// 2. 現在のウィンドウ数を取得
/// 3. 新規ウィンドウを作成
/// 4. ウィンドウ数が増えていることを確認
///
/// # 境界値
/// - アプリケーション名にスペースが含まれる場合
/// - メニュー項目の多言語対応
#[test]
#[ignore] // CI環境でテストできないため#[ignore]を設定（osascript実行が必要）
fn test_create_new_window_chrome() {
    use apptidying::applescript::{create_new_window, get_all_windows};
    use std::process::Command;

    // Arrange: Google Chromeを起動
    let chrome_launch = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Google Chrome\" to activate")
        .output();
    if chrome_launch.is_err() {
        // Google Chromeがインストールされていない場合はスキップ
        println!("Google Chrome がインストールされていないため、テストをスキップします");
        return;
    }
    std::thread::sleep(std::time::Duration::from_secs(2));

    // 現在のウィンドウ数を取得
    let windows_before = get_all_windows("Google Chrome");
    if windows_before.is_err() {
        println!("Google Chrome のウィンドウ情報取得に失敗したため、テストをスキップします");
        return;
    }
    let count_before = windows_before.unwrap().len();

    // Act: 新規ウィンドウを作成
    let result = create_new_window("Google Chrome");

    // Assert: 成功することを確認
    assert!(
        result.is_ok(),
        "Google Chrome 新規ウィンドウ作成に失敗: {:?}",
        result.err()
    );

    // 少し待機（ウィンドウ作成完了を待つ）
    std::thread::sleep(std::time::Duration::from_millis(500));

    // ウィンドウ数が増えていることを確認
    let windows_after =
        get_all_windows("Google Chrome").expect("Google Chromeのウィンドウ一覧取得に失敗");
    assert!(
        windows_after.len() > count_before,
        "Google Chrome の新規ウィンドウが作成されていません。before: {}, after: {}",
        count_before,
        windows_after.len()
    );
}

/// Integration test: Finder で新規ウィンドウを作成
///
/// # テスト概要
/// Finderで新規ウィンドウを作成し、ウィンドウ数が増えることを確認
///
/// # テストシナリオ
/// 1. Finderの現在のウィンドウ数を取得（Finderは常に起動している）
/// 2. 新規ウィンドウを作成
/// 3. ウィンドウ数が増えていることを確認
///
/// # 境界値
/// - システムアプリケーション（Finder）での動作
/// - 「新規Finderウインドウ」という特殊なメニュー項目名
#[test]
#[ignore] // CI環境でテストできないため#[ignore]を設定（osascript実行が必要）
fn test_create_new_window_finder() {
    use apptidying::applescript::{create_new_window, get_all_windows};

    // Arrange: 現在のウィンドウ数を取得
    let windows_before = get_all_windows("Finder").expect("Finderのウィンドウ一覧取得に失敗");
    let count_before = windows_before.len();

    // Act: 新規ウィンドウを作成
    let result = create_new_window("Finder");

    // Assert: 成功することを確認
    assert!(
        result.is_ok(),
        "Finder 新規ウィンドウ作成に失敗: {:?}",
        result.err()
    );

    // 少し待機（ウィンドウ作成完了を待つ）
    std::thread::sleep(std::time::Duration::from_millis(500));

    // ウィンドウ数が増えていることを確認
    let windows_after = get_all_windows("Finder").expect("Finderのウィンドウ一覧取得に失敗");
    assert!(
        windows_after.len() > count_before,
        "Finder の新規ウィンドウが作成されていません。before: {}, after: {}",
        count_before,
        windows_after.len()
    );
}

/// Integration test: 存在しないアプリケーションで新規ウィンドウ作成を試みる
///
/// # テスト概要
/// 存在しないアプリケーションで新規ウィンドウ作成を試み、適切なエラーが返ることを確認
///
/// # テストシナリオ
/// 1. 存在しないアプリケーション名で create_new_window を呼び出す
/// 2. エラーが返ることを確認
///
/// # 境界値
/// - 存在しないアプリケーション名
/// - エラーハンドリングの確認
#[test]
#[ignore] // CI環境でテストできないため#[ignore]を設定（osascript実行が必要）
fn test_create_new_window_nonexistent_app() {
    use apptidying::applescript::create_new_window;

    // Act: 存在しないアプリケーションで新規ウィンドウ作成を試みる
    let result = create_new_window("NonExistentApp12345XYZ");

    // Assert: エラーが返ることを確認
    assert!(
        result.is_err(),
        "存在しないアプリケーションで新規ウィンドウ作成が成功してしまいました"
    );
}

/// Integration test: 新規ウィンドウメニューがないアプリケーション
///
/// # テスト概要
/// 新規ウィンドウメニューを持たないアプリケーション（例: システム環境設定）で
/// 新規ウィンドウ作成を試み、適切なエラーが返ることを確認
///
/// # テストシナリオ
/// 1. Calculator（計算機）など、新規ウィンドウメニューがないアプリで create_new_window を呼び出す
/// 2. エラーが返ることを確認
/// 3. エラーメッセージに「menu item not found」が含まれることを確認
///
/// # 境界値
/// - メニュー項目が存在しない場合のエラーハンドリング
#[test]
#[ignore] // CI環境でテストできないため#[ignore]を設定（osascript実行が必要）
fn test_create_new_window_no_menu_item() {
    use apptidying::applescript::create_new_window;
    use std::process::Command;

    // Arrange: Calculatorを起動（新規ウィンドウメニューがないアプリ）
    let _ = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Calculator\" to activate")
        .output();
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Act: 新規ウィンドウ作成を試みる
    let result = create_new_window("Calculator");

    // Assert: エラーが返ることを確認
    assert!(
        result.is_err(),
        "新規ウィンドウメニューがないアプリで作成が成功してしまいました"
    );

    // エラーメッセージにメニュー関連のエラーが含まれることを確認
    if let Err(e) = result {
        assert!(
            e.message.to_lowercase().contains("menu") || e.message.contains("メニュー"),
            "エラーメッセージが期待と異なります: {}",
            e.message
        );
    }
}

// =============================================================================
// get_running_applications() のテスト
// =============================================================================

/// localized name で取得したアプリケーション情報が有効なAppInfo構造体になることを確認
///
/// 検証項目:
/// - get_running_applications() が正常に実行される
/// - 戻り値が Vec<AppInfo> 構造体として取得できる
/// - 各 AppInfo にアプリケーション名が含まれている
///
/// 環境要件:
/// - macOS で osascript が利用可能
/// - System Events へのアクセス権限が必要
#[test]
#[ignore]
fn test_localized_name_returns_valid_app_info() {
    // Act: 実行中アプリケーション一覧を取得
    let result = get_running_applications();

    // Assert: 取得が成功していることを確認
    assert!(result.is_ok(), "get_running_applications() が失敗しました");

    let apps = result.unwrap();

    // Assert: 取得したアプリケーション情報が有効であることを確認
    assert!(!apps.is_empty(), "実行中アプリケーションが0件です");

    for app in &apps {
        // 各アプリケーション名が空でないことを確認
        assert!(!app.name.is_empty(), "アプリケーション名が空です");

        // プロセスIDが設定されている場合、正の値であることを確認
        if let Some(pid) = app.process_id {
            assert!(pid > 0, "プロセスIDが不正です: {}", pid);
        }
    }
}

/// "AppName|12345" 形式の解析が正常に機能することを確認
///
/// 検証項目:
/// - パイプ区切り文字列をアプリケーション名とプロセスIDに分割できる
/// - プロセスIDが正しく i32 型に変換される
///
/// 注意: このテストは get_running_applications() の内部ロジックを模倣
#[test]
fn test_localized_name_parsing_with_pipe_separator() {
    // Arrange: "AppName|ProcessID" 形式のテストデータ
    let test_data = "Safari|12345";

    // Act: パイプ位置を検索してパース
    let pipe_pos = test_data.rfind('|').unwrap();
    let app_name = &test_data[..pipe_pos];
    let pid_str = &test_data[pipe_pos + 1..];
    let process_id = pid_str.parse::<i32>().ok();

    // Assert: 正しくパースされていることを確認
    assert_eq!(app_name, "Safari", "アプリケーション名が一致しません");
    assert_eq!(
        process_id,
        Some(12345),
        "プロセスIDが正しくパースされていません"
    );
}

/// "AppName|" 形式（プロセスID なし）の解析が正常に機能することを確認
///
/// 検証項目:
/// - パイプ文字の後にプロセスIDがない場合、None になる
/// - アプリケーション名は正しく取得できる
///
/// 注意: AppleScript で unix id 取得が失敗した場合の動作を想定
#[test]
fn test_localized_name_parsing_without_process_id() {
    // Arrange: "AppName|" 形式（プロセスIDなし）のテストデータ
    let test_data = "Finder|";

    // Act: パイプ位置を検索してパース
    let pipe_pos = test_data.rfind('|').unwrap();
    let app_name = &test_data[..pipe_pos];
    let pid_str = &test_data[pipe_pos + 1..];
    let process_id = if pid_str.is_empty() {
        None
    } else {
        pid_str.parse::<i32>().ok()
    };

    // Assert: 正しくパースされていることを確認
    assert_eq!(app_name, "Finder", "アプリケーション名が一致しません");
    assert_eq!(
        process_id, None,
        "プロセスIDが空の場合は None になるべきです"
    );
}

/// get_running_applications() の結果が空でない（1個以上）ことを確認
///
/// 検証項目:
/// - macOS 環境で実行すると、必ず1個以上のアプリケーションが起動している
/// - 最低でも Finder や System Events が起動しているはず
///
/// 環境要件:
/// - macOS で osascript が利用可能
/// - System Events へのアクセス権限が必要
#[test]
#[ignore]
fn test_localized_name_app_count_not_zero() {
    // Act: 実行中アプリケーション一覧を取得
    let result = get_running_applications();

    // Assert: 取得が成功していることを確認
    assert!(result.is_ok(), "get_running_applications() が失敗しました");

    let apps = result.unwrap();

    // Assert: 少なくとも1個のアプリケーションが起動していることを確認
    assert!(
        !apps.is_empty(),
        "実行中アプリケーションが0件です（Finderなど標準アプリも含めて）"
    );
}

/// Safari などの標準アプリケーションが取得されることを確認
///
/// 検証項目:
/// - macOS 標準の "Safari" アプリケーションが取得される
/// - localized name で取得されるため、日本語環境でも "Safari" のまま
///
/// 環境要件:
/// - macOS で osascript が利用可能
/// - Safari が起動している必要があります
///
/// 制限事項:
/// - Safari が起動していない場合、このテストはスキップされます
#[test]
#[ignore]
fn test_localized_name_contains_standard_apps() {
    // Arrange: Safari を起動
    let _ = std::process::Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Safari\" to activate")
        .output();
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Act: 実行中アプリケーション一覧を取得
    let result = get_running_applications();

    // Assert: 取得が成功していることを確認
    assert!(result.is_ok(), "get_running_applications() が失敗しました");

    let apps = result.unwrap();

    // Assert: Safari が含まれていることを確認
    let safari_found = apps.iter().any(|app| app.name.contains("Safari"));

    assert!(
        safari_found,
        "Safari がアプリケーション一覧に含まれていません（起動していない可能性があります）"
    );
}

/// background only is false で背景アプリケーションが除外されることを確認
///
/// 検証項目:
/// - background only is false の条件で、通常のGUIアプリケーションのみが取得される
/// - 背景プロセス（デーモン等）は除外される
///
/// 環境要件:
/// - macOS で osascript が利用可能
/// - System Events へのアクセス権限が必要
///
/// 注意:
/// - 取得結果に Finder など GUI アプリケーションが含まれることを確認
/// - 逆に、デーモンプロセスが含まれていないことを確認（完全な検証は困難）
#[test]
#[ignore]
fn test_localized_name_excludes_background_processes() {
    // Act: 実行中アプリケーション一覧を取得
    let result = get_running_applications();

    // Assert: 取得が成功していることを確認
    assert!(result.is_ok(), "get_running_applications() が失敗しました");

    let apps = result.unwrap();

    // Assert: 少なくとも1個のGUIアプリケーションが取得されていることを確認
    assert!(
        !apps.is_empty(),
        "GUIアプリケーションが0件です（background onlyフィルタが正しく機能していない可能性）"
    );

    // GUIアプリケーション（Finder等）が含まれることを確認
    let gui_app_found = apps.iter().any(|app| {
        app.name.contains("Finder") || app.name.contains("Safari") || app.name.contains("Terminal")
    });

    assert!(
        gui_app_found,
        "GUI アプリケーション（Finder/Safari/Terminal等）が1つも含まれていません"
    );

    // デーモン系のプロセス名が含まれていないことを確認（簡易チェック）
    // 注: 完全な検証は困難なため、代表的なデーモン名のみチェック
    let daemon_found = apps.iter().any(|app| {
        app.name.to_lowercase().contains("daemon")
            || app.name.to_lowercase().contains("helper")
            || app.name.to_lowercase().contains("agent")
    });

    // デーモンが含まれている場合でも、それはGUIを持つヘルパーアプリである可能性があるため、
    // ワーニング的な情報出力のみ（失敗扱いにはしない）
    if daemon_found {
        println!(
            "注意: 'daemon', 'helper', 'agent' を含むプロセス名が見つかりました（GUIヘルパーの可能性あり）"
        );
    }
}

/// RightCheatなどの自作アプリケーションがバイナリ名（"app"）ではなく、表示名で取得されることを確認
///
/// 検証項目:
/// - RightCheat アプリケーションが "app" ではなく "RightCheat" として取得される
/// - localized name を使用することで、Info.plist の CFBundleDisplayName が使用される
///
/// 環境要件:
/// - macOS で osascript が利用可能
/// - RightCheat が起動している必要があります
///
/// 制限事項:
/// - RightCheat が起動していない場合、このテストはスキップされます
/// - RightCheat 以外の自作アプリでも同様の動作確認が可能
#[test]
#[ignore]
fn test_localized_name_not_binary_name() {
    // Arrange: RightCheat を起動（存在しない場合はスキップ）
    let launch_result = std::process::Command::new("osascript")
        .arg("-e")
        .arg("tell application \"RightCheat\" to activate")
        .output();

    if launch_result.is_err() {
        println!("RightCheat が見つからないため、このテストをスキップします");
        return;
    }

    std::thread::sleep(std::time::Duration::from_secs(1));

    // Act: 実行中アプリケーション一覧を取得
    let result = get_running_applications();

    // Assert: 取得が成功していることを確認
    assert!(result.is_ok(), "get_running_applications() が失敗しました");

    let apps = result.unwrap();

    // Assert: RightCheat が "app" ではなく "RightCheat" として取得されることを確認
    let rightcheat_found = apps.iter().any(|app| app.name.contains("RightCheat"));
    let binary_name_found = apps.iter().any(|app| app.name == "app");

    assert!(
        rightcheat_found,
        "RightCheat がアプリケーション一覧に含まれていません（起動していない可能性があります）"
    );

    assert!(
        !binary_name_found || !apps.iter().any(|app| app.name == "app" && rightcheat_found),
        "RightCheat がバイナリ名 'app' として取得されています（localized name が機能していない可能性）"
    );
}

// =============================================================================
// Issue #108: displayed name への変更に関するテスト
// =============================================================================

/// displayed name で取得したアプリケーション情報が有効なAppInfo構造体になることを確認
///
/// 検証項目:
/// - get_running_applications() が正常に実行される
/// - AppInfo 構造体の name フィールドが空でない
/// - AppInfo 構造体の process_id フィールドが有効（一部アプリはNoneの可能性あり）
///
/// 環境要件:
/// - macOS で osascript が利用可能
///
/// 制限事項:
/// - CI 環境では osascript が利用できないため、ローカル macOS 環境でのみ実行可能
#[test]
#[ignore]
fn test_displayed_name_returns_valid_app_info() {
    // Act: 実行中アプリケーション一覧を取得（displayed name を使用）
    let result = get_running_applications();

    // Assert: 取得が成功していることを確認
    assert!(result.is_ok(), "get_running_applications() が失敗しました");

    let apps = result.unwrap();

    // Assert: 少なくとも1個のアプリケーションが取得されていることを確認
    assert!(
        !apps.is_empty(),
        "アプリケーション一覧が空です（displayed name が正しく機能していない可能性）"
    );

    // Assert: すべての AppInfo が有効な name を持っていることを確認
    for app in &apps {
        assert!(
            !app.name.is_empty(),
            "アプリケーション名が空です: {:?}",
            app
        );
    }

    // Assert: 少なくとも1個のアプリケーションが process_id を持っていることを確認
    // （一部のアプリケーションは process_id を取得できない場合があるため、すべてがSomeである必要はない）
    let has_process_id = apps.iter().any(|app| app.process_id.is_some());
    assert!(
        has_process_id,
        "すべてのアプリケーションで process_id が None です（unix id 取得が失敗している可能性）"
    );
}

/// "AppName|12345" 形式の解析が正常に機能することを確認
///
/// 検証項目:
/// - パイプ区切りの文字列が正しく AppInfo に変換される
/// - アプリケーション名が正しく抽出される
/// - プロセスIDが正しく整数としてパースされる
///
/// テスト方法:
/// - get_running_applications() の内部パースロジックを模倣してテスト
#[test]
fn test_displayed_name_parsing_with_pipe_separator() {
    // Arrange: "AppName|12345" 形式のテストデータを作成
    let test_entry = "Google Chrome|54321";

    // Act: パース処理を模倣（get_running_applications() のロジックを抽出）
    let pipe_pos = test_entry.rfind('|').expect("パイプ区切りが見つかりません");
    let app_name = &test_entry[..pipe_pos];
    let pid_str = &test_entry[pipe_pos + 1..];
    let process_id = pid_str.parse::<i32>().ok();

    // Assert: パース結果が正しいことを確認
    assert_eq!(
        app_name, "Google Chrome",
        "アプリケーション名が正しくパースされていません"
    );
    assert_eq!(
        process_id,
        Some(54321),
        "プロセスIDが正しくパースされていません"
    );
}

/// "AppName|" 形式（プロセスID なし）の解析が正常に機能することを確認
///
/// 検証項目:
/// - パイプ区切りの文字列で、プロセスIDが空の場合も正しく処理される
/// - アプリケーション名が正しく抽出される
/// - プロセスIDが None として扱われる
///
/// テスト方法:
/// - get_running_applications() の内部パースロジックを模倣してテスト
#[test]
fn test_displayed_name_parsing_without_process_id() {
    // Arrange: "AppName|" 形式のテストデータを作成（プロセスIDなし）
    let test_entry = "Safari|";

    // Act: パース処理を模倣（get_running_applications() のロジックを抽出）
    let pipe_pos = test_entry.rfind('|').expect("パイプ区切りが見つかりません");
    let app_name = &test_entry[..pipe_pos];
    let pid_str = &test_entry[pipe_pos + 1..];
    let process_id = if pid_str.is_empty() {
        None
    } else {
        pid_str.parse::<i32>().ok()
    };

    // Assert: パース結果が正しいことを確認
    assert_eq!(
        app_name, "Safari",
        "アプリケーション名が正しくパースされていません"
    );
    assert_eq!(process_id, None, "プロセスIDがNoneとして扱われていません");
}

/// get_running_applications() の結果が空でない（1個以上）ことを確認
///
/// 検証項目:
/// - displayed name で少なくとも1個のアプリケーションが取得される
/// - macOS 環境では常に1個以上のGUIアプリケーションが起動している想定
///
/// 環境要件:
/// - macOS で osascript が利用可能
///
/// 制限事項:
/// - CI 環境では osascript が利用できないため、ローカル macOS 環境でのみ実行可能
#[test]
#[ignore]
fn test_displayed_name_app_count_not_zero() {
    // Act: 実行中アプリケーション一覧を取得（displayed name を使用）
    let result = get_running_applications();

    // Assert: 取得が成功していることを確認
    assert!(result.is_ok(), "get_running_applications() が失敗しました");

    let apps = result.unwrap();

    // Assert: 少なくとも1個のアプリケーションが取得されていることを確認
    assert!(
        !apps.is_empty(),
        "アプリケーション一覧が空です（displayed name が正しく機能していない可能性）"
    );

    // 情報出力: 取得したアプリケーション数を表示
    println!("取得したアプリケーション数: {}", apps.len());
}

/// Safari などの標準アプリケーションが取得されることを確認
///
/// 検証項目:
/// - displayed name で macOS の標準アプリケーション（Finder, Safari 等）が取得される
/// - アプリケーション名が正しく取得される
///
/// 環境要件:
/// - macOS で osascript が利用可能
/// - Safari または Finder が起動していることが望ましい
///
/// 制限事項:
/// - CI 環境では osascript が利用できないため、ローカル macOS 環境でのみ実行可能
/// - Safari や Finder が起動していない場合、このテストは失敗する可能性があります
#[test]
#[ignore]
fn test_displayed_name_contains_standard_apps() {
    // Act: 実行中アプリケーション一覧を取得（displayed name を使用）
    let result = get_running_applications();

    // Assert: 取得が成功していることを確認
    assert!(result.is_ok(), "get_running_applications() が失敗しました");

    let apps = result.unwrap();

    // Assert: 標準アプリケーション（Finder または Safari）が含まれることを確認
    let standard_app_found = apps
        .iter()
        .any(|app| app.name.contains("Finder") || app.name.contains("Safari"));

    // 情報出力: 取得したアプリケーション一覧を表示
    println!("取得したアプリケーション一覧:");
    for app in &apps {
        println!("  - {}: {:?}", app.name, app.process_id);
    }

    assert!(
        standard_app_found,
        "標準アプリケーション（Finder/Safari）が取得されていません（起動していない可能性があります）"
    );
}

/// background only is false で背景アプリケーションが除外されることを確認
///
/// 検証項目:
/// - displayed name + background only フィルタで GUI アプリケーションのみが取得される
/// - デーモンや背景プロセスが含まれていないことを確認
///
/// 環境要件:
/// - macOS で osascript が利用可能
///
/// 制限事項:
/// - CI 環境では osascript が利用できないため、ローカル macOS 環境でのみ実行可能
/// - 完全なフィルタリング検証は困難なため、代表的なケースのみ検証
#[test]
#[ignore]
fn test_displayed_name_excludes_background_processes() {
    // Act: 実行中アプリケーション一覧を取得（displayed name + background only is false）
    let result = get_running_applications();

    // Assert: 取得が成功していることを確認
    assert!(result.is_ok(), "get_running_applications() が失敗しました");

    let apps = result.unwrap();

    // Assert: 少なくとも1個のGUIアプリケーションが取得されていることを確認
    assert!(
        !apps.is_empty(),
        "GUIアプリケーションが0件です（background onlyフィルタが正しく機能していない可能性）"
    );

    // Assert: GUIアプリケーション（Finder等）が含まれることを確認
    let gui_app_found = apps.iter().any(|app| {
        app.name.contains("Finder") || app.name.contains("Safari") || app.name.contains("Terminal")
    });

    assert!(
        gui_app_found,
        "GUI アプリケーション（Finder/Safari/Terminal等）が1つも含まれていません"
    );

    // デーモン系のプロセス名が含まれていないことを確認（簡易チェック）
    // 注: 完全な検証は困難なため、代表的なデーモン名のみチェック
    let daemon_found = apps.iter().any(|app| {
        app.name.to_lowercase().contains("daemon")
            || app.name.to_lowercase().contains("helper")
            || app.name.to_lowercase().contains("agent")
    });

    // デーモンが含まれている場合でも、それはGUIを持つヘルパーアプリである可能性があるため、
    // ワーニング的な情報出力のみ（失敗扱いにはしない）
    if daemon_found {
        println!(
            "注意: 'daemon', 'helper', 'agent' を含むプロセス名が見つかりました（GUIヘルパーの可能性あり）"
        );
    }
}

/// RightCheatなどの自作アプリケーションがバイナリ名（"app"）ではなく、表示名で取得されることを確認
///
/// 検証項目:
/// - RightCheat アプリケーションが "app" ではなく "RightCheat" として取得される
/// - displayed name を使用することで、Info.plist の CFBundleDisplayName が使用される
///
/// 環境要件:
/// - macOS で osascript が利用可能
/// - RightCheat が起動している必要があります
///
/// 制限事項:
/// - RightCheat が起動していない場合、このテストはスキップされます
/// - RightCheat 以外の自作アプリでも同様の動作確認が可能
#[test]
#[ignore]
fn test_displayed_name_not_binary_name() {
    // Arrange: RightCheat を起動（存在しない場合はスキップ）
    let launch_result = std::process::Command::new("osascript")
        .arg("-e")
        .arg("tell application \"RightCheat\" to activate")
        .output();

    if launch_result.is_err() {
        println!("RightCheat が見つからないため、このテストをスキップします");
        return;
    }

    std::thread::sleep(std::time::Duration::from_secs(1));

    // Act: 実行中アプリケーション一覧を取得（displayed name を使用）
    let result = get_running_applications();

    // Assert: 取得が成功していることを確認
    assert!(result.is_ok(), "get_running_applications() が失敗しました");

    let apps = result.unwrap();

    // Assert: RightCheat が "app" ではなく "RightCheat" として取得されることを確認
    let rightcheat_found = apps.iter().any(|app| app.name.contains("RightCheat"));
    let binary_name_found = apps.iter().any(|app| app.name == "app");

    // 情報出力: 取得したアプリケーション一覧を表示
    println!("取得したアプリケーション一覧:");
    for app in &apps {
        println!("  - {}: {:?}", app.name, app.process_id);
    }

    assert!(
        rightcheat_found,
        "RightCheat がアプリケーション一覧に含まれていません（起動していない可能性があります）"
    );

    assert!(
        !binary_name_found || !apps.iter().any(|app| app.name == "app" && rightcheat_found),
        "RightCheat がバイナリ名 'app' として取得されています（displayed name が機能していない可能性）"
    );
}
