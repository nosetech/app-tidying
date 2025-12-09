use log::LevelFilter;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[allow(dead_code)]
pub enum NotificationLevel {
    Info,
    Warn,
    Error,
}

pub struct LoggerConfig {
    pub debug_mode: bool,
    #[allow(dead_code)]
    pub notification_config: Option<NotificationConfig>,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct NotificationConfig {
    pub info: String,
    pub warn: String,
    pub error: String,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        NotificationConfig {
            info: "notification".to_string(),
            warn: "notification".to_string(),
            error: "dialog".to_string(),
        }
    }
}

fn get_log_file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("Failed to get home directory")?;
    let log_dir = home.join("Library/Application Support/biz.nosetech.apptidying");
    fs::create_dir_all(&log_dir)?;
    Ok(log_dir.join("apptidying.log"))
}

#[allow(dead_code)]
fn is_running_in_terminal() -> bool {
    std::env::var("TERM").is_ok()
}

fn append_to_log_file(message: &str) -> std::io::Result<()> {
    if let Ok(path) = get_log_file_path() {
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(file, "{}", message)?;
    }
    Ok(())
}

#[allow(dead_code)]
fn escape_applescript_string(s: &str) -> String {
    s.replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
}

pub fn init(config: LoggerConfig) {
    let filter_level = if config.debug_mode {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::Builder::from_default_env()
        .filter_level(filter_level)
        .format(|buf, record| {
            use chrono::Local;
            let log_message = format!(
                "[{}] [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            );

            // ログファイルに書き込む
            let _ = append_to_log_file(&log_message);

            writeln!(buf, "{}", log_message)
        })
        .try_init()
        .ok();
}

#[allow(dead_code)]
pub fn init_simple() {
    let config = LoggerConfig {
        debug_mode: false,
        notification_config: Some(NotificationConfig::default()),
    };
    init(config);
}

#[allow(dead_code)]
pub fn show_notification(level: NotificationLevel, message: &str) {
    let notification_type = match level {
        NotificationLevel::Info => "INFO",
        NotificationLevel::Warn => "WARN",
        NotificationLevel::Error => "ERROR",
    };

    let output_message = format!("[{}] {}", notification_type, message);

    // ログファイルに記録
    let _ = append_to_log_file(&output_message);

    if is_running_in_terminal() {
        // ターミナル実行時は標準出力のみ
        println!("{}", output_message);
    } else {
        // ターミナル外実行時は通知を表示
        show_os_notification(level, message);
    }
}

#[allow(dead_code)]
fn show_os_notification(level: NotificationLevel, message: &str) {
    match level {
        NotificationLevel::Info | NotificationLevel::Warn => {
            // macOS通知センターに通知を表示
            let script = format!(
                r#"display notification "{}" with title "App Tidying""#,
                escape_applescript_string(message)
            );
            match Command::new("osascript").arg("-e").arg(&script).output() {
                Ok(output) if !output.status.success() => {
                    log::warn!(
                        "Failed to show notification: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
                Err(e) => {
                    log::warn!("Failed to execute osascript: {}", e);
                }
                _ => {}
            }
        }
        NotificationLevel::Error => {
            // ダイアログを表示
            let script = format!(
                r#"display dialog "{}" buttons {{"OK"}} default button "OK""#,
                escape_applescript_string(message)
            );
            match Command::new("osascript").arg("-e").arg(&script).output() {
                Ok(output) if !output.status.success() => {
                    log::warn!(
                        "Failed to show dialog: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
                Err(e) => {
                    log::warn!("Failed to execute osascript: {}", e);
                }
                _ => {}
            }
        }
    }
}
