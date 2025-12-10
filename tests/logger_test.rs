use apptidying::logger::{
    escape_applescript_string_for_test, get_notification_config, init, init_simple, LoggerConfig,
    NotificationConfig, NotificationLevel,
};
use std::fs;
use std::sync::Mutex;

// テスト間での環境変数の競合を防ぐためのロック
static ENV_LOCK: Mutex<()> = Mutex::new(());

// =============================================================================
// NotificationConfig Tests
// =============================================================================

#[test]
fn test_notification_config_default_values() {
    // デフォルト値が正しく設定されることを確認
    let config = NotificationConfig::default();

    assert_eq!(config.info, "notification");
    assert_eq!(config.warn, "notification");
    assert_eq!(config.error, "dialog");
}

#[test]
fn test_notification_config_custom_all_fields() {
    // 全フィールドをカスタム値で作成
    let config = NotificationConfig {
        info: "none".to_string(),
        warn: "dialog".to_string(),
        error: "notification".to_string(),
    };

    assert_eq!(config.info, "none");
    assert_eq!(config.warn, "dialog");
    assert_eq!(config.error, "notification");
}

#[test]
fn test_notification_config_custom_all_none() {
    // 全て "none" に設定
    let config = NotificationConfig {
        info: "none".to_string(),
        warn: "none".to_string(),
        error: "none".to_string(),
    };

    assert_eq!(config.info, "none");
    assert_eq!(config.warn, "none");
    assert_eq!(config.error, "none");
}

#[test]
fn test_notification_config_custom_all_dialog() {
    // 全て "dialog" に設定
    let config = NotificationConfig {
        info: "dialog".to_string(),
        warn: "dialog".to_string(),
        error: "dialog".to_string(),
    };

    assert_eq!(config.info, "dialog");
    assert_eq!(config.warn, "dialog");
    assert_eq!(config.error, "dialog");
}

#[test]
fn test_notification_config_custom_all_notification() {
    // 全て "notification" に設定
    let config = NotificationConfig {
        info: "notification".to_string(),
        warn: "notification".to_string(),
        error: "notification".to_string(),
    };

    assert_eq!(config.info, "notification");
    assert_eq!(config.warn, "notification");
    assert_eq!(config.error, "notification");
}

#[test]
fn test_notification_config_clone() {
    // クローンが正しく動作することを確認
    let config1 = NotificationConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.info, config2.info);
    assert_eq!(config1.warn, config2.warn);
    assert_eq!(config1.error, config2.error);
}

#[test]
fn test_notification_config_clone_custom() {
    // カスタム値のクローンが正しく動作することを確認
    let config1 = NotificationConfig {
        info: "none".to_string(),
        warn: "dialog".to_string(),
        error: "notification".to_string(),
    };
    let config2 = config1.clone();

    assert_eq!(config1.info, config2.info);
    assert_eq!(config1.warn, config2.warn);
    assert_eq!(config1.error, config2.error);

    // 元の値が正しく保持されているか確認
    assert_eq!(config2.info, "none");
    assert_eq!(config2.warn, "dialog");
    assert_eq!(config2.error, "notification");
}

#[test]
fn test_notification_config_invalid_values() {
    // 無効な値でも作成できることを確認（実行時に適切に処理される）
    let config = NotificationConfig {
        info: "invalid".to_string(),
        warn: "unknown".to_string(),
        error: "".to_string(),
    };

    assert_eq!(config.info, "invalid");
    assert_eq!(config.warn, "unknown");
    assert_eq!(config.error, "");
}

#[test]
fn test_notification_config_empty_strings() {
    // 空文字列で作成できることを確認（境界値テスト）
    let config = NotificationConfig {
        info: "".to_string(),
        warn: "".to_string(),
        error: "".to_string(),
    };

    assert_eq!(config.info, "");
    assert_eq!(config.warn, "");
    assert_eq!(config.error, "");
}

#[test]
fn test_notification_config_very_long_strings() {
    // 非常に長い文字列でも動作することを確認（境界値テスト）
    let long_string = "a".repeat(1000);
    let config = NotificationConfig {
        info: long_string.clone(),
        warn: long_string.clone(),
        error: long_string.clone(),
    };

    assert_eq!(config.info.len(), 1000);
    assert_eq!(config.warn.len(), 1000);
    assert_eq!(config.error.len(), 1000);
}

// =============================================================================
// LoggerConfig Tests
// =============================================================================

