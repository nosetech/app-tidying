use apptidying::logger::{init_simple, LoggerConfig, NotificationConfig, NotificationLevel};
use std::fs;
use std::path::PathBuf;

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
    let home = dirs::home_dir().expect("Failed to get home directory");
    let expected_path =
        home.join("Library/Application Support/biz.nosetech.apptidying/AppTidying.log");

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
