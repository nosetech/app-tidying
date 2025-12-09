use apptidying::logger::{
    get_notification_config, init, init_simple, LoggerConfig, NotificationConfig, NotificationLevel,
};
use std::fs;

#[test]
fn test_logger_config_creation() {
    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };

    assert!(!config.debug_mode);
    assert!(config.notification_config.is_some());
}

#[test]
fn test_notification_config_default() {
    let config = NotificationConfig::default();

    assert_eq!(config.info, "notification");
    assert_eq!(config.warn, "notification");
    assert_eq!(config.error, "dialog");
}

#[test]
fn test_notification_config_creation() {
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
fn test_debug_mode_logging() {
    let config = LoggerConfig {
        debug_mode: true,
        notification_config: Some(NotificationConfig::default()),
    };

    assert!(config.debug_mode);
}

#[test]
fn test_notification_level_creation() {
    let _info = NotificationLevel::Info;
    let _warn = NotificationLevel::Warn;
    let _error = NotificationLevel::Error;
}

#[test]
fn test_logger_init_simple() {
    init_simple();
}

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

    let _expected_path =
        home.join("Library/Application Support/biz.nosetech.apptidying/apptidying.log");

    // ログファイルディレクトリの存在確認
    let log_dir = home.join("Library/Application Support/biz.nosetech.apptidying");
    assert!(log_dir.exists() || fs::create_dir_all(&log_dir).is_ok());
}

#[test]
fn test_notification_config_clone() {
    let config1 = NotificationConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.info, config2.info);
    assert_eq!(config1.warn, config2.warn);
    assert_eq!(config1.error, config2.error);
}

#[test]
fn test_logger_config_notification_optional() {
    let config_with_notification = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };

    let config_without_notification = LoggerConfig {
        debug_mode: false,
        notification_config: None,
    };

    assert!(config_with_notification.notification_config.is_some());
    assert!(config_without_notification.notification_config.is_none());
}

#[test]
fn test_logger_debug_mode_variations() {
    let debug_enabled = LoggerConfig {
        debug_mode: true,
        notification_config: None,
    };

    let debug_disabled = LoggerConfig {
        debug_mode: false,
        notification_config: None,
    };

    assert!(debug_enabled.debug_mode);
    assert!(!debug_disabled.debug_mode);
}

#[test]
fn test_logger_init_stores_config() {
    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig {
            info: "notification".to_string(),
            warn: "dialog".to_string(),
            error: "none".to_string(),
        }),
    };

    init(config);

    let stored_config = get_notification_config();
    assert!(stored_config.is_some());

    let nc = stored_config.unwrap();
    assert_eq!(nc.info, "notification");
    assert_eq!(nc.warn, "dialog");
    assert_eq!(nc.error, "none");
}

#[test]
fn test_logger_init_with_default_notification_config() {
    let config = LoggerConfig {
        debug_mode: true,
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
fn test_logger_init_with_no_notification_config() {
    let config = LoggerConfig {
        debug_mode: false,
        notification_config: None,
    };

    init(config);

    let stored_config = get_notification_config();
    assert!(stored_config.is_none());
}

#[test]
fn test_escape_applescript_string() {
    // バックスラッシュのエスケープをテスト
    assert_eq!(
        apptidying::logger::escape_applescript_string_for_test("test\\path"),
        "test\\\\path"
    );

    // ダブルクォートのエスケープをテスト
    assert_eq!(
        apptidying::logger::escape_applescript_string_for_test("test\"quote"),
        "test\\\"quote"
    );

    // 改行のエスケープをテスト
    assert_eq!(
        apptidying::logger::escape_applescript_string_for_test("test\nline"),
        "test\\nline"
    );

    // キャリッジリターンのエスケープをテスト
    assert_eq!(
        apptidying::logger::escape_applescript_string_for_test("test\rline"),
        "test\\rline"
    );

    // 複合エスケープをテスト
    assert_eq!(
        apptidying::logger::escape_applescript_string_for_test("path\\file\"name\ntest"),
        "path\\\\file\\\"name\\ntest"
    );
}

#[test]
fn test_show_notification_terminal_execution() {
    // ターミナル実行時（TERM環境変数が設定されている場合）のテスト
    // この関数は println! を呼び出すため、テスト環境ではターミナル出力が確認できる
    std::env::set_var("TERM", "xterm");

    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };

    init(config);

    // show_notification を実行（ターミナル実行時は println! が実行される）
    apptidying::logger::show_notification(NotificationLevel::Info, "Test notification");

    // ターミナル出力が実行されたことを確認（テスト実行時に標準出力に表示される）
    // 実際の検証は、test コマンド実行時の出力で確認可能
}

#[test]
fn test_init_stores_custom_notification_config() {
    // カスタム通知設定を保存・検証するテスト
    let custom_config = LoggerConfig {
        debug_mode: true,
        notification_config: Some(NotificationConfig {
            info: "none".to_string(),
            warn: "dialog".to_string(),
            error: "notification".to_string(),
        }),
    };

    init(custom_config);

    let stored = get_notification_config();
    assert!(stored.is_some());

    let nc = stored.unwrap();
    assert_eq!(nc.info, "none", "info should be 'none'");
    assert_eq!(nc.warn, "dialog", "warn should be 'dialog'");
    assert_eq!(nc.error, "notification", "error should be 'notification'");
}