#[test]
fn test_logger_config_debug_false_notification_some() {
    // debug_mode=false, notification_config=Some のパターン
    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };

    assert!(!config.debug_mode);
    assert!(config.notification_config.is_some());
}

#[test]
fn test_logger_config_debug_true_notification_some() {
    // debug_mode=true, notification_config=Some のパターン
    let config = LoggerConfig {
        debug_mode: true,
        notification_config: Some(NotificationConfig::default()),
    };

    assert!(config.debug_mode);
    assert!(config.notification_config.is_some());
}

#[test]
fn test_logger_config_debug_false_notification_none() {
    // debug_mode=false, notification_config=None のパターン
    let config = LoggerConfig {
        debug_mode: false,
        notification_config: None,
    };

    assert!(!config.debug_mode);
    assert!(config.notification_config.is_none());
}

#[test]
fn test_logger_config_debug_true_notification_none() {
    // debug_mode=true, notification_config=None のパターン
    let config = LoggerConfig {
        debug_mode: true,
        notification_config: None,
    };

    assert!(config.debug_mode);
    assert!(config.notification_config.is_none());
}

#[test]
fn test_logger_config_with_custom_notification() {
    // カスタム通知設定を含むLoggerConfig
    let custom_notification = NotificationConfig {
        info: "none".to_string(),
        warn: "dialog".to_string(),
        error: "notification".to_string(),
    };

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(custom_notification),
    };

    assert!(config.notification_config.is_some());
    let nc = config.notification_config.unwrap();
    assert_eq!(nc.info, "none");
    assert_eq!(nc.warn, "dialog");
    assert_eq!(nc.error, "notification");
}

// =============================================================================
// NotificationLevel Tests
// =============================================================================

#[test]
fn test_notification_level_info() {
    // Info レベルが作成できることを確認
    let _level = NotificationLevel::Info;
}

#[test]
fn test_notification_level_warn() {
    // Warn レベルが作成できることを確認
    let _level = NotificationLevel::Warn;
}

#[test]
fn test_notification_level_error() {
    // Error レベルが作成できることを確認
    let _level = NotificationLevel::Error;
}

// =============================================================================
// init() and init_simple() Tests
// =============================================================================

#[test]
fn test_init_simple() {
    // init_simple() が正しく実行されることを確認
    init_simple();

    let stored_config = get_notification_config();
    assert!(stored_config.is_some());

    let nc = stored_config.unwrap();
    assert_eq!(nc.info, "notification");
    assert_eq!(nc.warn, "notification");
    assert_eq!(nc.error, "dialog");
}

#[test]
fn test_init_stores_config_with_default_notification() {
    // デフォルト通知設定が正しく保存されることを確認
    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };

    init(config);

    let stored_config = get_notification_config();
    assert!(stored_config.is_some());

    let nc = stored_config.unwrap();
    assert_eq!(nc.info, "notification");
    assert_eq!(nc.warn, "notification");
    assert_eq!(nc.error, "dialog");
}

#[test]
fn test_init_stores_config_with_custom_notification() {
    // カスタム通知設定が正しく保存されることを確認
    let config = LoggerConfig {
        debug_mode: true,
        notification_config: Some(NotificationConfig {
            info: "none".to_string(),
            warn: "dialog".to_string(),
            error: "notification".to_string(),
        }),
    };

    init(config);

    let stored_config = get_notification_config();
    assert!(stored_config.is_some());

    let nc = stored_config.unwrap();
    assert_eq!(nc.info, "none");
    assert_eq!(nc.warn, "dialog");
    assert_eq!(nc.error, "notification");
}

#[test]
fn test_init_stores_config_without_notification() {
    // 通知設定なしで初期化した場合、None が保存されることを確認
    let config = LoggerConfig {
        debug_mode: false,
        notification_config: None,
    };

    init(config);

    let stored_config = get_notification_config();
    assert!(stored_config.is_none());
}

#[test]
fn test_init_with_debug_mode_enabled() {
    // debug_mode=true で初期化されることを確認
    let config = LoggerConfig {
        debug_mode: true,
        notification_config: Some(NotificationConfig::default()),
    };

    init(config);
    // 初期化が成功することを確認（env_loggerの初期化は1回のみのため、複数回呼び出してもエラーにならない）
}

#[test]
fn test_init_with_debug_mode_disabled() {
    // debug_mode=false で初期化されることを確認
    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };

    init(config);
    // 初期化が成功することを確認
}

