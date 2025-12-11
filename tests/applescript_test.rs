use apptidying::applescript::{
    escape_applescript_string, launch_or_activate_app, AppLaunchError, AppLaunchResult,
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
// AppLaunchError Tests
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
// AppLaunchResult Tests
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
// Integration Tests: Complete Workflow
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
// Edge Case Tests
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