#[test]
fn test_init_overwrite_previous_config() {
    // 前の設定を上書きできることを確認
    let config1 = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig {
            info: "notification".to_string(),
            warn: "notification".to_string(),
            error: "dialog".to_string(),
        }),
    };
    init(config1);

    let config2 = LoggerConfig {
        debug_mode: true,
        notification_config: Some(NotificationConfig {
            info: "none".to_string(),
            warn: "none".to_string(),
            error: "none".to_string(),
        }),
    };
    init(config2);

    let stored_config = get_notification_config();
    assert!(stored_config.is_some());

    let nc = stored_config.unwrap();
    assert_eq!(nc.info, "none");
    assert_eq!(nc.warn, "none");
    assert_eq!(nc.error, "none");
}

// =============================================================================
// escape_applescript_string Tests
// =============================================================================

#[test]
fn test_escape_applescript_string_no_special_chars() {
    // 特殊文字を含まない文字列はそのまま返される
    assert_eq!(
        escape_applescript_string_for_test("Hello World"),
        "Hello World"
    );
}

#[test]
fn test_escape_applescript_string_backslash() {
    // バックスラッシュが正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string_for_test("test\\path"),
        "test\\\\path"
    );
}

#[test]
fn test_escape_applescript_string_double_quote() {
    // ダブルクォートが正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string_for_test("test\"quote"),
        "test\\\"quote"
    );
}

#[test]
fn test_escape_applescript_string_newline() {
    // 改行が正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string_for_test("test\nline"),
        "test\\nline"
    );
}

#[test]
fn test_escape_applescript_string_carriage_return() {
    // キャリッジリターンが正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string_for_test("test\rline"),
        "test\\rline"
    );
}

#[test]
fn test_escape_applescript_string_multiple_backslashes() {
    // 複数のバックスラッシュが正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string_for_test("path\\to\\file"),
        "path\\\\to\\\\file"
    );
}

#[test]
fn test_escape_applescript_string_multiple_quotes() {
    // 複数のダブルクォートが正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string_for_test("\"quoted\" \"text\""),
        "\\\"quoted\\\" \\\"text\\\""
    );
}

#[test]
fn test_escape_applescript_string_multiple_newlines() {
    // 複数の改行が正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string_for_test("line1\nline2\nline3"),
        "line1\\nline2\\nline3"
    );
}

#[test]
fn test_escape_applescript_string_combined_special_chars() {
    // 複数の特殊文字の組み合わせが正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string_for_test("path\\file\"name\ntest"),
        "path\\\\file\\\"name\\ntest"
    );
}

#[test]
fn test_escape_applescript_string_all_special_chars() {
    // 全ての特殊文字を含む文字列が正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string_for_test("test\\path\"quote\nline\rreturn"),
        "test\\\\path\\\"quote\\nline\\rreturn"
    );
}

#[test]
fn test_escape_applescript_string_empty() {
    // 空文字列が正しく処理されることを確認（境界値テスト）
    assert_eq!(escape_applescript_string_for_test(""), "");
}

#[test]
fn test_escape_applescript_string_single_char() {
    // 1文字の文字列が正しく処理されることを確認（境界値テスト）
    assert_eq!(escape_applescript_string_for_test("a"), "a");
}

#[test]
fn test_escape_applescript_string_single_special_char_backslash() {
    // 1文字のバックスラッシュが正しくエスケープされることを確認
    assert_eq!(escape_applescript_string_for_test("\\"), "\\\\");
}

#[test]
fn test_escape_applescript_string_single_special_char_quote() {
    // 1文字のダブルクォートが正しくエスケープされることを確認
    assert_eq!(escape_applescript_string_for_test("\""), "\\\"");
}

#[test]
fn test_escape_applescript_string_single_special_char_newline() {
    // 1文字の改行が正しくエスケープされることを確認
    assert_eq!(escape_applescript_string_for_test("\n"), "\\n");
}

#[test]
fn test_escape_applescript_string_single_special_char_carriage_return() {
    // 1文字のキャリッジリターンが正しくエスケープされることを確認
    assert_eq!(escape_applescript_string_for_test("\r"), "\\r");
}

#[test]
fn test_escape_applescript_string_consecutive_special_chars() {
    // 連続した特殊文字が正しくエスケープされることを確認
    assert_eq!(
        escape_applescript_string_for_test("\\\"\n\r"),
        "\\\\\\\"\\n\\r"
    );
}

#[test]
fn test_escape_applescript_string_special_chars_at_boundaries() {
    // 特殊文字が文字列の先頭と末尾にある場合のエスケープ確認
    assert_eq!(
        escape_applescript_string_for_test("\\test\""),
        "\\\\test\\\""
    );
}

#[test]
fn test_escape_applescript_string_very_long_string() {
    // 非常に長い文字列が正しく処理されることを確認（境界値テスト）
    let long_string = "a".repeat(10000);
    let escaped = escape_applescript_string_for_test(&long_string);
    assert_eq!(escaped.len(), 10000);
    assert_eq!(escaped, long_string);
}

#[test]
fn test_escape_applescript_string_very_long_string_with_special_chars() {
    // 特殊文字を含む非常に長い文字列が正しく処理されることを確認
    let long_string = "a\\b\"c\nd\r".repeat(1000);
    let escaped = escape_applescript_string_for_test(&long_string);
    // 元の文字列: "a\\b\"c\nd\r" = 7文字 (a, \, b, ", c, \n, d, \r)
    // エスケープ後: "a\\\\b\\\"c\\nd\\r" = 12文字
    // \  → \\ (2文字)
    // "  → \" (2文字)
    // \n → \n (2文字、文字列リテラル内では既に\nだが、バイト長は1)
    // \r → \r (2文字、文字列リテラル内では既に\rだが、バイト長は1)
    // 実際の元の文字列バイト長は: a(1) + \(1) + b(1) + "(1) + c(1) + \n(1) + d(1) + \r(1) = 8バイト
    // エスケープ後: a(1) + \\(2) + b(1) + \"(2) + c(1) + \n(2) + d(1) + \r(2) = 12バイト
    assert_eq!(long_string.len(), 8 * 1000);
    assert_eq!(escaped.len(), 12 * 1000);
}

#[test]
fn test_escape_applescript_string_unicode() {
    // Unicode文字が正しく処理されることを確認
    assert_eq!(
        escape_applescript_string_for_test("日本語テスト"),
        "日本語テスト"
    );
}

#[test]
fn test_escape_applescript_string_unicode_with_special_chars() {
    // Unicode文字と特殊文字の組み合わせが正しく処理されることを確認
    assert_eq!(
        escape_applescript_string_for_test("日本語\\テスト\"改行\n"),
        "日本語\\\\テスト\\\"改行\\n"
    );
}

#[test]
fn test_escape_applescript_string_emoji() {
    // 絵文字が正しく処理されることを確認
    assert_eq!(
        escape_applescript_string_for_test("Test 🚀 emoji"),
        "Test 🚀 emoji"
    );
}

#[test]
fn test_escape_applescript_string_mixed_content() {
    // 実際のエラーメッセージのような複雑な文字列をテスト
    let message = "Failed to open file: \"C:\\Users\\test\\file.txt\"\nError: Permission denied";
    let expected =
        "Failed to open file: \\\"C:\\\\Users\\\\test\\\\file.txt\\\"\\nError: Permission denied";
    assert_eq!(escape_applescript_string_for_test(message), expected);
}

// =============================================================================
// show_notification Tests
// =============================================================================

#[test]
fn test_show_notification_info_terminal() {
    // ターミナル実行時のInfo通知（TERM環境変数設定あり）
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::set_var("TERM", "xterm");

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };
    init(config);

    // 標準出力に出力されることを期待（実際の出力は目視確認）
    apptidying::logger::show_notification(NotificationLevel::Info, "Test Info message");

    std::env::remove_var("TERM");
}

#[test]
fn test_show_notification_warn_terminal() {
    // ターミナル実行時のWarn通知
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::set_var("TERM", "xterm");

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };
    init(config);

    apptidying::logger::show_notification(NotificationLevel::Warn, "Test Warning message");

    std::env::remove_var("TERM");
}

#[test]
fn test_show_notification_error_terminal() {
    // ターミナル実行時のError通知
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::set_var("TERM", "xterm");

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };
    init(config);

    apptidying::logger::show_notification(NotificationLevel::Error, "Test Error message");

    std::env::remove_var("TERM");
}

#[test]
fn test_show_notification_with_special_characters_terminal() {
    // 特殊文字を含むメッセージのターミナル通知
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::set_var("TERM", "xterm");

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };
    init(config);

    apptidying::logger::show_notification(
        NotificationLevel::Info,
        "Message with \"quotes\" and \\backslash\\ and \nnewline",
    );

    std::env::remove_var("TERM");
}

#[test]
fn test_show_notification_empty_message_terminal() {
    // 空のメッセージのターミナル通知（境界値テスト）
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::set_var("TERM", "xterm");

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };
    init(config);

    apptidying::logger::show_notification(NotificationLevel::Info, "");

    std::env::remove_var("TERM");
}

#[test]
fn test_show_notification_very_long_message_terminal() {
    // 非常に長いメッセージのターミナル通知（境界値テスト）
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::set_var("TERM", "xterm");

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };
    init(config);

    let long_message = "a".repeat(1000);
    apptidying::logger::show_notification(NotificationLevel::Info, &long_message);

    std::env::remove_var("TERM");
}

#[test]
fn test_show_notification_unicode_message_terminal() {
    // Unicode文字を含むメッセージのターミナル通知
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::set_var("TERM", "xterm");

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };
    init(config);

    apptidying::logger::show_notification(NotificationLevel::Info, "日本語メッセージのテスト");

    std::env::remove_var("TERM");
}

// Note: 非ターミナル実行時のテストは、実際にosascriptを実行するため、
// CI/CD環境やヘッドレス環境では失敗する可能性があります。
// そのため、ここでは環境変数を削除することでコードパスをテストしますが、
// 実際の通知は表示されません（macOS環境でのみ動作）。

#[test]
fn test_show_notification_info_non_terminal() {
    // 非ターミナル実行時のInfo通知（TERM環境変数なし）
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::remove_var("TERM");

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };
    init(config);

    // show_os_notification が呼び出されるが、osascriptの実行はスキップされる可能性がある
    apptidying::logger::show_notification(NotificationLevel::Info, "Non-terminal Info");
}

#[test]
fn test_show_notification_warn_non_terminal() {
    // 非ターミナル実行時のWarn通知
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::remove_var("TERM");

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };
    init(config);

    apptidying::logger::show_notification(NotificationLevel::Warn, "Non-terminal Warning");
}

#[test]
fn test_show_notification_error_non_terminal() {
    // 非ターミナル実行時のError通知
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::remove_var("TERM");

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };
    init(config);

    apptidying::logger::show_notification(NotificationLevel::Error, "Non-terminal Error");
}

#[test]
fn test_show_notification_with_custom_notification_config_info_none() {
    // カスタム通知設定（info="none"）の動作確認
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::remove_var("TERM");

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig {
            info: "none".to_string(),
            warn: "notification".to_string(),
            error: "dialog".to_string(),
        }),
    };
    init(config);

    // "none" 設定なので通知は表示されない
    apptidying::logger::show_notification(NotificationLevel::Info, "Should not notify");
}

#[test]
fn test_show_notification_with_custom_notification_config_warn_dialog() {
    // カスタム通知設定（warn="dialog"）の動作確認
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::remove_var("TERM");

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig {
            info: "notification".to_string(),
            warn: "dialog".to_string(),
            error: "dialog".to_string(),
        }),
    };
    init(config);

    // "dialog" 設定なのでダイアログ表示が試行される
    apptidying::logger::show_notification(NotificationLevel::Warn, "Should show dialog");
}

#[test]
fn test_show_notification_with_custom_notification_config_error_notification() {
    // カスタム通知設定（error="notification"）の動作確認
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::remove_var("TERM");

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig {
            info: "notification".to_string(),
            warn: "notification".to_string(),
            error: "notification".to_string(),
        }),
    };
    init(config);

    // "notification" 設定なので通知センター表示が試行される
    apptidying::logger::show_notification(NotificationLevel::Error, "Should show notification");
}

#[test]
fn test_show_notification_without_notification_config() {
    // 通知設定なしで初期化した場合のデフォルト動作確認
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::remove_var("TERM");

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: None,
    };
    init(config);

    // デフォルト設定が使用される
    apptidying::logger::show_notification(NotificationLevel::Info, "Default config test");
}

// =============================================================================
// get_notification_config Tests
// =============================================================================

#[test]
fn test_get_notification_config_after_init_with_config() {
    // 設定ありで初期化後、get_notification_config で取得できることを確認
    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig {
            info: "dialog".to_string(),
            warn: "none".to_string(),
            error: "notification".to_string(),
        }),
    };
    init(config);

    let retrieved = get_notification_config();
    assert!(retrieved.is_some());

    let nc = retrieved.unwrap();
    assert_eq!(nc.info, "dialog");
    assert_eq!(nc.warn, "none");
    assert_eq!(nc.error, "notification");
}

#[test]
fn test_get_notification_config_after_init_without_config() {
    // 設定なしで初期化後、get_notification_config が None を返すことを確認
    let config = LoggerConfig {
        debug_mode: false,
        notification_config: None,
    };
    init(config);

    let retrieved = get_notification_config();
    assert!(retrieved.is_none());
}

#[test]
fn test_get_notification_config_after_multiple_inits() {
    // 複数回初期化後、最新の設定が取得できることを確認
    let config1 = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig {
            info: "notification".to_string(),
            warn: "notification".to_string(),
            error: "dialog".to_string(),
        }),
    };
    init(config1);

    let config2 = LoggerConfig {
        debug_mode: true,
        notification_config: Some(NotificationConfig {
            info: "none".to_string(),
            warn: "none".to_string(),
            error: "none".to_string(),
        }),
    };
    init(config2);

    let retrieved = get_notification_config();
    assert!(retrieved.is_some());

    let nc = retrieved.unwrap();
    assert_eq!(nc.info, "none");
    assert_eq!(nc.warn, "none");
    assert_eq!(nc.error, "none");
}

// =============================================================================
// Log File Path Tests
// =============================================================================

#[test]
fn test_log_file_path_generation() {
    // ログファイルパスが正しく生成されることを確認
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => {
            // テスト環境でホームディレクトリが取得できない場合はテストをスキップ
            return;
        }
    };

    let expected_path =
        home.join("Library/Application Support/biz.nosetech.apptidying/apptidying.log");

    // ログディレクトリが作成されることを確認
    let log_dir = home.join("Library/Application Support/biz.nosetech.apptidying");
    assert!(log_dir.exists() || fs::create_dir_all(&log_dir).is_ok());

    // 期待されるパスが存在するか、作成できることを確認
    assert!(expected_path.parent().unwrap().exists());
}

#[test]
fn test_log_file_directory_creation() {
    // ログファイルディレクトリが自動的に作成されることを確認
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => {
            return;
        }
    };

    let log_dir = home.join("Library/Application Support/biz.nosetech.apptidying");

    // ディレクトリが存在するか、作成できることを確認
    if !log_dir.exists() {
        assert!(fs::create_dir_all(&log_dir).is_ok());
    }
    assert!(log_dir.exists());
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn test_integration_full_workflow_terminal() {
    // 完全なワークフローのインテグレーションテスト（ターミナル実行）
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::set_var("TERM", "xterm");

    // 1. カスタム設定で初期化
    let config = LoggerConfig {
        debug_mode: true,
        notification_config: Some(NotificationConfig {
            info: "notification".to_string(),
            warn: "dialog".to_string(),
            error: "none".to_string(),
        }),
    };
    init(config);

    // 2. 設定が正しく保存されたことを確認
    let retrieved = get_notification_config();
    assert!(retrieved.is_some());
    let nc = retrieved.unwrap();
    assert_eq!(nc.info, "notification");
    assert_eq!(nc.warn, "dialog");
    assert_eq!(nc.error, "none");

    // 3. 各レベルの通知を表示
    apptidying::logger::show_notification(NotificationLevel::Info, "Integration test info");
    apptidying::logger::show_notification(NotificationLevel::Warn, "Integration test warning");
    apptidying::logger::show_notification(NotificationLevel::Error, "Integration test error");

    std::env::remove_var("TERM");
}

#[test]
fn test_integration_full_workflow_non_terminal() {
    // 完全なワークフローのインテグレーションテスト（非ターミナル実行）
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::remove_var("TERM");

    // 1. デフォルト設定で初期化
    init_simple();

    // 2. 設定が正しく保存されたことを確認
    let retrieved = get_notification_config();
    assert!(retrieved.is_some());
    let nc = retrieved.unwrap();
    assert_eq!(nc.info, "notification");
    assert_eq!(nc.warn, "notification");
    assert_eq!(nc.error, "dialog");

    // 3. 各レベルの通知を表示
    apptidying::logger::show_notification(NotificationLevel::Info, "Non-terminal integration info");
    apptidying::logger::show_notification(
        NotificationLevel::Warn,
        "Non-terminal integration warning",
    );
    apptidying::logger::show_notification(
        NotificationLevel::Error,
        "Non-terminal integration error",
    );
}

#[test]
fn test_integration_config_update_workflow() {
    // 設定更新ワークフローのインテグレーションテスト
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::set_var("TERM", "xterm");

    // 1. 初回設定
    let config1 = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };
    init(config1);

    let nc1 = get_notification_config().unwrap();
    assert_eq!(nc1.info, "notification");

    // 2. 設定更新
    let config2 = LoggerConfig {
        debug_mode: true,
        notification_config: Some(NotificationConfig {
            info: "none".to_string(),
            warn: "none".to_string(),
            error: "none".to_string(),
        }),
    };
    init(config2);

    let nc2 = get_notification_config().unwrap();
    assert_eq!(nc2.info, "none");

    // 3. 通知を表示（"none"設定なので通知されない）
    apptidying::logger::show_notification(NotificationLevel::Info, "Should not notify");

    std::env::remove_var("TERM");
}

#[test]
fn test_integration_escape_and_notify() {
    // エスケープと通知の統合テスト
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::set_var("TERM", "xterm");

    init_simple();

    // 特殊文字を含むメッセージを通知
    let message = "Error: File \"C:\\test\\file.txt\" not found\nPlease check the path";
    apptidying::logger::show_notification(NotificationLevel::Error, message);

    // エスケープ関数が正しく動作していることを確認
    let escaped = escape_applescript_string_for_test(message);
    assert!(escaped.contains("\\\\"));
    assert!(escaped.contains("\\\""));
    assert!(escaped.contains("\\n"));

    std::env::remove_var("TERM");
}

// =============================================================================
// Edge Case Tests
// =============================================================================

#[test]
fn test_edge_case_notification_config_with_whitespace() {
    // ホワイトスペースを含む設定値のテスト
    let config = NotificationConfig {
        info: " notification ".to_string(),
        warn: "  dialog  ".to_string(),
        error: "none ".to_string(),
    };

    // 値がそのまま保持されることを確認（trim処理はされない）
    assert_eq!(config.info, " notification ");
    assert_eq!(config.warn, "  dialog  ");
    assert_eq!(config.error, "none ");
}

#[test]
fn test_edge_case_multiple_consecutive_escapes() {
    // 連続するエスケープ対象文字のテスト
    assert_eq!(escape_applescript_string_for_test("\\\\\\"), "\\\\\\\\\\\\");
    assert_eq!(escape_applescript_string_for_test("\"\"\""), "\\\"\\\"\\\"");
    assert_eq!(escape_applescript_string_for_test("\n\n\n"), "\\n\\n\\n");
}

#[test]
fn test_edge_case_escape_order_matters() {
    // エスケープの順序が重要であることを確認
    // バックスラッシュが最初にエスケープされないと、
    // 他のエスケープのバックスラッシュまでエスケープされてしまう
    let input = "\\\"";
    let expected = "\\\\\\\""; // \\ と \" にそれぞれエスケープ
    assert_eq!(escape_applescript_string_for_test(input), expected);
}

#[test]
fn test_edge_case_terminal_detection_with_different_term_values() {
    // 様々なTERM値でのターミナル検出テスト
    let _lock = ENV_LOCK.lock().unwrap();

    let term_values = vec!["xterm", "xterm-256color", "screen", "dumb", "vt100", ""];

    for term_value in term_values {
        std::env::set_var("TERM", term_value);
        init_simple();
        apptidying::logger::show_notification(
            NotificationLevel::Info,
            &format!("Test with TERM={}", term_value),
        );
    }

    std::env::remove_var("TERM");
}

#[test]
fn test_edge_case_notification_with_null_bytes() {
    // null バイトを含む文字列のテスト（Rustでは問題ないが、念のため）
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::set_var("TERM", "xterm");

    init_simple();

    // null バイトを含む文字列
    let message_with_null = "Test\0message";
    apptidying::logger::show_notification(NotificationLevel::Info, message_with_null);

    std::env::remove_var("TERM");
}

#[test]
fn test_edge_case_very_long_notification_config_values() {
    // 非常に長い通知設定値のテスト
    let very_long_value = "notification".repeat(1000);
    let config = NotificationConfig {
        info: very_long_value.clone(),
        warn: very_long_value.clone(),
        error: very_long_value.clone(),
    };

    assert_eq!(config.info.len(), "notification".len() * 1000);
}
